use anyhow::{bail, Result};
use common::get_input_file_lines_with_variant;

const START_A: usize = 20;
const STOP_A: usize = 220;
const GAP_A: usize = 40;

struct Processor<const START: usize, const STOP: usize, const GAP: usize> {
    completed_cycles: usize,
    signal: isize,
    samples: Vec<isize>,
}

enum Continue {
    Yes,
    No,
}

enum Instruction {
    Noop,
    Add(isize),
}

impl<const START: usize, const STOP: usize, const GAP: usize> Processor<START, STOP, GAP> {
    fn new() -> Processor<START, STOP, GAP> {
        Processor::<START, STOP, GAP> {
            completed_cycles: 0,
            signal: 1,
            samples: vec![],
        }
    }

    fn noop(&mut self) -> Continue {
        self.increment_then_sample(1)
    }

    fn add(&mut self, diff: isize) -> Continue {
        let keep_going = self.increment_then_sample(2);
        self.signal += diff;
        keep_going
    }

    fn increment_then_sample(&mut self, inc: usize) -> Continue {
        self.completed_cycles += inc;
        let want_samples = (self.completed_cycles + START) / GAP;
        if self.samples.len() < want_samples {
            self.samples.push(self.signal);
        }
        if self.completed_cycles > STOP {
            return Continue::No;
        }
        Continue::Yes
    }
}

fn parse_line(line: &str) -> Result<Instruction> {
    let mut parts = line.split_whitespace();
    let command = parts.next();
    match command {
        None => bail!("parsing: empty line"),
        Some(command) => match command {
            "noop" => Ok(Instruction::Noop),
            "addx" => Ok(Instruction::Add(match parts.next() {
                None => bail!("parsing: no argument for add command"),
                Some(operand) => operand.parse()?,
            })),
            _ => bail!("parsing: unrecognized command"),
        },
    }
}

fn main() -> Result<()> {
    let (lines, _variant) = get_input_file_lines_with_variant()?;

    let mut processor = Processor::<START_A, STOP_A, GAP_A>::new();
    for line in lines {
        let keep_going = match parse_line(line.as_str())? {
            Instruction::Noop => processor.noop(),
            Instruction::Add(diff) => processor.add(diff),
        };
        if let Continue::No = keep_going {
            break;
        }
    }

    let sum = processor
        .samples
        .iter()
        .enumerate()
        .fold(0isize, |acc, (index, signal)| {
            acc + ((START_A as isize + (index * GAP_A) as isize) * signal)
        });
    println!("The sum of the strengths at the sample points is: {}", sum);
    Ok(())
}
