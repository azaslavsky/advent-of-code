use anyhow::{bail, Result};
use common::get_input_file_lines_with_variant;

fn main() -> Result<()> {
    let (lines, _variant) = get_input_file_lines_with_variant()?;
    Ok(())
}
