use std::{io::Result, path::PathBuf};

use clap::Parser;

/// Simple CLI to use remove-dir-alls recursive deletion logic from the command
/// line.
#[derive(Parser)]
#[command(author, version, long_about = None)]
struct Cli {
    /// Paths to delete
    #[arg(value_name = "FILE")]
    names: Vec<PathBuf>,
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    for p in cli.names {
        remove_dir_all::remove_dir_all(p)?;
    }
    Ok(())
}
