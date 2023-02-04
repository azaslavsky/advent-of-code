use anyhow::{bail, Context, Error, Result};
use common::get_input_file_lines_with_variant;

#[derive(Debug)]
struct Point {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct BoundingBox {
    top_left: Point,
    bottom_right: Point,
}

#[derive(Debug)]
struct Multiline {
    points: Vec<Point>,
}

/// A tri-state to store the state of a given grid cell. We use this instead of a boolean for
/// rendering purposes.
#[derive(Clone, Copy, Debug, Default)]
enum CellState {
    Rock,
    Sand,
    #[default]
    Empty,
}

#[derive(Debug)]
struct Grid {
    source: Point,
    width: usize,
    cells: Vec<CellState>,
}

/// The point from which sand flows, as defined by the prompt.
const DEFAULT_SAND_SOURCE: Point = Point { row: 0, col: 500 };

/// Parse the input into a set of |Multiline|s, expanding of the |BoundingBox| as we go.
fn parse(lines: Vec<String>) -> Result<(Vec<Multiline>, BoundingBox)> {
    let mut bounds = BoundingBox {
        top_left: DEFAULT_SAND_SOURCE,
        bottom_right: DEFAULT_SAND_SOURCE,
    };
    let multilines = lines
        .into_iter()
        .try_fold(Vec::<Multiline>::new(), |mut outer, line| {
            outer.push(Multiline {
                points: line
                    .split("->")
                    .try_fold(Vec::<Point>::new(), |mut inner, segment| {
                        // Parse out the |row| and |col| values for this point.
                        let mut nums = segment.trim().split(',');
                        let col = nums
                            .next()
                            .context("parsing: col coordinate missing")?
                            .parse::<usize>()
                            .context("parsing: row coordinate invalid")?;
                        let row = nums
                            .next()
                            .context("parsing: row coordinate missing")?
                            .parse::<usize>()
                            .context("parsing: row coordinate invalid")?;
                        if let Some(_) = nums.next() {
                            bail!("parsing: third coordinate in coordinate pair disallowed");
                        }

                        // Update the bounds if we have exceeded them in any direction.
                        if row < bounds.top_left.row {
                            bounds.top_left.row = row;
                        }
                        if row > bounds.bottom_right.row {
                            bounds.bottom_right.row = row;
                        }
                        if col < bounds.top_left.col {
                            bounds.top_left.col = col;
                        }
                        if col > bounds.bottom_right.col {
                            bounds.bottom_right.col = col;
                        }

                        inner.push(Point { row, col });
                        Ok::<_, Error>(inner)
                    })?,
            });
            Ok::<_, Error>(outer)
        })?;

    Ok((multilines, bounds))
}

/// Create the starting state for the grid by filling in each point touched by a line.
fn init_grid(multilines: Vec<Multiline>, bounds: BoundingBox) -> Result<Grid> {
    let height = (bounds.bottom_right.row + 1) - bounds.top_left.row;
    let width = (bounds.bottom_right.col + 1) - bounds.top_left.col;
    let row_offset = bounds.top_left.row;
    let col_offset = bounds.top_left.col;
    let mut grid = Grid {
        source: Point {
            row: DEFAULT_SAND_SOURCE.row - bounds.top_left.row,
            col: DEFAULT_SAND_SOURCE.col - bounds.top_left.col,
        },
        width,
        cells: vec![CellState::default(); height * width],
    };

    multilines.into_iter().try_for_each(|multiline| {
        if multiline.points.len() <= 1 {
            bail!("init: multiline must have at least two points");
        }
        let mut row = multiline.points[0].row;
        let mut col = multiline.points[0].col;
        multiline.points.into_iter().skip(1).try_for_each(|point| {
            if point.row != row && point.col != col {
                bail!("init: multiline segment not straight");
            }
            if point.col == col {
                // Vertical line.
                while point.row != row {
                    let cell = (width * (row - row_offset)) + (col - col_offset);
                    grid.cells[cell] = CellState::Rock;
                    if row > point.row {
                        row -= 1;
                    } else {
                        row += 1
                    }
                }
            } else {
                // Horizontal line.
                while point.col != col {
                    let cell = (width * (row - row_offset)) + (col - col_offset);
                    grid.cells[cell] = CellState::Rock;
                    if col > point.col {
                        col -= 1;
                    } else {
                        col += 1
                    }
                }
            }
            
            // Make sure to initialize the last |Point| in the |Multiline|.
            let cell = (width * (row - row_offset)) + (col - col_offset);
            grid.cells[cell] = CellState::Rock;
            Ok(())
        })?;
        Ok(())
    })?;
    Ok(grid)
}

/// Iteratively add grains of sand until the pile is cannot accept any more. Record the number of
/// grains needed to achieve this end state.
fn simulate_until_full(grid: &mut Grid) -> Result<usize> {
    let cell_count = grid.cells.len();
    let width = grid.width;
    let cells = &mut grid.cells;
    let source = (grid.source.row * width) + grid.source.col;
    let mut pos = source;
    let mut count = 0;
    loop {
        // Try going down...
        let down_pos = pos + width;
        if down_pos >= cell_count {
            // We've fallen off the bottom, the simulation is done.
            return Ok(count);
        }
        if let CellState::Empty = cells[down_pos] {
            pos = down_pos;
            continue;
        }
        
        // ...then down and left...
        if down_pos % width == 0 {
            // We've reached the left edge of the grid, the simulation is done.
            return Ok(count);
        }
        let left_and_down_pos = down_pos - 1;
        if let CellState::Empty = cells[left_and_down_pos] {
            pos = left_and_down_pos;
            continue;
        }

        // ...then, finally, down and right.
        let right_and_down_pos = down_pos + 1;
        if right_and_down_pos % width == 0 {
            // We've reached the right edge of the grid, the simulation is done.
            return Ok(count);
        }
        if let CellState::Empty = cells[right_and_down_pos] {
            pos = right_and_down_pos;
            continue;
        }

        // If we're stuck, place the grain, then loop again.
        cells[pos] = CellState::Sand;
        count += 1;
        pos = source;
    }
}

/// Iteratively add grains of sand until the pile is cannot accept any more.
// fn simulate_until_full(grid: Grid) -> Result<usize> {}

/// Print the grid for debugging purposes.
fn render(grid: &Grid) -> Result<()> {
    if grid.cells.len() == 0 {
        bail!("rendering: empty grid");
    }

    // Render the sand entry point
    let mut printing = String::new();
    for x in 0..grid.width {
        printing.push(if grid.source.col == x { '|' } else { '_' });
    }
    println!("{}", printing);

    printing = String::new();
    grid.cells.iter().enumerate().for_each(|(index, cell)| {
        printing.push(match cell {
            CellState::Rock => '#',
            CellState::Sand => 'o',
            CellState::Empty => '.',
        });
        if (index + 1) % grid.width == 0 {
            println!("{}", printing);
            printing = String::new();
        }
    });
    Ok(())
}

fn main() -> Result<()> {
    let (lines, _variant) = get_input_file_lines_with_variant()?;
    let (multilines, bounds) = parse(lines)?;
    // println!("\n{:#?}", multilines);
    // println!("\n{:#?}", bounds);

    let mut grid = init_grid(multilines, bounds)?;
    // println!("\n{:#?}", grid);
    println!("\nThe initial grid:\n");
    render(&grid)?;

    let grains = simulate_until_full(&mut grid)?;
    println!("\nThe filled grid:\n");
    render(&grid)?;

    println!("\nThe number of grains needed was: {}", grains);
    Ok(())
}
