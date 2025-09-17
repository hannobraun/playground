mod evaluate;
mod nodes;
mod spec;
mod stack;

fn main() -> anyhow::Result<()> {
    spec::run_all_tests()?;
    Ok(())
}
