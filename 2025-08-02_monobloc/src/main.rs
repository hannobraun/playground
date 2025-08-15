use std::{fs::File, io::Read};

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let file = "add.mbl";

    let mut input_code = String::new();
    File::open(file)
        .with_context(|| format!("Opening `{file}`"))?
        .read_to_string(&mut input_code)
        .with_context(|| format!("Reading code from `{file}`"))?;

    println!("{input_code}");

    let mut stack = Vec::<i32>::new();

    for identifier in input_code.split_whitespace() {
        if let Ok(value) = identifier.parse() {
            stack.push(value);
        } else {
            println!("Unknown identifier: `{identifier}`");
        }
    }

    for (i, value) in stack.into_iter().enumerate() {
        if i > 0 {
            print!(" ");
        }

        print!("{value}");
    }

    println!();

    Ok(())
}
