use clap::{Parser, Subcommand};

mod api;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Search for a crate
    Search {
        /// PATTERN to search for
        pattern: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Search { pattern } => api::search_pattern(pattern),
    }
}
