use std::io::{stdin, Read};

use color_eyre::eyre::{OptionExt as _, Report, Result};
use regex::Regex;

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let regex = Regex::new(r#"(?<left>\d).*(?<right>\d)|(?<only>\d)"#)?;

    let ans = input.lines().try_fold(0, |sum, line| {
        let caps = regex.captures(line).ok_or_eyre("Parsing error")?;

        Ok::<u128, Report>(
            sum + if let Some(only) = caps.name("only") {
                only.as_str().repeat(2)
            } else if let (Some(left), Some(right)) = (caps.name("left"), caps.name("right")) {
                left.as_str().to_owned() + right.as_str()
            } else {
                unreachable!()
            }
            .parse::<u128>()?,
        )
    })?;

    println!("{}", ans);

    Ok(())
}
