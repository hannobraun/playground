use std::{fs::File, io::Read, path::Path, sync::mpsc};

use notify::{RecursiveMode, Watcher};
use stack_assembly::Eval;

fn main() -> anyhow::Result<()> {
    run_script()?;
    Ok(())
}

fn run_script() -> anyhow::Result<()> {
    let (notify_tx, notify_rx) = mpsc::channel();

    let mut watcher = notify::recommended_watcher(notify_tx)?;
    watcher.watch(Path::new("snake.stack"), RecursiveMode::NonRecursive)?;

    let mut run = 0;

    'outer: loop {
        let mut script = String::new();
        File::open("snake.stack")?.read_to_string(&mut script)?;

        let mut eval = Eval::start(&script);

        let effect = eval.run();
        eprintln!("{run}: Script triggered effect: {effect:?}");

        'inner: loop {
            let event = notify_rx.recv()??;

            match event.kind {
                notify::EventKind::Modify(_) => {
                    run += 1;
                    continue 'outer;
                }
                _ => {
                    continue 'inner;
                }
            }
        }
    }
}
