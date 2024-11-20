use std::{
    collections::HashSet,
    io::{stdin, Read},
};

use color_eyre::eyre::{eyre, Result};

fn parse_numbers(s: &str) -> Result<HashSet<i32>> {
    s.split_whitespace()
        .map(|o| o.parse::<i32>().map_err(|_| eyre!("int parse error")))
        .collect()
}

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let mut ans: u32 = 0;

    for line in input.lines() {
        let numbers = line.split_once(": ").unwrap().1;

        let (winning_set, owned_set) = numbers.split_once(" | ").unwrap();

        let winning_set: HashSet<i32> = parse_numbers(winning_set)?;
        let owned_set: HashSet<i32> = parse_numbers(owned_set)?;

        let owned_winners = &winning_set & &owned_set;

        let num_winners = owned_winners.len() as u32;
        if num_winners > 0 {
            ans += 2_u32.pow(num_winners - 1);
        }
    }

    println!("{}", ans);

    Ok(())
}
