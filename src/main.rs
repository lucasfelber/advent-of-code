mod args;
mod commands;
mod date;

use args::{Cli, Commands};
use clap::Parser;
use commands::puzzle;

fn main() {
    let cli = Cli::parse();

    match match cli.command {
        Commands::Puzzle {
            year,
            day,
            empty,
            no_input,
            no_solution,
        } => puzzle::handle(year, day, empty, no_input, no_solution),
    } {
        Ok(_) => (),
        Err(e) => eprintln!("{e}"),
    }
}
