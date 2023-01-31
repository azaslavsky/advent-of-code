use anyhow::{bail, Result};
use common::{get_input_file_lines_with_variant, Variant};
use std::collections::HashSet;

const MARKER_SIZE_A: usize = 4;
const MARKER_SIZE_B: usize = 14;

fn main() -> Result<()> {
    let (lines, variant) = get_input_file_lines_with_variant()?;
    if lines.len() != 1 {
        bail!("input must be single line");
    }

    let marker_size = match variant {
        Variant::A => MARKER_SIZE_A,
        Variant::B => MARKER_SIZE_B,
    };

    lines[0]
        .chars()
        .collect::<Vec<char>>()
        .as_slice()
        .windows(marker_size)
        .enumerate()
        .take_while(|(index, window)| {
            let mut seen = HashSet::<char>::new();
            for ch in *window {
                if !seen.insert(*ch) {
                    return true;
                }
            }
            println!("The first non-start-of-packer character is at: {}", index + marker_size);
            return false;
        })
        .for_each(drop);
    Ok(())
}
