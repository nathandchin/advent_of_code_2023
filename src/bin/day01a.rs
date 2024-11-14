use color_eyre::eyre::{eyre, Result};

static INPUT: &str = include_str!("../../input/day01.real");

fn main() -> Result<()> {
    let r = regex::Regex::new(r#"(?<left>\d).*(?<right>\d)|(?<only>\d)"#)?;

    let mut sum: u128 = 0;

    for line in INPUT.lines() {
        let caps = r.captures(line).ok_or(eyre!("Parsing error"))?;

        let calibration_value = if let Some(only) = caps.name("only") {
            let mut s = String::new();
            s.push_str(only.as_str());
            s.push_str(only.as_str());
            s.to_string()
        } else {
            let left = caps.name("left").expect("malformed input");
            let right = caps.name("right").expect("malformed input");

            let mut s = String::new();
            s.push_str(left.as_str());
            s.push_str(right.as_str());
            s
        };

        let calibration_value: u128 = calibration_value.parse().expect("integer parsing error");
        sum += calibration_value;
    }

    println!("{}", sum);

    Ok(())
}
