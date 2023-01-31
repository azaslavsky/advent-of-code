use anyhow::{bail, Context, Result};
use std::fs::File;

// Use the first argument passed to this binary as the file path to a file containing input data.
pub fn open_input_file() -> Result<File> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        bail!("Must specify exactly one argument, a file to load as input");
    }
    let file_path = &args[1];
    File::open(file_path).context("could not find input file")
}
