use std::{fs::File, io::Read, iter::Peekable, str::Chars};

use anyhow::Context;

pub fn read_input_code<'a>(
    path: &str,
    buf: &'a mut String,
) -> anyhow::Result<InputCode<'a>> {
    File::open(path)
        .with_context(|| format!("Opening `{path}`"))?
        .read_to_string(buf)
        .with_context(|| format!("Reading code from `{path}`"))?;

    Ok(buf.chars().peekable())
}

pub type InputCode<'a> = Peekable<Chars<'a>>;
