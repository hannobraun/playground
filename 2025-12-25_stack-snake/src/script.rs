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
use stack_assembly::{Effect, Eval, Script, Value};

use crate::{BYTES_PER_PIXEL, Input, PIXELS_SIZE_BYTES, Pixels};

mod memory {
    use crate::PIXELS_SIZE;

    pub struct Region {
        pub start: usize,
        pub size: usize,
    }

    impl Region {
        pub const fn end(&self) -> usize {
            self.start + self.size
        }

        pub fn iter(&self) -> impl Iterator<Item = usize> {
            self.start..self.end()
        }
    }

    pub const PIXELS: Region = Region {
        start: 0,
        size: PIXELS_SIZE,
    };
    pub const INPUT_INDICES: Region = Region {
        start: PIXELS.end(),
        size: 2,
    };
    pub const INPUT: Region = Region {
        start: INPUT_INDICES.end(),
        size: 8,
    };
    pub const GAME_STATE: Region = Region {
        start: INPUT.end(),
        size: 2,
    };
}

pub fn run(
    input_rx: Receiver<Input>,
    pixels_tx: Sender<Pixels>,
) -> anyhow::Result<()> {
    let path = Path::new("snake.stack");

    let (notify_tx, notify_rx) = unbounded();

    let mut watcher = notify::recommended_watcher(notify_tx)?;
    watcher.watch(path, RecursiveMode::NonRecursive)?;

    let mut run = 0;
    let (mut script, mut eval) = load(path)?;

    loop {
        let (effect, _) = eval.run(&script);

        for input in input_rx.try_iter() {
            let read_index = memory::INPUT_INDICES.start;
            let write_index = read_index + 1;

            let [read, write] = [read_index, write_index]
                .map(|i| eval.memory.values[i].to_u32() as usize);

            if write - read >= memory::INPUT.size {
                // Ring buffer is full. Throw away the input event.
                continue;
            }

            eval.memory.values
                [memory::INPUT.start + (write & (memory::INPUT.size - 1))] =
                Value::from(input as u32);
            eval.memory.values[write_index] =
                Value::from((write as u32).wrapping_add(1));
        }

        match effect {
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

                eval.clear_effect();
                continue;
            }
            effect => {
                eprintln!("{run}: Script triggered effect: {effect:?}");

                match wait_for_change(&mut run, &notify_rx, &input_rx)? {
                    WaitForChangeOutcome::ScriptHasChanged => {
                        (script, eval) = load(path)?;
                        continue;
                    }
                    WaitForChangeOutcome::InputReceived { input } => {
                        let _ = input;

                        // Right now, the only way to recover from an unhandled
                        // effect is to restart the evaluation. The new
                        // evaluation has no use for input from the old one, so
                        // we can just throw it away.
                    }
                    WaitForChangeOutcome::MustQuit => {
                        return Ok(());
                    }
                }
            }
        }
    }
}

fn load(path: &Path) -> anyhow::Result<(Script, Eval)> {
    let mut script = String::new();
    File::open(path)?.read_to_string(&mut script)?;

    let script = Script::compile(&script);
    let mut eval = Eval::new();

    // Give the script twice as much memory as the memory regions we use for I/O
    // take up.
    eval.memory.values = vec![Value::from(0); memory::INPUT.end() * 2];

    for i in memory::INPUT_INDICES.iter() {
        eval.memory.values[i] = Value::from(0);
    }

    for (address, value) in memory::GAME_STATE.iter().zip([16, 16]) {
        eval.memory.values[address] = value.into();
    }

    Ok((script, eval))
}

fn wait_for_change(
    run: &mut u64,
    notify_rx: &Receiver<notify::Result<notify::Event>>,
    input_rx: &Receiver<Input>,
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
            recv(input_rx) -> message => {
                let outcome = match message {
                    Ok(input) => {
                        WaitForChangeOutcome::InputReceived { input }
                    }
                    Err(RecvError) => {
                        // Sender has been dropped. We're done.
                        WaitForChangeOutcome::MustQuit
                    }
                };

                return Ok(outcome);
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
    InputReceived { input: Input },
    MustQuit,
}
