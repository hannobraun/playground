mod compiler;
mod runtime;

fn main() -> anyhow::Result<()> {
    let code = compiler::backend::generate()?;
    runtime::execute(&code)?;

    Ok(())
}
