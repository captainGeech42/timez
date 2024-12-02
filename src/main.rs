use clap::{Parser, Subcommand};

use crate::cmds::*;

mod cmds;
mod settings;
mod timestamps;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Show the current time in the configured timezones.
    Now,

    /// Inspect the config file to confirm it is valid.
    Validate,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Now) => cmd_now(),
        Some(Commands::Validate) => cmd_validate(),
        None => {}
    }
}
