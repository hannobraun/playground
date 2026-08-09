mod build;
mod util;

fn main() -> anyhow::Result<()> {
    build::build()?;
    Ok(())
}
