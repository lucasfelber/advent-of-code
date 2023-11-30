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
    Solution {
        #[arg(long)]
        year: Option<Year>,
        #[arg(long)]
        day: Option<Day>,
    },
}