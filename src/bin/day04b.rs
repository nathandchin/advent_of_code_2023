use std::{
    collections::{HashMap, HashSet},
    io::{stdin, Read},
};

use color_eyre::eyre::{eyre, Result};

fn parse_numbers(s: &str) -> Result<HashSet<i32>> {
    s.split_whitespace()
        .map(|o| o.parse::<i32>().map_err(|_| eyre!("int parse error")))
        .collect()
}

// Clippy has a false positive warning when this function is called because
// `card_values` must be passed as reference as it is borrowed multiple times
// here
#[allow(clippy::needless_borrow)]
fn backtrack(id: i32, card_values: &HashMap<i32, i32>) -> i32 {
    let mut res = 1;
    let value = card_values.get(&id).expect("not in card_values");

    for i in (id + 1)..=(id + value) {
        res += backtrack(i, &card_values);
    }

    res
}

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;
    let input_lines: Vec<&str> = input.lines().collect();

    let mut card_values: HashMap<i32, i32> = HashMap::new();

    for line in input_lines.iter().rev() {
        let (id, numbers) = line.split_once(": ").unwrap();
        let id: i32 = id.split_whitespace().nth(1).unwrap().parse().unwrap();
        let (winning_set, owned_set) = numbers.split_once(" | ").unwrap();
        let num_matching_numbers = {
            let winning_set: HashSet<i32> = parse_numbers(winning_set)?;
            let owned_set: HashSet<i32> = parse_numbers(owned_set)?;
            (&winning_set & &owned_set).len() as i32
        };

        card_values.insert(id, num_matching_numbers);
    }

    let mut ans = 0;
    for id in 1..=card_values.len() {
        ans += backtrack(id as i32, &card_values);
    }

    println!("{}", ans);

    Ok(())
}
