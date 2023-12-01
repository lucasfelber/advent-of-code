use aoc_client::{AocClient, AocResult};

use crate::types::date::{Day, Year};

pub fn get_client(year: &Year, day: &Day, input: &str) -> AocResult<AocClient> {
    AocClient::builder()
        .session_cookie_from_file("aoc.session")?
        .year(year.into_inner())?
        .day(day.into_inner())?
        .overwrite_files(true)
        .input_filename(input)
        .build()
}
