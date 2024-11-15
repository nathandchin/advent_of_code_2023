use std::io::{stdin, Read};

use color_eyre::{
    eyre::{eyre, OptionExt as _},
    Report, Result,
};
use regex::Regex;

fn parse(s: &str) -> Result<u128> {
    match s.parse::<u128>() {
        Ok(val) => Ok(val),
        Err(_) => {
            let val = match s {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                "zero" => 0,
                _ => return Err(eyre!("unknown \"digit\" {}", s)),
            };
            Ok(val)
        }
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    // The regex crate doesn't support backreferences, so we have to use `format!()` to insert the
    // word forms in all three spots manually
    let regex = Regex::new(&format!(
        r#"(?<left>\d|{0}).*(?<right>\d|{0})|(?<only>\d|{0})"#,
        "one|two|three|four|five|six|seven|eight|nine|zero"
    ))?;

    let ans = input.lines().try_fold(0, |sum, line| {
        let caps = regex.captures(line).ok_or_eyre("Parsing error")?;

        Ok::<u128, Report>(
            sum + if let Some(only) = caps.name("only") {
                let val = parse(only.as_str())?;
                val * 10 + val
            } else if let (Some(left), Some(right)) = (caps.name("left"), caps.name("right")) {
                parse(left.as_str())? * 10 + parse(right.as_str())?
            } else {
                unreachable!()
            },
        )
    })?;

    println!("{}", ans);

    Ok(())
}
