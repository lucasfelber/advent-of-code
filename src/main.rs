mod args;
mod date;
mod template;
mod util;


use args::{Cli, Commands};
use clap::Parser;
use template::commands::solution;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Solution { year, day } => solution::handle(year, day),
    }
}