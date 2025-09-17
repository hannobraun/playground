mod evaluate;
mod node;
mod spec;
mod stack;

fn main() -> anyhow::Result<()> {
    spec::run_all_tests()?;
    Ok(())
}
