use clap::{Parser, Subcommand};

mod api;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(long, short, default_value = "false")]
    colored: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Search for a crate
    Search {
        /// PATTERN to search for
        pattern: String,
        #[arg(default_value = "10", short, long, value_parser=clap::value_parser!(u8).range(1..=30))]
        limit: u8,
    },
}

fn main() {
    let cli = Cli::parse();
    let colored = cli.colored;
    match &cli.command {
        Commands::Search { pattern, limit } => api::search_pattern(pattern, *limit, colored),
    }
}
