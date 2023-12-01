use std::{
    fs::{File, OpenOptions},
    io::Write,
    process,
};

use crate::{
    handler::aoc_client::get_client,
    types::date::{Day, Year},
};

use super::get_year_day;

const MODULE_TEMPLATE: &str = r#"use aoc::{types::{part::Part, date::{Year, Day}}, handler::{read_input, runner::run_part, InputType}};

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let year = Year::new(YEAR_NUMBER).unwrap();
    let day = Day::new(DAY_NUMBER).unwrap();

    let example = read_input(InputType::Example, year, day);
    let input = read_input(InputType::Input, year, day);

    run_part(part_one, InputType::Example, &example, year, day, Part::One);
    run_part(part_one, InputType::Input, &input, year, day, Part::One);

    run_part(part_two, InputType::Example,&example, year, day, Part::Two);
    run_part(part_two, InputType::Input, &input, year, day, Part::Two);
}
"#;

pub fn handle(year: Option<Year>, day: Option<Day>) {
    let (year, day) = get_year_day(year, day);

    let expample_path = format!("data/examples/{}-{}", year, day);
    let input_path = format!("data/inputs/{}-{}", year, day);
    let solution_path = format!("src/bin/{}-{}.rs", year, day);

    create_example(&expample_path);
    create_input(&input_path, &year, &day);
    create_solution(&solution_path, &year, &day);
}

fn create_example(example_path: &str) {
    match create_file(example_path, false) {
        Ok(_file) => {
            println!("Created empty example file \"{}\"", example_path);
        }
        Err(e) => {
            eprintln!("Failed to create example file: {e}");
            process::exit(1);
        }
    };
}

fn create_input(input_path: &str, year: &Year, day: &Day) {
    let client = match get_client(year, day, input_path) {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to create aoc client: {e}");
            process::exit(1);
        }
    };

    if client.day_unlocked() {
        match client.save_input() {
            Ok(()) => println!("Created filled input file \"{}\"", input_path),
            Err(e) => {
                eprintln!("Failed to create input file: {e}");
                process::exit(1);
            }
        }
    } else {
        match create_file(input_path, false) {
            Ok(_file) => {
                println!(
                    "Created empty input file \"{}\": Check if day is unlocked",
                    input_path
                );
            }
            Err(e) => {
                eprintln!("Failed to create solution file: {e}");
                process::exit(1);
            }
        };
    }
}

fn create_solution(solution_path: &str, year: &Year, day: &Day) {
    let mut solution_file = match create_file(solution_path, true) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create solution file: {e}");
            process::exit(1);
        }
    };

    match solution_file.write_all(
        MODULE_TEMPLATE
            .replace("YEAR_NUMBER", &year.into_inner().to_string())
            .replace("DAY_NUMBER", &day.into_inner().to_string())
            .as_bytes(),
    ) {
        Ok(()) => {
            println!("Created solution file \"{}\"", solution_path);
        }
        Err(e) => {
            eprintln!("Failed to write solution contents: {e}");
            process::exit(1);
        }
    }
}

fn create_file(file: &str, safe: bool) -> Result<File, std::io::Error> {
    match safe {
        true => OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(file),
        false => OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file),
    }
}
