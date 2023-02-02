use anyhow::{bail, Context, Result};
use common::get_input_file_lines_with_variant;
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

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
struct Position {
    x: i16,
    y: i16,
}

#[derive(Default)]
struct State {
    head: Position,
    tail: Position,
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

fn move_tail(head: &mut Position, tail: &mut Position, visited: &mut HashSet<Position>) {
    loop {
        let x_diff = head.x - tail.x;
        let y_diff = head.y - tail.y;
        if x_diff.abs() <= 1 && y_diff.abs() <= 1 {
            // The tail has moved to its final position.
            return;
        }
        if x_diff != 0 {
            // The tail needs to move horizontally.
            if x_diff > 0 {
                tail.x += 1;
            } else {
                tail.x -= 1;
            }
        }
        if y_diff != 0 {
            // The tail needs to move vertically.
            if y_diff > 0 {
                tail.y += 1;
            } else {
                tail.y -= 1;
            }
        }
        visited.insert(tail.clone());
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

fn main() -> Result<()> {
    let (lines, _variant) = get_input_file_lines_with_variant()?;

    let mut state = State::default();
    let mut visited = HashSet::<Position>::new();
    visited.insert(state.tail.clone());
    lines
        .into_iter()
        .map(|line| parse_line(line.as_str()))
        .try_for_each(|input| {
            match input {
                Err(err) => bail!(err),
                Ok(movement) => move_tail(
                    move_head(&mut state.head, &movement),
                    &mut state.tail,
                    &mut visited,
                ),
            }
            Ok(())
        })?;

    println!("Unique cells visited: {}", visited.len());
    Ok(())
}
