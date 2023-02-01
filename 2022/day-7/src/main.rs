use anyhow::{bail, Context, Result};
use common::{get_input_file_lines_with_variant, Variant};
use std::collections::{BTreeSet, HashMap};

const LIMIT: usize = 100_000;
const DISK_SIZE: usize = 70_000_000;
const UPDATE_SIZE: usize = 30_000_000;

trait Sizeable {
    fn get_size(&mut self) -> Result<usize>;
}

#[derive(Debug)]
struct File {
    size: usize,
}

impl File {
    fn new(size: usize) -> File {
        File { size }
    }
}

impl Sizeable for File {
    fn get_size(&mut self) -> Result<usize> {
        if self.size == 0 {
            bail!("tried to get size of file with unknown size")
        }
        return Ok(self.size);
    }
}

#[derive(Debug)]
struct Dir {
    // A size of 0 means we do not know the size yet, and must calculate it.
    size: usize,
    dirs: HashMap<String, Dir>,
    files: HashMap<String, File>,
}

impl Dir {
    fn new() -> Dir {
        Dir {
            size: 0,
            dirs: HashMap::new(),
            files: HashMap::new(),
        }
    }

    fn add_dir(&mut self, name: &str, dir: Dir) -> Result<&mut Dir> {
        if let None = self.dirs.insert(name.to_string(), dir) {
            self.size = 0
        }
        self.get_dir(name).context("could not get just-added dir")
    }

    fn add_file(&mut self, name: &str, file: File) -> Result<&mut File> {
        if let None = self.files.insert(name.to_string(), file) {
            self.size = 0
        }
        self.get_file(name).context("could not get just-added file")
    }

    fn get_dir(&mut self, name: &str) -> Option<&mut Dir> {
        self.dirs.get_mut(name)
    }

    fn get_file(&mut self, name: &str) -> Option<&mut File> {
        self.files.get_mut(name)
    }

    // No need for a `visit_files` method, though in theory we could have one.
    fn visit_dirs<F>(&mut self, f: &mut F) -> Result<()>
    where
        F: FnMut(&mut Dir) -> Result<()>,
    {
        for (_, dir) in &mut self.dirs {
            dir.visit_dirs(f)?;
        }
        Ok(f(self)?)
    }
}

impl Sizeable for Dir {
    fn get_size(&mut self) -> Result<usize> {
        if self.size == 0 {
            for (_, file) in &mut self.files {
                self.size += file.get_size()?;
            }
            for (_, dir) in &mut self.dirs {
                self.size += dir.get_size()?;
            }
        }
        return Ok(self.size);
    }
}

type LinesParsed = usize;
fn parse(lines: &[String], dir: &mut Dir) -> Result<LinesParsed> {
    let line_count = lines.len();
    let mut lines_parsed = 0;
    while lines_parsed < line_count - 1 {
        lines_parsed += 1;
        let mut parts = lines[lines_parsed].split_whitespace();
        match parts.next() {
            None => bail!("invalid empty line"),
            Some(first) => match first {
                "$" => match parts.next() {
                    None => bail!("empty command"),
                    Some(second) => match second {
                        "ls" => {}
                        "cd" => match parts.next() {
                            None => bail!("no target dir for cd command"),
                            Some(third) => match third {
                                "/" => bail!("cannot return to root after start"),
                                ".." => return Ok(lines_parsed),
                                _ => {
                                    let mut next_dir = dir.add_dir(third, Dir::new())?;
                                    lines_parsed += parse(&lines[lines_parsed..], &mut next_dir)?;
                                }
                            },
                        },
                        _ => bail!("invalid command"),
                    },
                },
                "dir" => match parts.next() {
                    None => bail!("invalid dir line in response to ls"),
                    Some(second) => {
                        dir.add_dir(second, Dir::new())?;
                    }
                },
                _ => match parts.next() {
                    None => bail!("invalid dir line in response to ls"),
                    Some(second) => {
                        dir.add_file(second, File::new(first.parse::<usize>()?))?;
                    }
                },
            },
        };
    }
    Ok(lines_parsed)
}

fn main() -> Result<()> {
    let (lines, variant) = get_input_file_lines_with_variant()?;
    if lines.is_empty() || lines[0] != "$ cd /" {
        bail!("first command must be `$ cd /`")
    }

    let mut root = Dir::new();
    parse(&lines[1..], &mut root)?;

    match variant {
        Variant::A => {
            let mut sum = 0;
            root.visit_dirs(&mut |dir: &mut Dir| {
                let size = dir.get_size()?;
                if size < LIMIT {
                    sum += size;
                }
                Ok(())
            })?;
            println!("The sum of all sufficiently small directories is: {}", sum);
        }
        Variant::B => {
            let free_disk_space = DISK_SIZE.checked_sub(root.get_size()?).context("a")?;
            let deletion_target = UPDATE_SIZE.checked_sub(free_disk_space).context("b")?;
            println!("The deletion target is: {}", deletion_target);

            let mut dir_sizes = BTreeSet::<usize>::new();
            root.visit_dirs(&mut |dir: &mut Dir| {
                dir_sizes.insert(dir.get_size()?);
                Ok(())
            })?;
            println!(
                "The size of the smallest directory to delete to make space for the update is: {}",
                dir_sizes.range(deletion_target..).next().context("no directory is small enough")?
            );
        }
    }
    Ok(())
}
