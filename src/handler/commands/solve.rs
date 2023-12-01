use std::process::{Command, Stdio};

use crate::types::date::{Day, Year};

use super::get_year_day;

pub fn handle(year: Option<Year>, day: Option<Day>, release: bool) {
    let (year, day) = get_year_day(year, day);

    let path = format!("{}-{}", year, day);
    let mut cmd_args = vec!["run".to_string(), "--bin".to_string(), path];

    if release {
        cmd_args.push("--release".to_string());
    }

    let mut cmd = Command::new("cargo")
        .args(&cmd_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    cmd.wait().unwrap();
}
