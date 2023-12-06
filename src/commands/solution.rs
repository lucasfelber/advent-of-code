use std::{fs::OpenOptions, io::Write};

use crate::date::{Day, Year};

use super::CommandError;

pub fn handle(year: Year, day: Day, empty: bool) -> Result<(), CommandError> {
    let solution_path = format!("src/bin/{}-{}.rs", year, day);
    const SOLUTION_TEMPLATE: &str = include_str!("../templates/solution.tpl");

    match empty {
        true => {
            println!("=> Creating empty solution:");

            create_file(&solution_path)?;
            println!("=> => solution:   {}", &solution_path);
        }
        false => {
            println!("=> Creating filled solution:");

            match create_file(&solution_path)?.write_all(
                SOLUTION_TEMPLATE
                    .replace("%YEAR%", &year.into_inner().to_string())
                    .replace("%DAY%", &day.into_inner().to_string())
                    .as_bytes(),
            ) {
                Ok(_) => {
                    println!("=> => Trying to fill in solution template: SUCCESS!");
                    println!("=> => => solution:   {}", &solution_path);
                }
                Err(_) => {
                    println!("=> => Trying to fill in solution template: FAILURE!");
                    println!("=> => => Creating empty solution instead:");
                    println!("=> => => => solution:   {}", &solution_path);
                }
            }
        }
    }

    Ok(())
}

fn create_file(path: &str) -> Result<std::fs::File, std::io::Error> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(path)
}
