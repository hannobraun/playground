mod compiler;
mod runtime;

fn main() -> anyhow::Result<()> {
    let program = compiler::frontend::parse();
    let code = compiler::backend::generate(program)?;
    runtime::execute(&code)?;

    Ok(())
}
