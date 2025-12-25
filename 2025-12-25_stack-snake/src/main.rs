use std::{fs::File, io::Read};

use stack_assembly::Eval;

fn main() -> anyhow::Result<()> {
    let mut script = String::new();
    File::open("snake.stack")?.read_to_string(&mut script)?;

    let mut eval = Eval::start(&script);

    let effect = eval.run();
    eprintln!("Script triggered effect: {effect:?}");

    Ok(())
}
