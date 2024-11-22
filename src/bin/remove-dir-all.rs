use std::{io::Result, path::PathBuf};

use clap::{Parser, ValueEnum};

/// What kind of parallelism to use
#[derive(Debug, Clone, Copy, ValueEnum)]
enum Parallelism {
    /// No operations in parallel
    Serial,
    /// Parallelise readdir and unlink operations
    Parallel,
}

/// Simple CLI to use remove-dir-alls recursive deletion logic from the command
/// line.
#[derive(Parser)]
#[command(author, version, long_about = None)]
struct Cli {
    /// Paths to delete
    #[arg(value_name = "FILE")]
    names: Vec<PathBuf>,
    /// Choose the parallelism strategy
    #[arg(short = 'p', long = "parallelism")]
    parallelism: Option<Parallelism>,
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    let remover = match cli.parallelism {
        None => remove_dir_all::RemoverBuilder::new().build(),
        Some(Parallelism::Serial) => remove_dir_all::RemoverBuilder::new().serial().build(),
        Some(Parallelism::Parallel) => remove_dir_all::RemoverBuilder::new().parallel().build(),
    };

    for p in cli.names {
        remover.remove_dir_all(&p)?;
    }
    Ok(())
}
