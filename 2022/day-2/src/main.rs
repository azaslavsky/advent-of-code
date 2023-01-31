use anyhow::{bail, Context, Result};
use common::{open_input_file_with_variant, Variant};
use std::io::{self, BufRead};

enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    fn from_string(code: &str) -> Result<Outcome> {
        match code {
            "Z" => Ok(Outcome::Win),
            "Y" => Ok(Outcome::Draw),
            "X" => Ok(Outcome::Lose),
            _ => bail!("invalid input code"),
        }
    }
}

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

    fn cheat(&self, want_outcome: Outcome) -> u32 {
        match self {
            // To "cheat" against rock, we need to throw paper to win, and scissors to lose.
            Throw::Rock => match want_outcome {
                Outcome::Draw => 4,
                Outcome::Win => 8,
                Outcome::Lose => 3,
            },
            // To "cheat" against paper, we need to throw scissors to win, and rock to lose.
            Throw::Paper => match want_outcome {
                Outcome::Draw => 5,
                Outcome::Win => 9,
                Outcome::Lose => 1,
            },
            // To "cheat" against scissors, we need to throw rock to win, and paper to lose.
            Throw::Scissors => match want_outcome {
                Outcome::Draw => 6,
                Outcome::Win => 7,
                Outcome::Lose => 2,
            },
        }
    }
}

fn main() -> Result<()> {
    let (input, variant) = open_input_file_with_variant()?;
    let sum = io::BufReader::new(input)
        .lines()
        .try_fold(0u32, |acc, line| match line {
            Ok(line) => {
                if !line.is_empty() {
                    let mut strategy = line.as_str().split_whitespace();
                    let them = Throw::from_string(strategy.next().context("missing their throw")?)?;
                    return match variant {
                        Variant::A => {
                            let me =
                                Throw::from_string(strategy.next().context("missing my throw")?)?;
                            Ok(acc + me.versus(them))
                        }
                        Variant::B => {
                            let want_outcome = Outcome::from_string(
                                strategy.next().context("missing desired outcome")?,
                            )?;
                            Ok(acc + them.cheat(want_outcome))
                        }
                    };
                }
                Ok(acc)
            }
            Err(err) => bail!(err),
        })?;

    println!("My cumulative score is: {}", sum);
    Ok(())
}
