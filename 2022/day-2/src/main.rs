use anyhow::{bail, Context, Result};
use common::open_input_file;
use std::io::{self, BufRead};

enum Throw {
    Rock,
    Paper,
    Scissors,
}

impl Throw {
    fn from_string(code: &str) -> Result<Throw> {
        match code {
            "A" | "X" => Ok(Throw::Rock),
            "B" | "Y" => Ok(Throw::Paper),
            "C" | "Z" => Ok(Throw::Scissors),
            _ => bail!("invalid input code"),
        }
    }

    fn versus(&self, against: Throw) -> u32 {
        match self {
            Throw::Rock => match against {
                Throw::Rock => 4,
                Throw::Paper => 1,
                Throw::Scissors => 7,
            },
            Throw::Paper => match against {
                Throw::Rock => 8,
                Throw::Paper => 5,
                Throw::Scissors => 2,
            },
            Throw::Scissors => match against {
                Throw::Rock => 3,
                Throw::Paper => 9,
                Throw::Scissors => 6,
            },
        }
    }
}

fn main() -> Result<()> {
    let sum =
        io::BufReader::new(open_input_file()?)
            .lines()
            .try_fold(0u32, |acc, line| match line {
                Ok(line) => {
                    if !line.is_empty() {
                        let mut throws = line.as_str().split_whitespace();
                        let them =
                            Throw::from_string(throws.next().context("missing their throw")?)?;
                        let me = Throw::from_string(throws.next().context("missing my throw")?)?;
                        return Ok(acc + me.versus(them));
                    }
                    Ok(acc)
                }
                Err(err) => bail!(err),
            })?;

    println!("My cumulative score is: {}", sum);
    Ok(())
}
