use std::fs;

use crate::types::date::{Day, Year};

pub mod aoc_client;
pub mod commands;
pub mod runner;

#[allow(dead_code)]
#[derive(Debug)]
pub enum InputType {
    Example,
    Input,
}

#[allow(dead_code)]
pub fn read_input(input_type: InputType, year: Year, day: Day) -> String {
    let path = match input_type {
        InputType::Example => format!("data/examples/{}-{}", year, day),
        InputType::Input => format!("data/inputs/{}-{}", year, day),
    };
    println!("{path}");
    let input = fs::read_to_string(path);
    input.expect("Could not read input file")
}
