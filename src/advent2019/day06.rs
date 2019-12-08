use crate::errors::{ACResult, Error};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&crate::utils::read_lines(data)?).map(|r| r.to_string()),
        2 => level_2(&crate::utils::read_lines(data)?).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

fn level_1(lines: &[String]) -> ACResult<u32> {
    let orbits = lines
        .iter()
        .map(|l| l.split(')').collect::<Vec<_>>())
        .map(|l| (l[0], l[1]))
        .collect::<Vec<_>>();

    let mut next = vec!["COM".to_owned()];
    let mut orbit_count = 0;
    let mut level = 0;
    while !next.is_empty() {
        let list = next;
        level += 1;
        next = Vec::new();
        for current in list.iter() {
            for (f, t) in orbits.iter() {
                if **f == *current {
                    next.push(t.to_string());
                    orbit_count += level;
                }
            }
        }
    }
    Ok(orbit_count)
}

#[derive(PartialEq, Eq)]
struct StepNaive {
    level: u32,
    name: String,
    visited: HashSet<String>,
}

impl PartialOrd for StepNaive {
    fn partial_cmp(&self, other: &StepNaive) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for StepNaive {
    fn cmp(&self, other: &StepNaive) -> Ordering {
        match self.level.cmp(&other.level) {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
        }
    }
}

fn level_2(lines: &[String]) -> ACResult<u32> {
    let orbits = lines
        .iter()
        .map(|l| l.split(')').collect::<Vec<_>>())
        .map(|l| (l[0], l[1]))
        .collect::<Vec<_>>();

    let mut next = BinaryHeap::new();
    next.push(StepNaive {
        level: 0,
        name: "SAN".to_owned(),
        visited: HashSet::new(),
    });
    while let Some(current) = next.pop() {
        for (f, t) in orbits.iter() {
            let mut visited = current.visited.clone();
            visited.insert(current.name.clone());
            if **f == current.name && !visited.contains(&t.to_string()) {
                next.push(StepNaive {
                    level: current.level + 1,
                    name: t.to_string(),
                    visited,
                });
                if *t == "YOU" {
                    return Ok(current.level - 1);
                }
            } else if **t == current.name && !visited.contains(&f.to_string()) {
                next.push(StepNaive {
                    level: current.level + 1,
                    name: f.to_string(),
                    visited,
                });
                if *f == "YOU" {
                    return Ok(current.level - 1);
                }
            }
        }
    }
    Err(Error::new_str("Not found"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_level_1_examples() {
        assert_eq!(
            level_1(&[
                "COM)B".to_owned(),
                "B)C".to_owned(),
                "C)D".to_owned(),
                "D)E".to_owned(),
                "E)F".to_owned(),
                "B)G".to_owned(),
                "G)H".to_owned(),
                "D)I".to_owned(),
                "E)J".to_owned(),
                "J)K".to_owned(),
                "K)L".to_owned(),
            ]),
            Ok(42),
        );
    }

    #[test]
    fn run_level_2_examples() {
        assert_eq!(
            level_2(&[
                "COM)B".to_owned(),
                "B)C".to_owned(),
                "C)D".to_owned(),
                "D)E".to_owned(),
                "E)F".to_owned(),
                "B)G".to_owned(),
                "G)H".to_owned(),
                "D)I".to_owned(),
                "E)J".to_owned(),
                "J)K".to_owned(),
                "K)L".to_owned(),
                "K)YOU".to_owned(),
                "I)SAN".to_owned()
            ]),
            Ok(4),
        );
    }
}
