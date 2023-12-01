use aoc::{types::{part::Part, date::{Year, Day}}, handler::{read_input, runner::run_part, InputType}};

const DIGITS: [&str; 9] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];

pub fn part_one(input: &str) -> Option<u32> {
    let result: u32 = input.lines().map(|line| {
        line.chars().find_map(|c| c.to_digit(10)).unwrap() * 10 + 
        line.chars().rev().find_map(|c| c.to_digit(10)).unwrap()
    }).sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result: u32 = input.lines().map(|line| {
        line.char_indices().find_map(|(i, c)| {
            c.to_digit(10).or(
            (0..).zip(DIGITS.iter()).find_map(|(value, digit)| {
                line[i..].starts_with(digit).then_some(value + 1)
            }))
        }).unwrap() * 10 +
        line.char_indices().rev().find_map(|(i, c)| {
            c.to_digit(10).or(
            (0..).zip(DIGITS.iter()).find_map(|(value, digit)| {
                line[..i + 1].ends_with(digit).then_some(value + 1)
            }))
        }).unwrap()
    }).sum();

    println!("{:?}", result);
    None
}

fn main() {
    let year = Year::new(2023).unwrap();
    let day = Day::new(1).unwrap();

    let example = read_input(InputType::Example, year, day);
    let input = read_input(InputType::Input, year, day);

    run_part(part_one, InputType::Example, &example, year, day, Part::One);
    run_part(part_one, InputType::Input, &input, year, day, Part::One);

    run_part(part_two, InputType::Example,&example, year, day, Part::Two);
    run_part(part_two, InputType::Input, &input, year, day, Part::Two);
}
