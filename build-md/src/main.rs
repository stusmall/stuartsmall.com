use build_md::{build_folder, build_md};
use clap::Parser;
use std::error::Error;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    debug: bool,
    input: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    if cli.input.metadata()?.is_dir() {
        build_folder(cli.debug, &cli.input)
    } else {
        build_md(cli.debug, &cli.input)
    }
}
