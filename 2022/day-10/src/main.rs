use anyhow::{bail, Result};
use common::{get_input_file_lines_with_variant, Variant};

// Which cycle to start sampling at.
const START: usize = 20;

// Which cycle to stop the processor at.
const STOP: usize = 240;

// The gap between samples (and also, conveniently, the width of the screen in pixels).
const GAP: usize = 40;

struct Processor<const START: usize, const STOP: usize, const GAP: usize> {
    cycles: usize,
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
            cycles: 0,
            signal: 1,
            samples: vec![],
        }
    }

    fn noop<F>(&mut self, f: &mut F) -> Result<Continue>
    where
        F: FnMut(usize, isize) -> Result<()>,
    {
        self.increment_then_sample(1, f)
    }

    fn add<F>(&mut self, diff: isize, f: &mut F) -> Result<Continue>
    where
        F: FnMut(usize, isize) -> Result<()>,
    {
        let keep_going = self.increment_then_sample(2, f);
        self.signal += diff;
        keep_going
    }

    fn increment_then_sample<F>(&mut self, inc: usize, f: &mut F) -> Result<Continue>
    where
        F: FnMut(usize, isize) -> Result<()>,
    {
        for _ in 0..inc {
            self.cycles += 1;
            f(self.cycles, self.signal)?;
        }
        let want_samples = (self.cycles + START) / GAP;
        if self.samples.len() < want_samples {
            self.samples.push(self.signal);
        }
        Ok(self.keep_going())
    }

    fn keep_going(&self) -> Continue {
        if self.cycles > STOP {
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

fn render<const TOTAL: usize, const WIDTH: usize>(pixels: [bool; TOTAL]) {
    let mut row = String::new();
    for (index, is_set) in pixels.iter().enumerate() {
        if index % WIDTH == 0 {
            println!("{}", row);
            row = String::new();
        }
        row.push(match is_set {
            true => '#',
            false => '.',
        });
    }
    println!("{}", row);
}

fn main() -> Result<()> {
    let (lines, variant) = get_input_file_lines_with_variant()?;

    let mut processor = Processor::<START, STOP, GAP>::new();
    match variant {
        Variant::A => {
            let mut handler = |_, _| Ok(());
            for line in lines {
                let keep_going = match parse_line(line.as_str())? {
                    Instruction::Noop => processor.noop(&mut handler),
                    Instruction::Add(diff) => processor.add(diff, &mut handler),
                };
                if let Continue::No = keep_going? {
                    break;
                }
            }

            println!(
                "The combination of the strengths at the sample points is: {}",
                processor
                    .samples
                    .iter()
                    .enumerate()
                    .fold(0isize, |acc, (index, signal)| {
                        acc + ((START as isize + (index * GAP) as isize) * signal)
                    })
            );
        }
        Variant::B => {
            let mut pixels = [false; STOP];
            let mut handler = |cycle, signal: isize| {
                let pos: usize = cycle % GAP;
                let offset: isize = pos.try_into()?;
                if offset >= signal && offset <= signal + 2 {
                    pixels[cycle - 1] = true
                } else {
                    pixels[cycle - 1] = false
                }
                Ok(())
            };
            for line in lines {
                let keep_going = match parse_line(line.as_str())? {
                    Instruction::Noop => processor.noop(&mut handler),
                    Instruction::Add(diff) => processor.add(diff, &mut handler),
                };
                if let Continue::No = keep_going? {
                    break;
                }
            }
            render::<STOP, GAP>(pixels);
        }
    };
    Ok(())
}
