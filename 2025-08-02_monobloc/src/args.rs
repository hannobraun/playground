use std::path::PathBuf;

#[derive(clap::Parser)]
pub struct Args {
    pub program: Option<PathBuf>,

    #[arg(short, long)]
    pub interactive: bool,
}

impl Args {
    pub fn parse() -> Self {
        <Self as clap::Parser>::parse()
    }
}
