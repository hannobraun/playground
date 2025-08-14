use std::{fs::File, io::Read};

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let file = "add.mbl";

    let mut code = String::new();
    File::open(file)
        .with_context(|| format!("Opening `{file}`"))?
        .read_to_string(&mut code)
        .with_context(|| format!("Reading code from `{file}`"))?;

    println!("{code}");

    Ok(())
}
