use std::{
    fs::File,
    io::Read,
    path::Path,
    time::{Duration, Instant},
};

use crossbeam_channel::{
    Receiver, RecvError, SendError, Sender, after, bounded, select, unbounded,
};
use notify::{RecursiveMode, Watcher};
use stack_assembly::{Effect, Eval};

use crate::{BYTES_PER_PIXEL, PIXELS_SIZE_BYTES, Pixels};

mod memory {
    use crate::PIXELS_SIZE;

    pub struct Region {
        pub start: usize,
        pub size: usize,
    }

    impl Region {
        pub fn iter(&self) -> impl Iterator<Item = usize> {
            self.start..self.start + self.size
        }
    }

    pub const PIXELS: Region = Region {
        start: 0,
        size: PIXELS_SIZE,
    };
}

pub fn run(
    lifeline_rx: Receiver<()>,
    pixels_tx: Sender<Pixels>,
) -> anyhow::Result<()> {
    let path = Path::new("snake.stack");

    let (notify_tx, notify_rx) = unbounded();

    let mut watcher = notify::recommended_watcher(notify_tx)?;
    watcher.watch(path, RecursiveMode::NonRecursive)?;

    let mut run = 0;
    let mut eval = load(path)?;

    loop {
        match eval.run() {
            Effect::Yield => {
                let mut pixels = [0; PIXELS_SIZE_BYTES];
                for i in memory::PIXELS.iter() {
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
                        eval = load(path)?;
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

fn load(path: &Path) -> anyhow::Result<Eval> {
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
    // We don't intend to ever trigger a timeout using this channel. We might
    // overwrite the receiver later though.
    let (_timeout_tx, mut timeout_rx) = bounded::<Instant>(0);

    let mut event_received = false;

    loop {
        select! {
            recv(notify_rx) -> event => {
                let event = event??;

                if event_received {
                    // We have already received an event and are currently
                    // debouncing it.
                    continue;
                }

                let notify::EventKind::Modify(_) = event.kind else {
                    // We are only interested in changes to the script. Ignore.
                    continue;
                };

                // This is a change to the script, which we are interested in.
                // Set off the timer, so we can debounce the event before
                // returning.
                event_received = true;
                timeout_rx = after(Duration::from_millis(20));
            }
            recv(lifeline_rx) -> message => {
                let Err(RecvError) = message else {
                    unreachable!(
                        "Lifeline channel only exists to get disconnected."
                    );
                };

                // Sender has been dropped. We're done.
                return Ok(WaitForChangeOutcome::MustQuit);
            }
            recv(timeout_rx) -> _ => {
                *run += 1;
                return Ok(WaitForChangeOutcome::ScriptHasChanged);
            }
        }
    }
}

enum WaitForChangeOutcome {
    ScriptHasChanged,
    MustQuit,
}
