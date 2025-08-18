mod compiler;
mod runtime;
mod tests;

fn main() -> anyhow::Result<()> {
    use crate::tests::compile;

    let path = "examples/single-number.mbl";

    let stack = compile(path)?;

    assert_eq!(stack, vec![1]);

    Ok(())
}
