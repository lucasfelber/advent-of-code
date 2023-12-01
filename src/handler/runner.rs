use std::{
    fmt::Display,
    time::{Duration, Instant},
};

use crate::types::{
    date::{Day, Year},
    part::Part,
};

use super::InputType;

#[allow(dead_code)]
pub fn run_part<I: Clone, R: Display>(
    function: impl Fn(I) -> Option<R>,
    input_type: InputType,
    input: I,
    year: Year,
    day: Day,
    part: Part,
) {
    println!("{year} {day}");
    println!("{part} {:?}", input_type);

    let timer = Instant::now();
    let result = function(input.clone());
    let elapsed = timer.elapsed();

    print_result(&result, elapsed);
    println!();
}

#[allow(dead_code)]
fn print_result<R: Display>(result: &Option<R>, duration: Duration) {
    match result {
        Some(result) => {
            println!("({duration:.1?})");
            println!("{result}");
        }
        None => println!("no result"),
    }
}
