use clap::{Parser, Subcommand};

use crate::cmds::*;

mod cmds;
mod settings;
mod timestamps;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Show the current time in the configured timezones.
    Now,

    /// Show the current time in a specific timezone.
    As {
        /// The timezone name, as defined by IANA (ex: 'America/Los_Angeles')
        timezone: String,
    },

    /// Convert a provided timestamp into the configured timezones.
    Convert {
        /// A UTC timestamp to convert (ex: 2024-12-01 18:23:42).
        value: String,
    },

    /// Inspect the config file to confirm it is valid.
    Validate,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Now => cmd_now(),
        Commands::As { timezone } => cmd_as(timezone),
        Commands::Convert { value } => cmd_convert(value),
        Commands::Validate => cmd_validate(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
