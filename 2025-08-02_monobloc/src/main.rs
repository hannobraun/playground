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

    let mut stack = Vec::<i32>::new();

    for word in code.split_whitespace() {
        match word {
            "+" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                stack.push(a + b);
            }
            identifier => {
                if let Ok(value) = identifier.parse() {
                    stack.push(value);
                } else {
                    println!("Unknown identifier: `{identifier}`");
                }
            }
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
