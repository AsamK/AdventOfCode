use crate::errors::{ACResult, Error};
use nom::{bytes::complete::take_while, combinator::map, IResult};
use nom::{do_parse, named, tag};
use std::collections::HashSet;
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&crate::utils::read_lines(data)?).map(|r| r.to_string()),
        2 => level_2().map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
    a: i64,
}

impl Position {
    fn dist(&self, p: &Position) -> i64 {
        (self.x - p.x).abs() + (self.y - p.y).abs() + (self.z - p.z).abs() + (self.a - p.a).abs()
    }
}

fn number(input: &str) -> IResult<&str, i64> {
    map(
        take_while(|c: char| c == '-' || c == ' ' || c.is_digit(10)),
        |s: &str| s.trim().parse().unwrap(),
    )(input)
}

// 9,-1,5,3
named!(
    parse_pos<&str, Position>,
    do_parse!(
        x: number
            >> tag!(",")
            >> y: number
            >> tag!(",")
            >> z: number
            >> tag!(",")
            >> a: number
            >> (Position { x, y, z, a })
    )
);

fn level_1(lines: &[String]) -> ACResult<usize> {
    let positions: Vec<_> = lines.iter().map(|l| parse_pos(l).unwrap().1).collect();

    let mut constellations: Vec<Vec<Position>> = Vec::new();
    let mut used = HashSet::new();
    for (i, pos) in positions.iter().enumerate() {
        if used.contains(&i) {
            continue;
        }

        let mut constellation = Vec::new();
        constellation.push(pos.clone());
        used.insert(i);

        for (j, pos2) in positions.iter().enumerate() {
            if used.contains(&j) {
                continue;
            }
            if pos.dist(pos2) <= 3 {
                constellation.push(pos2.clone());
                used.insert(j);
            }
        }

        let existing_constellation = constellations.iter_mut().find(|c| {
            c.iter()
                .any(|p| constellation.iter().any(|p2| p.dist(p2) <= 3))
        });
        if let Some(c2) = existing_constellation {
            c2.append(&mut constellation);
        } else {
            constellations.push(constellation);
        };
    }

    loop {
        let mut dup = None;
        'outer: for (i, c1) in constellations.iter().enumerate() {
            for (j, c2) in constellations.iter().enumerate() {
                if j <= i {
                    continue;
                }
                if c1.iter().any(|p1| c2.iter().any(|p2| p1.dist(p2) <= 3)) {
                    dup = Some((i, j));
                    break 'outer;
                }
            }
        }
        if let Some((i, j)) = dup {
            let mut c = constellations.remove(j);
            constellations[i].append(&mut c);
        } else {
            break;
        }
    }

    Ok(constellations.len())
}

fn level_2() -> ACResult<i64> {
    Ok(0)
}
