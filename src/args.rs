use clap::{Parser, Subcommand};

use crate::types::date::{Day, Year};

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Solution {
        #[arg(short, long)]
        year: Option<Year>,
        #[arg(short, long)]
        day: Option<Day>,
    },
    Solve {
        #[arg(short, long)]
        year: Option<Year>,
        #[arg(short, long)]
        day: Option<Day>,
        #[arg(short, long)]
        release: bool,
    },
}
