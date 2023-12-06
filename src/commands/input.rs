use std::{fs::OpenOptions, io::Write, process};

use aoc_client::AocClient;
use regex::Regex;

use crate::date::{Day, Year};

use super::CommandError;

pub fn handle(year: Year, day: Day, empty: bool) -> Result<(), CommandError> {
    let expample_path = format!("data/examples/{}-{}.in", year, day);
    let input_path = format!("data/inputs/{}-{}.in", year, day);

    let aoc_client = AocClient::builder()
        .session_cookie_from_file("aoc.session")?
        .year(year.into_inner())?
        .day(day.into_inner())?
        .overwrite_files(true)
        .input_filename(&input_path)
        .build()?;

    match (empty, aoc_client.day_unlocked()) {
        (true, _) => {
            println!("=> Creating empty inputs:");

            create_file(&input_path)?;
            println!("=> => input:      {}", &input_path);

            create_file(&expample_path)?;
            println!("=> => example:    {}", &expample_path);
        }
        (false, true) => {
            println!("=> Creating filled inputs:");

            match aoc_client.save_input() {
                Ok(_) => {
                    println!("=> => Trying to access aoc input: SUCCESS!");
                    println!("=> => => input:      {}", &input_path);
                }
                Err(_) => {
                    println!("=> => Trying to access aoc input: FAILURE!");
                    println!("=> => => Creating empty input instead:");
                    create_file(&input_path)?;
                    println!("=> => => => input:      {}", &input_path);
                }
            }

            let mut file = create_file(&expample_path)?;

            let puzzle_html = aoc_client.get_puzzle_html()?;
            let captures = Regex::new(r"<pre><code>(?<example>(.|\n)*?)<\/code><\/pre>")
                .unwrap()
                .captures(&puzzle_html);

            match captures {
                Some(captures) => {
                    file.write_all(captures.name("example").unwrap().as_str().as_bytes())?;
                    println!("=> => Trying to find example aoc html: SUCCESS!");
                    println!("=> => => example:    {}", &expample_path);
                }
                None => {
                    println!("=> => Trying to find example aoc html: FAILURE!");
                    println!("=> => => Creating empty example instead:");
                    println!("=> => => => example:    {}", &expample_path);
                }
            }
        }
        (false, false) => {
            eprintln!("=> Day not unlocked! Create files anyway with --empty");
            process::exit(1);
        }
    };

    Ok(())
}

fn create_file(path: &str) -> Result<std::fs::File, std::io::Error> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
}
