use anyhow::{bail, Context, Result};
use common::get_input_file_lines_with_variant;

fn walk_forwards(grid: &mut Vec<Vec<isize>>) -> Result<usize> {
    if grid.is_empty() {
        bail!("cannot have zero rows in grid");
    }
    if grid[0].is_empty() {
        bail!("cannot have zero columns in grid");
    }

    let height = grid.len();
    let width = grid.len();
    let mut rows = vec![0; height];
    let mut cols = vec![0; width];
    let mut count = 0;
    for y in 0..height {
        if grid[y].len() != width {
            bail!("all rows must be the same width");
        }
        for x in 0..width {
            let cell = grid[y][x];
            let abs = cell.abs();
            let mut inc = false;
            if abs >= rows[y] {
                rows[y] = abs + 1;
                inc = true;
            }
            if abs >= cols[x] {
                cols[x] = abs + 1;
                inc = true;
            }
            if inc && cell >= 0 {
                grid[y][x] = cell * -1;
                count += 1;
            }
        }
    }
    Ok(count)
}

fn walk_backwards(grid: &mut Vec<Vec<isize>>) -> Result<usize> {
    if grid.is_empty() {
        bail!("cannot have zero rows in grid");
    }
    if grid[0].is_empty() {
        bail!("cannot have zero columns in grid");
    }

    let height = grid.len();
    let width = grid.len();
    let mut rows = vec![0; height];
    let mut cols = vec![0; width];
    let mut count = 0;
    for y in (0..height).rev() {
        if grid[y].len() != width {
            bail!("all rows must be the same width");
        }
        for x in (0..width).rev() {
            let cell = grid[y][x];
            let abs = cell.abs();
            let mut inc = false;
            if abs >= rows[y] {
                rows[y] = abs + 1;
                inc = true;
            }
            if abs >= cols[x] {
                cols[x] = abs + 1;
                inc = true;
            }
            if inc && cell >= 0 {
                grid[y][x] = cell * -1;
                count += 1;
            }
        }
    }
    Ok(count)
}

fn init_grid(lines: Vec<String>) -> Result<Vec<Vec<isize>>> {
    lines
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|ch| {
                    let num = ch.to_digit(10).context("invalid digit")?;
                    Ok(num as isize)
                })
                .collect::<Result<Vec<isize>>>()
        })
        .collect::<Result<Vec<Vec<isize>>>>()
}

fn main() -> Result<()> {
    let (lines, _variant) = get_input_file_lines_with_variant()?;
    let mut grid = init_grid(lines)?;
    let visible = walk_forwards(&mut grid)? + walk_backwards(&mut grid)?;

    println!("The number of visible trees is: {}", visible);
    Ok(())
}
