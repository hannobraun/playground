#[cfg(test)]
use crate::tests::compile;

#[cfg(test)]
mod compiler;
#[cfg(test)]
mod runtime;
#[cfg(test)]
mod tests;

fn main() -> anyhow::Result<()> {
    Ok(())
}

#[test]
fn single_number() -> anyhow::Result<()> {
    let stack = compile("examples/single-number.mbl")?;
    assert_eq!(stack, vec![1]);
    Ok(())
}
