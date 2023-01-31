use anyhow::{bail, Context, Result};
use std::fs::File;

// Use the first argument passed to this binary as the file path to a file containing input data.
pub fn open_input_file() -> Result<File> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        bail!("must specify exactly one argument, a file to load as input");
    }
    let file_path = &args[1];
    File::open(file_path).context("could not find input file")
}

pub enum Variant {
    A,
    B,
}

// Use the first argument passed to this binary as the file path to a file containing input data,
// and the second to bifurcate depending on which of the day's prompts (a or b) is being solved.
pub fn open_input_file_with_variant() -> Result<(File, Variant)> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        bail!("Must specify exactly two arguments, a file to load as input, and a variant (a/b)");
    }
    let file_path = &args[1];
    let variant = match args[2].as_str() {
        "a" | "A" => Variant::A,
        "b" | "B" => Variant::B,
        _ => bail!("incorrect variant")
    };
    Ok((File::open(file_path).context("could not find input file")?, variant))
}