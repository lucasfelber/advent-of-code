use thiserror::Error;

use crate::date::{get_latest_event_year, get_latest_puzzle_day, Day, Year};

mod input;
mod solution;

pub mod puzzle;

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Error while interacting with the aoc client: {0}")]
    Aoc(#[from] aoc_client::AocError),
    #[error("Error while interacting with the file system: {0}")]
    Io(#[from] std::io::Error),
}

pub fn get_year_day(year: Option<Year>, day: Option<Day>) -> (Year, Day) {
    let year = match year {
        Some(year) => year,
        None => Year::new(get_latest_event_year()).unwrap(),
    };
    let day = match day {
        Some(day) => day,
        None => Day::new(get_latest_puzzle_day()).unwrap(),
    };

    (year, day)
}
