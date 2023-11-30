use std::{fs::{File, OpenOptions, self}, process, io::Write};

use crate::{date::{Day, Year, get_latest_puzzle_day, get_latest_event_year}, util::aoc_client::get_client};

const MODULE_TEMPLATE: &str = r#"advent_of_code::solution!(DAY_NUMBER);

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
"#;

// TODO: client will only work with recent year
pub fn handle(year: Option<Year>, day: Option<Day>) {
    let year = match year {
        Some(year) => year,
        None => Year::new(get_latest_event_year()).unwrap(),
    };
    let day = match day {
        Some(day) => day,
        None => Day::new(get_latest_puzzle_day()).unwrap(),
    };
    let input_folder = format!("data/inputs/{}", year);
    let input_path = format!("data/inputs/{}/{}", year, day);
    let solution_folder = format!("src/solutions/{}", year);
    let solution_path = format!("src/solutions/{}/{}.rs", year, day);

    create_input(&input_folder, &input_path, &year, &day);
    create_solution(&solution_folder, &solution_path, &year, &day);
}

fn create_input(input_folder: &str, input_path: &str, year: &Year, day: &Day) {
    if let Err(e) = create_folder(input_folder) {
        eprintln!("Failed to create input folder: {e}");
        process::exit(1);
    }

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
        println!("Created empty input file \"{}\": Check if day is unlocked", input_path);
    }
}

fn create_solution(solution_folder: &str, solution_path: &str, _year: &Year, day: &Day) {
    if let Err(e) = create_folder(solution_folder) {
        eprintln!("Failed to create solution folder: {e}");
        process::exit(1);
    }

    let mut solution_file = match create_file(solution_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create solution file: {e}");
            process::exit(1);
        }
    };

    match solution_file.write_all(
        MODULE_TEMPLATE
            .replace("DAY_NUMBER", &day.into_inner().to_string())
            .as_bytes()
    ) {
        Ok(()) => {
            println!("Created solution file \"{}\"", solution_path);
        },
        Err(e) => {
            eprintln!("Failed to write solution contents: {e}");
            process::exit(1);
        }
    }
}

fn create_folder(folder: &str) -> Result<(), std::io::Error> {
    fs::create_dir_all(folder)
}

fn create_file(file: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().read(true).write(true).create_new(true).open(file)
}