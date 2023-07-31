use std::panic;

use crossterm::terminal;

use crate::{
    audio::Audio,
    synth::{self},
    ui,
};

pub fn run() -> anyhow::Result<()> {
    terminal::enable_raw_mode()?;
    let result = panic::catch_unwind(run_inner);
    terminal::disable_raw_mode()?;

    // This would probably be a good case for `Result::flatten`, but as of this
    // writing, that is not stable yet.
    match result {
        Ok(Ok(())) => Ok(()),
        Ok(err) => err,
        Err(payload) => panic::resume_unwind(payload),
    }
}

fn run_inner() -> anyhow::Result<()> {
    let audio = Audio::start()?;
    let input = ui::start();
    let join_handle = synth::start::start(input, audio.buffers);

    join_handle.join().unwrap();

    Ok(())
}
