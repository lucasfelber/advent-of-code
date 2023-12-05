use regex::Regex;

use aoc::{types::{part::Part, date::{Year, Day}}, handler::{read_input, runner::run_part, InputType}};

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"((1[3-9]|[2-9]\d|\d{3,})\sred)|((1[4-9]|[2-9]\d|\d{3,})\sgreen)|((1[5-9]|[2-9]\d|\d{3,})\sblue)").unwrap();

    let result = input.lines().enumerate().fold(0, |result, (index, line)| {
        if re.is_match(line) {
            result
        } else {
            result + index + 1
        }
    });
    
    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let res = vec![
        Regex::new(r"(?<num>\d+)\sred").unwrap(),
        Regex::new(r"(?<num>\d+)\sgreen").unwrap(),
        Regex::new(r"(?<num>\d+)\sblue").unwrap(),
    ];

    let result: i32 = input.lines().map(|line| {
        res.iter().map(|re| {
            re.captures_iter(line).map(|caps| {
                caps["num"].parse::<i32>().unwrap()
            }).max().unwrap()
        }).product::<i32>()
    }).sum();

    Some(result as u32)
}

fn main() {
    let year = Year::new(2023).unwrap();
    let day = Day::new(2).unwrap();

    let example = read_input(InputType::Example, year, day);
    let input = read_input(InputType::Input, year, day);

    run_part(part_one, InputType::Example, &example, year, day, Part::One);
    run_part(part_one, InputType::Input, &input, year, day, Part::One);

    run_part(part_two, InputType::Example,&example, year, day, Part::Two);
    run_part(part_two, InputType::Input, &input, year, day, Part::Two);
}
