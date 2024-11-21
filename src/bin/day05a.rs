use std::io::{stdin, Read};

use color_eyre::eyre::Result;

#[derive(Debug)]
struct Map {
    mappings: Vec<Mapping>,
}

#[derive(Debug)]
struct Mapping {
    src: u128,
    dest: u128,
    size: u128,
}

impl Map {
    fn process(&self, n: u128) -> u128 {
        if let Some(mapping_idx) = self
            .mappings
            .iter()
            .position(|m| m.src <= n && n <= m.src + m.size)
        {
            let mapping = &self.mappings[mapping_idx];
            n + mapping.dest - mapping.src
        } else {
            n
        }
    }
}

fn parse_input(input: &str) -> (Vec<u128>, Vec<Map>) {
    let chunks: Vec<&str> = input.split("\n\n").collect();
    let seeds = chunks[0]
        .split_whitespace()
        .skip(1) // Skip "seeds: "
        .map(|o| o.parse().unwrap())
        .collect();
    let mut almanac = vec![];

    let maps: &[&str] = &chunks[1..];
    for map in maps {
        let lines: Vec<_> = map.lines().collect();
        let mut mappings = vec![];
        for line in &lines[1..] {
            let parts: Vec<u128> = line
                .split_whitespace()
                .map(|o| o.parse().unwrap())
                .collect();
            mappings.push(Mapping {
                src: parts[1],
                dest: parts[0],
                size: parts[2],
            })
        }

        almanac.push(Map { mappings });
    }

    (seeds, almanac)
}

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;
    let (seeds, almanac) = parse_input(&input);

    let mut locations = vec![];
    for seed in seeds {
        let mut res = seed;
        for map in &almanac {
            res = map.process(res);
        }
        locations.push(res);
    }

    println!("{}", locations.iter().min().unwrap());

    Ok(())
}
