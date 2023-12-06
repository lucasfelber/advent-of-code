use std::{error::Error, fmt::Display, str::FromStr};

use chrono::{DateTime, Datelike, FixedOffset, TimeZone, Utc};

const FIRST_PUZZLE_DAY: u32 = 1;
const LAST_PUZZLE_DAY: u32 = 25;
const FIRST_EVENT_YEAR: i32 = 2015;
const DECEMBER: u32 = 12;
const RELEASE_TIMEZONE_OFFSET: i32 = -5 * 3600;

fn get_aoc_now() -> DateTime<FixedOffset> {
    FixedOffset::east_opt(RELEASE_TIMEZONE_OFFSET)
        .unwrap()
        .from_utc_datetime(&Utc::now().naive_utc())
}

#[derive(Debug, Clone, Copy)]
pub struct Day(u32);

pub fn get_latest_puzzle_day() -> u32 {
    let now = get_aoc_now();

    if get_latest_event_year() < now.year() {
        LAST_PUZZLE_DAY
    } else if now.day() < 25 {
        now.day()
    } else {
        LAST_PUZZLE_DAY
    }
}

impl Day {
    pub fn new(day: u32) -> Option<Self> {
        if day == FIRST_PUZZLE_DAY - 1 || day > LAST_PUZZLE_DAY {
            return None;
        }
        Some(Self(day))
    }

    pub fn into_inner(self) -> u32 {
        self.0
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}

impl FromStr for Day {
    type Err = DayFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let day = s.parse().map_err(|_| DayFromStrError)?;
        Self::new(day).ok_or(DayFromStrError)
    }
}

#[derive(Debug)]
pub struct DayFromStrError;

impl Error for DayFromStrError {}

impl Display for DayFromStrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "expected a number between {} and {}",
                FIRST_PUZZLE_DAY, LAST_PUZZLE_DAY
            )
            .as_str(),
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Year(i32);

pub fn get_latest_event_year() -> i32 {
    let now = get_aoc_now();

    if now.month() < DECEMBER {
        now.year() - 1
    } else {
        now.year()
    }
}

impl Year {
    pub fn new(year: i32) -> Option<Self> {
        if year < FIRST_EVENT_YEAR || year > get_latest_event_year() {
            return None;
        }
        Some(Self(year))
    }

    pub fn into_inner(self) -> i32 {
        self.0
    }
}

impl Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Year {
    type Err = YearFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let year = s.parse().map_err(|_| YearFromStrError)?;
        Self::new(year).ok_or(YearFromStrError)
    }
}

#[derive(Debug)]
pub struct YearFromStrError;

impl Error for YearFromStrError {}

impl Display for YearFromStrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "expected a number between {} and {}",
                FIRST_EVENT_YEAR,
                get_latest_event_year()
            )
            .as_str(),
        )
    }
}
