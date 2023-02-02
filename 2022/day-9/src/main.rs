use anyhow::{bail, Context, Result};
use common::{get_input_file_lines_with_variant, Variant};
use std::collections::HashSet;

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

struct Movement {
    direction: Direction,
    magnitude: i16,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Position {
    x: i16,
    y: i16,
}

#[derive(Debug)]
struct State<const LENGTH: usize> {
    knots: [Position; LENGTH],
}

impl<const LENGTH: usize> State<LENGTH> {
    fn new() -> State<LENGTH> {
        State {
            knots: [Position::default(); LENGTH],
        }
    }
}

fn move_head<'a>(head: &'a mut Position, movement: &Movement) -> &'a mut Position {
    match movement.direction {
        Direction::Up => head.y += movement.magnitude,
        Direction::Down => head.y -= movement.magnitude,
        Direction::Right => head.x += movement.magnitude,
        Direction::Left => head.x -= movement.magnitude,
    };
    head
}

fn move_knot(ends: &mut [Position], visited: &mut Option<&mut HashSet<Position>>) -> bool {
    let mut moved = false;
    loop {
        let x_diff = ends[0].x - ends[1].x;
        let y_diff = ends[0].y - ends[1].y;
        if x_diff.abs() <= 1 && y_diff.abs() <= 1 {
            // The tail has moved to its final position.
            return moved;
        }

        moved = true;
        if x_diff != 0 {
            // The tail needs to move horizontally.
            if x_diff > 0 {
                ends[1].x += 1;
            } else {
                ends[1].x -= 1;
            }
        }
        if y_diff != 0 {
            // The tail needs to move vertically.
            if y_diff > 0 {
                ends[1].y += 1;
            } else {
                ends[1].y -= 1;
            }
        }
        if let Some(ref mut v) = visited {
            v.insert(ends[1].clone());
        }
    }
}

fn parse_line(line: &str) -> Result<Movement> {
    let mut parts = line.split_whitespace();
    let out = Movement {
        direction: match parts.next().context("parsing: missing direction")? {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "R" => Direction::Right,
            "L" => Direction::Left,
            _ => bail!("parsing: unknown direction"),
        },
        magnitude: parts
            .next()
            .context("parsing: missing direction")?
            .parse()?,
    };
    if let Some(_) = parts.next() {
        bail!("parsing: too many parts");
    }
    return Ok(out);
}

const ROPE_LENGTH_B: usize = 10;

fn main() -> Result<()> {
    let (lines, variant) = get_input_file_lines_with_variant()?;
    let mut parsed_lines = lines.into_iter().map(|line| parse_line(line.as_str()));

    let mut visited = HashSet::<Position>::new();
    match variant {
        Variant::A => {
            let mut state = State::<2>::new();
            visited.insert(state.knots[0].clone());
            parsed_lines.try_for_each(|input| {
                match input {
                    Err(err) => bail!(err),
                    Ok(movement) => {
                        move_head(&mut state.knots[0], &movement);
                        move_knot(&mut state.knots[..], &mut Some(&mut visited));
                    }
                }
                Ok(())
            })?;
        }
        Variant::B => {
            let mut state = State::<ROPE_LENGTH_B>::new();
            visited.insert(state.knots[0].clone());
            parsed_lines.try_for_each(|input| {
                match input {
                    Err(err) => bail!(err),
                    Ok(movement) => {
                        // Move the head.
                        move_head(&mut state.knots[0], &movement);

                        // Move intermediate knots.
                        for i in 0..(ROPE_LENGTH_B - 2) {
                            if !move_knot(&mut state.knots[i..], &mut None) {
                                break;
                            }
                        }

                        // Move tail knot.
                        move_knot(
                            &mut state.knots[(ROPE_LENGTH_B - 2)..ROPE_LENGTH_B],
                            &mut Some(&mut visited),
                        );
                        // println!("State: {:#?} \n\n", state);
                    }
                }
                Ok(())
            })?;
        }
    }

    println!("Unique cells visited by tail: {}", visited.len());
    Ok(())
}
