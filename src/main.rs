mod args;
mod handler;
mod types;

use args::{Cli, Commands};
use clap::Parser;
use handler::commands::{solution, solve};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Solution { year, day } => solution::handle(year, day),
        Commands::Solve { year, day, release } => solve::handle(year, day, release),
    }
}
