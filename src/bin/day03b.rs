use std::{
    collections::HashMap,
    io::{stdin, Read},
};

use color_eyre::eyre::Result;

type Grid = Vec<Vec<char>>;

fn star_adjacent(grid: &Grid, row_idx: i32, col_idx: i32) -> Option<(usize, usize)> {
    let rs = [-1, -1, -1, 0, 0, 1, 1, 1];
    let cs = [-1, 0, 1, -1, 1, -1, 0, 1];

    for (r, c) in rs.iter().zip(cs) {
        let r = (row_idx + r).clamp(0, (grid.len() - 1) as i32);
        let c = (col_idx + c).clamp(0, (grid[0].len() - 1) as i32);

        if grid[r as usize][c as usize] == '*' {
            return Some((r as usize, c as usize));
        }
    }

    None
}

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let grid: Grid = input.lines().map(|line| line.chars().collect()).collect();
    let mut gears: HashMap<(usize, usize), (i32, i32)> = HashMap::new();

    for y in 0..grid.len() {
        let mut x = 0;
        while x < grid[0].len() {
            let c = grid[y][x];

            if c.is_ascii_digit() {
                let mut end = x;
                let mut star_pos_opt = None;
                while end < grid[0].len() && grid[y][end].is_ascii_digit() {
                    if let None = star_pos_opt {
                        star_pos_opt = star_adjacent(&grid, y as i32, end as i32);
                    }
                    end += 1;
                }

                if let Some(star_pos) = star_pos_opt {
                    let entry = gears.entry(star_pos);
                    let val = grid[y][x..end].iter().collect::<String>().parse::<i32>()?;

                    entry.and_modify(|e| e.1 = val).or_insert((val, 0));
                }

                x = end;
            }

            x += 1;
        }
    }

    let ans = gears.iter().fold(0, |acc, e| acc + (e.1 .0 * e.1 .1));

    println!("{}", &ans);

    Ok(())
}
