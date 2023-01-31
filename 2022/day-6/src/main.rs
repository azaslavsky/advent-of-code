use anyhow::{bail, Result};
use common::get_input_file_lines_with_variant;
use std::collections::HashSet;

const MARKER_SIZE: usize = 4;

fn main() -> Result<()> {
    let (lines, _variant) = get_input_file_lines_with_variant()?;
    if lines.len() != 1 {
        bail!("input must be single line");
    }
    lines[0]
        .chars()
        .collect::<Vec<char>>()
        .as_slice()
        .windows(MARKER_SIZE)
        .enumerate()
        .take_while(|(index, window)| {
            let mut seen = HashSet::<char>::new();
            for ch in *window {
                if !seen.insert(*ch) {
                    return true;
                }
            }
            println!("The first non-start-of-packer character is at: {}", index + MARKER_SIZE);
            return false;
        })
        .for_each(drop);
    Ok(())
}
