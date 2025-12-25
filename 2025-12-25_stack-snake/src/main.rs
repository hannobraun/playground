use std::{fs::File, io::Read};

fn main() -> anyhow::Result<()> {
    let mut script = String::new();
    File::open("snake.stack")?.read_to_string(&mut script)?;

    dbg!(script);

    Ok(())
}
