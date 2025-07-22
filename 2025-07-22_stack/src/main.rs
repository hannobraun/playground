mod compiler;
mod runtime;

fn main() -> anyhow::Result<()> {
    let tokens = compiler::tokenizer::tokenize();
    let program = compiler::parser::parse(tokens.into())?;
    let code = compiler::backend::compile_program(program)?;
    runtime::execute(&code)?;

    Ok(())
}
