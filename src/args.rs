use clap::{Parser, Subcommand};

use crate::date::{Day, Year};

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Puzzle {
        #[arg(short, long)]
        year: Option<Year>,
        #[arg(short, long)]
        day: Option<Day>,
        #[arg(short, long)]
        empty: bool,
        #[arg(long)]
        no_input: bool,
        #[arg(long)]
        no_solution: bool,
    },
}
