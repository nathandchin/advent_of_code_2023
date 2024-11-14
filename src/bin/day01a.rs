use std::io::{stdin, Read};

use color_eyre::eyre::{eyre, Result};

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let r = regex::Regex::new(r#"(?<left>\d).*(?<right>\d)|(?<only>\d)"#)?;

    let mut sum: u128 = 0;

    for line in input.lines() {
        let caps = r.captures(line).ok_or(eyre!("Parsing error"))?;

        let calibration_value = if let Some(only) = caps.name("only") {
            only.as_str().repeat(2)
        } else if let (Some(left), Some(right)) = (caps.name("left"), caps.name("right")) {
            left.as_str().to_owned() + right.as_str()
        } else {
            unreachable!()
        };

        let calibration_value: u128 = calibration_value
            .parse()
            .map_err(|_| eyre!("integer parsing error"))?;

        sum += calibration_value;
    }

    println!("{}", sum);

    Ok(())
}
