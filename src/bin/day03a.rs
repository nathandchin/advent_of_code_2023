use std::io::{stdin, Read};

use color_eyre::eyre::Result;

type Grid = Vec<Vec<char>>;

fn is_symbol_adjacent(grid: &Grid, row_idx: i32, col_idx: i32) -> bool {
    let rs = [-1, -1, -1, 0, 0, 1, 1, 1];
    let cs = [-1, 0, 1, -1, 1, -1, 0, 1];

    for (r, c) in rs.iter().zip(cs) {
        let r = (row_idx + r).clamp(0, (grid.len() - 1) as i32);
        let c = (col_idx + c).clamp(0, (grid[0].len() - 1) as i32);

        let ch = grid[r as usize][c as usize];
        if ch != '.' && !ch.is_ascii_digit() {
            return true;
        }
    }

    false
}

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let grid: Grid = input.lines().map(|line| line.chars().collect()).collect();
    let mut ans = 0;

    for y in 0..grid.len() {
        let mut x = 0;
        while x < grid[0].len() {
            let c = grid[y][x];

            if c.is_ascii_digit() {
                let mut end = x;
                let mut is_part_number = false;
                while end < grid[0].len() && grid[y][end].is_ascii_digit() {
                    is_part_number =
                        is_part_number || is_symbol_adjacent(&grid, y as i32, end as i32);
                    end += 1;
                }

                if is_part_number {
                    ans += grid[y][x..end].iter().collect::<String>().parse::<i32>()?;
                }

                x = end;
            }

            x += 1;
        }
    }

    println!("{}", ans);

    Ok(())
}
