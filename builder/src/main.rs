use clap::Parser;

mod build;
mod upgrade;
mod util;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Some(Command::Build) | None => {
            build::build()?;
        }
        Some(Command::Upgrade) => {
            upgrade::upgrade()?;
        }
    }

    Ok(())
}

#[derive(clap::Parser)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(clap::Subcommand)]
enum Command {
    Build,
    Upgrade,
}
