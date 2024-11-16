use std::io::{stdin, Read};

use color_eyre::eyre::Result;

#[derive(Debug)]
struct CubeCounts {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<CubeCounts>,
}

fn parse_input(input: &str) -> Result<Vec<Game>> {
    input
        .lines()
        .into_iter()
        .map(|line| -> Result<Game> {
            let (prefix, rest) = line.split_once(": ").unwrap();
            let id: u32 = prefix.split_once(' ').unwrap().1.parse()?;
            Ok(Game {
                id,
                rounds: rest
                    .split("; ")
                    .map(|round| -> Result<CubeCounts> {
                        let mut counts = CubeCounts {
                            red: 0,
                            green: 0,
                            blue: 0,
                        };
                        for pairing in round.split(", ") {
                            let (count, color) = pairing.split_once(' ').unwrap();
                            let count: u32 = count.parse()?;
                            match color {
                                "red" => counts.red += count,
                                "green" => counts.green += count,
                                "blue" => counts.blue += count,
                                _ => panic!("unrecognized color"),
                            }
                        }
                        Ok(counts)
                    })
                    .collect::<Result<Vec<CubeCounts>>>()?,
            })
        })
        .collect()
}

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let counts = CubeCounts {
        red: 12,
        green: 13,
        blue: 14,
    };

    let games = parse_input(&input)?;

    let mut ans: u32 = 0;
    'outer: for game in games {
        for round in game.rounds {
            if round.red > counts.red || round.green > counts.green || round.blue > counts.blue {
                continue 'outer;
            }
        }
        ans += game.id;
    }

    println!("{}", ans);

    Ok(())
}
