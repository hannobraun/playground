use std::{panic, thread};

use crossbeam_channel::bounded;

mod io;
mod script;

const GRID_SIZE: usize = 32;
const PIXELS_SIZE: usize = GRID_SIZE * GRID_SIZE;
const BYTES_PER_PIXEL: usize = 4;
const PIXELS_SIZE_BYTES: usize = PIXELS_SIZE * BYTES_PER_PIXEL;

type Pixels = [u8; PIXELS_SIZE_BYTES];

fn main() -> anyhow::Result<()> {
    let (lifeline_tx, lifeline_rx) = bounded(0);
    let (pixels_tx, pixels_rx) = bounded(0);

    let handle = thread::spawn(|| script::run_script(lifeline_rx, pixels_tx));
    io::start_and_wait(lifeline_tx, pixels_rx)?;

    match handle.join() {
        Ok(result) => result?,
        Err(err) => {
            panic::resume_unwind(err);
        }
    }

    Ok(())
}
