use anyhow::{bail, Result};
use std::fs::File;
use std::io::{self, BufRead};

use anyhow::Context;

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        bail!("Must specify exactly one argument, a file to load as input");
    }
    let file_path = &args[1];

    let input = File::open(file_path).context("could not find input file")?;

    let mut max = 0;
    io::BufReader::new(input)
        .lines()
        .try_fold(0u32, |acc, line| {
            match line {
                Ok(line) => {
                    if !line.is_empty() {
                        let val = line.parse::<u32>().context("invalid input line")?;
                        return Ok::<u32, anyhow::Error>(acc + val);
                    }
                }
                _ => {}
            }

            if acc > max {
                max = acc
            }
            Ok(0)
        })?;

    println!("The maximum number of calories carried by a single elf is: {}", max);
    Ok(())
}
