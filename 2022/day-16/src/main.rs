use anyhow::{bail, Context, Error, Result};
use common::get_input_file_lines_with_variant;
use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Name {
    id: [char; 2],
}

#[derive(Debug)]
struct Valve {
    flow_rate: u32,
}

#[derive(Debug, Default)]
struct Network {
    valves: HashMap<Name, Valve>,
    tunnels: HashMap<Name, Vec<Name>>,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Timer {
    remaining: u32,
}

impl Timer {
    fn is_done(&self) -> bool {
        self.remaining == 0
    }

    fn enter_room(&self) -> Timer {
        if self.remaining <= 1 {
            return Timer { remaining: 0 };
        }
        Timer {
            remaining: self.remaining - 1,
        }
    }

    fn open_valve(&self) -> Timer {
        if self.remaining <= 1 {
            return Timer { remaining: 0 };
        }
        Timer {
            remaining: self.remaining - 1,
        }
    }
}

#[derive(Debug, Default)]
struct State {
    pressure_released: u32,
    timer: Timer,
}

fn parse_name(text: &str) -> Result<Name> {
    if text.len() != 2 {
        bail!(format!(
            "parsing: valve names must be exactly two characters long, found: {}",
            text
        ));
    }

    let mut chars = text.chars();
    let id = [
        chars.next().context("parsing: first character missing")?,
        chars.next().context("parsing: second character missing")?,
    ];
    id.iter().try_for_each(|ch| {
        if !ch.is_ascii_uppercase() {
            bail!(format!(
                "parsing: valve names must only contain uppercase alphabetic characters, saw: {}",
                ch
            ));
        }
        Ok(())
    })?;
    Ok(Name { id })
}

fn parse(lines: Vec<String>) -> Result<Network> {
    lines
        .into_iter()
        .try_fold(Network::default(), |mut network, line| {
            let re = Regex::new(
                r"Valve (\w\w) has flow rate=(\d+); tunnels? leads? to valves? ((?:\w\w, )*)(\w\w)",
            )
            .context("parsing: regex matching line failed")?;
            for (index, cap) in re.captures_iter(line.as_str()).enumerate() {
                if index > 0 {
                    bail!("parsing: unexpectedly encountered multiple lines");
                }
                if cap[1].is_empty() {
                    bail!("parsing: unmatched valve name");
                }
                if cap[2].is_empty() {
                    bail!("parsing: unmatched flow rate");
                }
                if cap[4].is_empty() {
                    bail!("parsing: must connect to at least one other valve room");
                }

                let name = parse_name(&cap[1])?;
                network.tunnels.insert(
                    name,
                    cap[3]
                        .split(",")
                        .try_fold(vec![parse_name(&cap[4])?], |mut acc, end| {
                            let trimmed = end.trim();
                            if !trimmed.is_empty() {
                                acc.push(parse_name(trimmed)?);
                            }
                            Ok::<_, Error>(acc)
                        })?,
                );
                network.valves.insert(
                    name,
                    Valve {
                        flow_rate: cap[2].parse::<u32>()?,
                    },
                );
            }
            Ok(network)
        })
}

fn dfs(
    network: &Network,
    visited: &mut HashMap<Name, u32>,
    path: &mut Vec<Name>,
    at: Name,
    state: State,
) -> Result<(u32, Vec<Name>)> {
    if state.timer.is_done() {
        return Ok((state.pressure_released, vec![]));
    }

    // If this valve has not yet been flipped, calculate how much pressure flipping it will release.
    let flow_rate = network
        .valves
        .get(&at)
        .context(format!("dfs: name {:?} not found in valve map", at))?
        .flow_rate;
    let mut pressure_released = state.pressure_released;
    let next_timer = if flow_rate > 0 {
        // println!("take extra second off!");
        state.timer.enter_room().open_valve()
    } else {
        state.timer.enter_room()
    };
    visited
        .entry(at)
        .and_modify(|count| {
            *count += 1;
        })
        .or_insert_with(|| {
            pressure_released += state.timer.remaining * flow_rate;
            1
        });

    let mut best_pressure_release = (pressure_released, vec![]);
    let possible_tunnels = network
        .tunnels
        .get(&at)
        .context(format!("dfs: name {:?} not found in tunnels map", at))?;
    for name in possible_tunnels {
        let mut foo = vec![*name];
        path.push(*name);
        // println!(
        //     "At time remaining ({:?}), try {:?} from {:?}, with next_timer: {:?}",
        //     state.timer.remaining, name, at, next_timer
        // );
        let mut best_for_tunnel = dfs(
            network,
            visited,
            path,
            *name,
            State {
                pressure_released,
                timer: next_timer,
            },
        )?;
        // println!(
        //     "At time remaining ({:?}), for {:?}, released {:?}",
        //     state.timer.remaining, name, best_for_tunnel.0
        // );
        if best_for_tunnel.0 > best_pressure_release.0 {
            foo.append(&mut best_for_tunnel.1);
            best_pressure_release = (best_for_tunnel.0, foo);
        }
        path.pop();
    }

    if flow_rate > 0 {
        *visited.entry(at).or_insert(0) -= 1;
    }
    // println!(
    //     "At time remaining ({:?}), best release is {:?}, path is {:?}",
    //     state.timer.remaining, best_pressure_release.0, best_pressure_release.1
    // );
    return Ok(best_pressure_release);
}

fn main() -> Result<()> {
    let (lines, _variant) = get_input_file_lines_with_variant()?;

    let starting_room_name = parse_name("AA")?;
    let network = parse(lines)?;
    // println!("{:#?}", network);

    let pressure_released = dfs(
        &network,
        &mut HashMap::<Name, u32>::new(),
        &mut vec![],
        starting_room_name,
        State {
            pressure_released: 0,
            timer: Timer { remaining: 30 },
        },
    )?;

    println!("{:#?}", pressure_released.1);
    println!(
        "The most pressure we can release in the time allotted is: {}",
        pressure_released.0
    );
    Ok(())
}
