use anyhow::{bail, Result};
use common::{get_input_file_lines_with_variant};
use std::collections::VecDeque;

type Column = VecDeque<char>;
type Columns = Vec<Column>;
type Moves = Vec<Move>;

// Specifies |num| moves from |src| to |dest|.
struct Move {
    num: usize,
    src: usize,
    dest: usize,
}

fn parse_crate_line(cols: &mut Columns, line: &str) -> Result<()> {
    for (i, ch) in line.chars().enumerate() {
        match ch {
            '[' | ']' | ' ' => continue,
            _ => {
                // Each column in the input is 4 characters wide.
                let col = i/4;
                if !ch.is_alphabetic() {
                    bail!("parsing: invalid crate identifier")
                }
                if cols.len() <= col {
                    cols.append(&mut vec![Column::new(); col + 1 - cols.len()]);
                }
                cols[col].push_front(ch);
            }
        }
    }
    Ok(())
}

fn init_columns(lines: Vec<String>) -> Result<Columns> {
    let mut cols = Columns::new();

    // Skip the line enumerating the crates, as this doesn't really help us.
    for line in lines.into_iter().rev().skip(1) {
        parse_crate_line(&mut cols, line.as_str())?;
    }
    Ok(cols)
}

fn parse_move_line(line: &str) -> Result<Move> {
    let parsed = line
        .split_whitespace()
        .filter_map(|word| word.parse::<usize>().ok())
        .collect::<Vec<_>>();
    if parsed.len() != 3 {
        bail!("parsing: move line formatted incorrectly");
    }

    // Note that the inputs are 1-indexed (to reflect the input), but the |Columns| storage
    // array is 0-indexed, so a decrement needs to happen to adjust.
    Ok(Move {
        num: parsed[0],
        src: parsed[1] - 1,
        dest: parsed[2] - 1,
    })
}

fn init_moves(lines: Vec<String>) -> Result<Moves> {
    let mut moves = Moves::new();

    // Skip the empty newline at the start.
    for line in lines.into_iter().skip(1) {
        moves.push(parse_move_line(line.as_str())?)
    }
    Ok(moves)
}

fn apply_moves(mut cols: Columns, moves: Moves) -> Result<Columns> {
    let num_cols = cols.len();
    for mv in moves {
        if mv.src >= num_cols {
            bail!("Tried to move from an unknown stack");
        }
        if mv.dest >= num_cols {
            bail!("Tried to move to an unknown stack");
        }
        
        for _ in 0..mv.num {
            match cols[mv.src].pop_front() {
                Some(ch) => cols[mv.dest].push_front(ch),
                None => bail!("Tried to move from an empty stack"),
            }
        }
    }
    Ok(cols)
}

fn print_top_crates(cols: &Columns) -> Result<String> {
    let mut out = String::new();
    for c in cols {
        match c.front() {
            Some(ch) => out.push(*ch),
            None => bail!("cannot print empty stack"),
        }
    }
    Ok(out)
}

fn main() -> Result<()> {
    let (lines, _variant) = get_input_file_lines_with_variant()?;
    let mut partition = true;
    let (cols, moves) = lines.into_iter().partition(|line| {
        if line.is_empty() {
            partition = false;
        }
        partition
    });

    let restacked = apply_moves(init_columns(cols)?, init_moves(moves)?)?;
    println!("The top crates on each column are: {}", print_top_crates(&restacked)?);
    Ok(())
}
