mod compiler;
mod runtime;

fn main() -> anyhow::Result<()> {
    let module = compiler::backend::generate()?;
    runtime::execute(&module)?;

    Ok(())
}
