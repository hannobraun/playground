use std::{fs::File, io::Read, panic, path::Path, thread};

use crossbeam_channel::{
    Receiver, RecvError, SendError, Sender, bounded, select, unbounded,
};
use notify::{RecursiveMode, Watcher};
use stack_assembly::{Effect, Eval};

mod io;

const GRID_SIZE: usize = 32;
const PIXELS_SIZE: usize = GRID_SIZE * GRID_SIZE;
const BYTES_PER_PIXEL: usize = 4;
const PIXELS_SIZE_BYTES: usize = PIXELS_SIZE * BYTES_PER_PIXEL;

fn main() -> anyhow::Result<()> {
    let (lifeline_tx, lifeline_rx) = bounded(0);
    let (pixels_tx, pixels_rx) = bounded(0);

    let handle = thread::spawn(|| run_script(lifeline_rx, pixels_tx));
    io::start_and_wait(lifeline_tx, pixels_rx)?;

    match handle.join() {
        Ok(result) => result?,
        Err(err) => {
            panic::resume_unwind(err);
        }
    }

    Ok(())
}

fn run_script(
    lifeline_rx: Receiver<()>,
    pixels_tx: Sender<[u8; PIXELS_SIZE_BYTES]>,
) -> anyhow::Result<()> {
    let path = Path::new("snake.stack");

    let (notify_tx, notify_rx) = unbounded();

    let mut watcher = notify::recommended_watcher(notify_tx)?;
    watcher.watch(path, RecursiveMode::NonRecursive)?;

    let mut run = 0;
    let mut eval = load_script(path)?;

    loop {
        match eval.run() {
            Effect::Yield => {
                let mut pixels = [0; PIXELS_SIZE_BYTES];
                for i in 0..PIXELS_SIZE {
                    let pixel = eval.memory.values[i].to_u32().to_be_bytes();
                    pixels[i * BYTES_PER_PIXEL
                        ..i * BYTES_PER_PIXEL + BYTES_PER_PIXEL]
                        .copy_from_slice(&pixel);
                }

                // `pixels_tx` is bounded with capacity zero, so this will block
                // until the pixels are being drawn, tying the frame rate of the
                // script to the frame rate of the I/O.
                if let Err(SendError(_)) = pixels_tx.send(pixels) {
                    // Other end has hung up, which means we need to quit too.
                    return Ok(());
                }

                eval.effect = None;
                continue;
            }
            effect => {
                eprintln!("{run}: Script triggered effect: {effect:?}");

                match wait_for_change(&mut run, &notify_rx, &lifeline_rx)? {
                    WaitForChangeOutcome::ScriptHasChanged => {
                        continue;
                    }
                    WaitForChangeOutcome::MustQuit => {
                        return Ok(());
                    }
                }
            }
        }
    }
}

fn load_script(path: &Path) -> anyhow::Result<Eval> {
    let mut script = String::new();
    File::open(path)?.read_to_string(&mut script)?;

    let eval = Eval::start(&script);

    Ok(eval)
}

fn wait_for_change(
    run: &mut u64,
    notify_rx: &Receiver<notify::Result<notify::Event>>,
    lifeline_rx: &Receiver<()>,
) -> anyhow::Result<WaitForChangeOutcome> {
    loop {
        let event = select! {
            recv(notify_rx) -> event => {
                event??
            }
            recv(lifeline_rx) -> message => {
                let Err(RecvError) = message else {
                    unreachable!(
                        "Lifeline channel only exists to get dropped."
                    );
                };

                // Channel has been dropped. We're done.
                return Ok(WaitForChangeOutcome::MustQuit);
            }
        };

        match event.kind {
            notify::EventKind::Modify(_) => {
                *run += 1;
                return Ok(WaitForChangeOutcome::ScriptHasChanged);
            }
            _ => {
                continue;
            }
        }
    }
}

enum WaitForChangeOutcome {
    ScriptHasChanged,
    MustQuit,
}
