use crate::types::date::{get_latest_event_year, get_latest_puzzle_day, Day, Year};

pub mod solution;
pub mod solve;

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
