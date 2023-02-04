use anyhow::{bail, Context, Error, Result};
use common::get_input_file_lines_with_variant;
use std::collections::{HashSet, VecDeque};

type Grid = Vec<Vec<i8>>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    row: usize,
    col: usize,
}

struct History {
    pos: Position,
    steps: usize,
}

#[derive(Debug)]
struct Description {
    grid: Grid,
    from: Position,
    goal: Position,
}

fn parse(lines: Vec<String>) -> Result<Description> {
    let mut from = None;
    let mut goal = None;
    let grid = lines
        .into_iter()
        .enumerate()
        .try_fold(Grid::new(), |mut acc, (row, line)| {
            acc.push(line.chars().enumerate().try_fold(
                Vec::<i8>::new(),
                |mut acc, (col, ch)| {
                    acc.push(match ch {
                        'S' => match from {
                            Some(_) => bail!("parsing: multiple S cells"),
                            None => {
                                from = Some(Position { row, col });
                                Ok::<_, Error>(0) // aka "a"
                            }
                        },
                        'E' => match goal {
                            Some(_) => bail!("parsing: multiple E cells"),
                            None => {
                                goal = Some(Position { row, col });
                                Ok::<_, Error>(25) // aka "z"
                            }
                        },
                        _ => {
                            // Must be a lower-case letter - we can use the ASCII table to verify.
                            let ascii = ch as i8;
                            if ascii < 97 || ascii > 122 {
                                bail!("parsing: unknown character (not lowercase letter, S, or E)")
                            }
                            Ok(ascii - 97)
                        }
                    }?);
                    Ok(acc)
                },
            )?);
            Ok::<Grid, Error>(acc)
        })?;

    Ok(Description {
        from: from.context("parsing: no S cell seen")?,
        goal: goal.context("parsing: no E cell seen")?,
        grid,
    })
}

fn search(from: &Position, goal: &Position, grid: &Grid) -> Result<usize> {
    let mut queue = VecDeque::<History>::new();
    let mut visited = HashSet::<Position>::new();
    queue.push_front(History {
        pos: *from,
        steps: 0,
    });

    loop {
        match queue.pop_back() {
            None => bail!("end never reached"),
            Some(hist) => {
                // Check if we've already visited this cell - repeat visits always make for a longer
                // than necessary, so exit early.
                let pos = hist.pos;
                if visited.contains(&pos) {
                    continue;
                }

                // Exit as soon as we reach the goal.
                let current = grid[pos.row][pos.col];
                if goal == &pos {
                    return Ok(hist.steps)
                }

                // Mark this cell as visited, so we don't do redundant work.
                visited.insert(pos);

                // Try each of the cardinal directions, and add that direction to the queue if
                // possible. First, try going up.
                if pos.row > 0 && grid[pos.row - 1][pos.col] - current <= 1 {
                    queue.push_front(History {
                        pos: Position {
                            row: pos.row - 1,
                            col: pos.col,
                        },
                        steps: hist.steps + 1,
                    });
                }

                // Next, down.
                if pos.row < grid.len() - 1 && grid[pos.row + 1][pos.col] - current <= 1 {
                    queue.push_front(History {
                        pos: Position {
                            row: pos.row + 1,
                            col: pos.col,
                        },
                        steps: hist.steps + 1,
                    });
                }

                // Then left.
                if pos.col > 0 && grid[pos.row][pos.col - 1] - current <= 1 {
                    queue.push_front(History {
                        pos: Position {
                            row: pos.row,
                            col: pos.col - 1,
                        },
                        steps: hist.steps + 1,
                    });
                }

                // Finally, right.
                if pos.col < grid[0].len() - 1 && grid[pos.row][pos.col + 1] - current <= 1 {
                    queue.push_front(History {
                        pos: Position {
                            row: pos.row,
                            col: pos.col + 1,
                        },
                        steps: hist.steps + 1,
                    });
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let (lines, _variant) = get_input_file_lines_with_variant()?;
    let desc = parse(lines)?;

    println!(
        "Minimum number of steps is: {}",
        search(&desc.from, &desc.goal, &desc.grid)?
    );
    Ok(())
}
