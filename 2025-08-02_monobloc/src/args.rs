use std::path::PathBuf;

#[derive(clap::Parser)]
pub struct Args {
    pub program: Option<PathBuf>,

    #[arg(short, long)]
    pub interactive: bool,
}
