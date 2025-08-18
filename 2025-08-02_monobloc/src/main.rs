mod compiler;
mod runtime;
mod tests;

fn main() -> anyhow::Result<()> {
    use crate::tests::compile;

    let stack = compile("examples/single-number.mbl")?;
    assert_eq!(stack, vec![1]);

    Ok(())
}
