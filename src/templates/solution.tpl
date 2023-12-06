use std::fmt::Display;

fn part1(input: &str) -> Option<impl Display> {
    Option::<usize>::None
}

fn part2(input: &str) -> Option<impl Display> {
    Option::<usize>::None
}

#[aoc::main(year = %YEAR%, day = %DAY%)]
fn main(input: &str) -> (Option<impl Display>, Option<impl Display>) {
    let part1 = part1(input);
    let part2 = part2(input);
    (part1, part2)
}