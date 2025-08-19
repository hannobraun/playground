use std::{fs::File, io::Read, iter::Peekable, path::Path, str::Chars};

use anyhow::Context;

pub fn read_input_code<'a>(
    path: impl AsRef<Path>,
    buf: &'a mut String,
) -> anyhow::Result<InputCode<'a>> {
    let path = path.as_ref();

    File::open(path)
        .with_context(|| format!("Opening `{path}`", path = path.display()))?
        .read_to_string(buf)
        .with_context(|| {
            format!("Reading code from `{path}`", path = path.display())
        })?;

    Ok(buf.chars().peekable())
}

pub type InputCode<'a> = Peekable<Chars<'a>>;
