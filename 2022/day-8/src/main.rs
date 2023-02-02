use anyhow::{bail, Context, Result};
use common::{get_input_file_lines_with_variant, Variant};
use std::collections::BTreeMap;

fn count_forwards(grid: &mut Vec<Vec<isize>>) -> Result<usize> {
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

fn count_backwards(grid: &mut Vec<Vec<isize>>) -> Result<usize> {
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

#[derive(Debug)]
struct Distances {
    left: usize,
    right: usize,
    up: usize,
    down: usize,
}

type DistanceGrid = Vec<Vec<Distances>>;
type ViewBlockers = BTreeMap<u8, usize>;

// We can compute the `left` and `up` distances by walking forwards.
fn score_forwards(grid: &mut Vec<Vec<isize>>) -> Result<DistanceGrid> {
    if grid.is_empty() {
        bail!("cannot have zero rows in grid");
    }
    if grid[0].is_empty() {
        bail!("cannot have zero columns in grid");
    }

    let mut distances = DistanceGrid::new();
    let height = grid.len();
    let width = grid.len();
    let mut rows = vec![ViewBlockers::new(); height];
    let mut cols = vec![ViewBlockers::new(); width];
    for y in 0..height {
        distances.push(Vec::new());
        if grid[y].len() != width {
            bail!("all rows must be the same width");
        }
        for x in 0..width {
            let cell = grid[y][x] as u8;
            let row_blockers = &mut rows[y];
            let col_blockers = &mut cols[x];

            // Score looking left.
            *row_blockers = row_blockers.split_off(&cell);
            distances[y].push(Distances {
                left: match row_blockers.range(cell..).next() {
                    Some((_, index)) => x - index,
                    None => x,
                },
                right: 0,
                up: 0,
                down: 0,
            });
            row_blockers.insert(cell, x);

            // Score looking up.
            *col_blockers = col_blockers.split_off(&cell);
            distances[y][x].up = match col_blockers.range(cell..).next() {
                Some((_, index)) => y - index,
                None => y,
            };
            col_blockers.insert(cell, y);
        }
    }
    Ok(distances)
}

// We can compute the `right` and `down` distances by walking forwards.
fn score_backwards(grid: &mut Vec<Vec<isize>>, mut distances: DistanceGrid) -> Result<usize> {
    if grid.is_empty() {
        bail!("cannot have zero rows in grid");
    }
    if grid[0].is_empty() {
        bail!("cannot have zero columns in grid");
    }

    let height = grid.len();
    let width = grid.len();
    let mut rows = vec![ViewBlockers::new(); height];
    let mut cols = vec![ViewBlockers::new(); width];
    let mut max = 0;
    for y in (0..height).rev() {
        if grid[y].len() != width {
            bail!("all rows must be the same width");
        }
        for x in (0..width).rev() {
            let cell = grid[y][x] as u8;
            let mut dist = &mut distances[y][x];
            let row_blockers = &mut rows[y];
            let col_blockers = &mut cols[x];

            // Score looking right.
            *row_blockers = row_blockers.split_off(&cell);
            dist.right = match row_blockers.range(cell..).next() {
                Some((_, index)) => index - x,
                None => (width - 1) - x,
            };
            row_blockers.insert(cell, x);

            // Score looking up.
            *col_blockers = col_blockers.split_off(&cell);
            dist.down = match col_blockers.range(cell..).next() {
                Some((_, index)) => index - y,
                None => (height - 1) - y,
            };
            col_blockers.insert(cell, y);

            // Calculate total score.
            let score = dist.left * dist.right * dist.up * dist.down;
            if score > max {
                max = score;
            }
        }
    }
    Ok(max)
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
    let (lines, variant) = get_input_file_lines_with_variant()?;
    let mut grid = init_grid(lines)?;

    match variant {
        Variant::A => println!(
            "The number of visible trees is: {}",
            count_forwards(&mut grid)? + count_backwards(&mut grid)?
        ),
        Variant::B => {
            let distances = score_forwards(&mut grid)?;
            println!(
                "The most scenic tree's scenic score is: {}",
                score_backwards(&mut grid, distances)?
            )
        }
    }
    Ok(())
}
