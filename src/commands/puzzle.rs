use crate::date::{Day, Year};

use super::{get_year_day, input, solution, CommandError};

pub fn handle(
    year: Option<Year>,
    day: Option<Day>,
    empty: bool,
    no_input: bool,
    no_solution: bool,
) -> Result<(), CommandError> {
    let (year, day) = get_year_day(year, day);

    println!("Creating puzzle for {}-{}:", year, day);

    if !no_input {
        input::handle(year, day, empty)?
    }
    if !no_solution {
        solution::handle(year, day, empty)?
    }

    println!("Done! Good Luck!");
    Ok(())
}
