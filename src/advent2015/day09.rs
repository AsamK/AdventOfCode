use crate::errors::{ACResult, Error};
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_while1},
    IResult,
};
use std::collections::HashSet;
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&crate::utils::read_lines(data)?).map(|r| r.to_string()),
        2 => level_2(&crate::utils::read_lines(data)?).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

#[derive(Debug, Clone)]
struct Link {
    from: String,
    to: String,
    distance: usize,
}

fn parse_link(i: &str) -> IResult<&str, Link> {
    let (i, from) = take_while1(|c: char| c.is_alphabetic())(i)?;
    let (i, _) = tag(" to ")(i)?;
    let (i, to) = take_while1(|c: char| c.is_alphabetic())(i)?;
    let (i, _) = tag(" = ")(i)?;
    let (i, distance) = take_while1(|c: char| c.is_digit(10))(i)?;

    Ok((
        i,
        Link {
            from: from.to_string(),
            to: to.to_string(),
            distance: distance.parse().unwrap(),
        },
    ))
}

fn step(
    max: usize,
    links: &[Link],
    visited: &HashSet<String>,
    from: &str,
    check: &dyn Fn(usize, usize) -> bool,
) -> Option<usize> {
    let mut visited = visited.clone();
    visited.insert(from.to_string());

    if max == visited.len() {
        return Some(0);
    }

    let mut shortest = None;
    for l in links
        .iter()
        .filter(|l| l.from == from && !visited.contains(&l.to))
    {
        let res = step(max, links, &visited, &l.to, check);
        if let Some(dist) = res {
            let dist = dist + l.distance;
            if shortest.is_none() || check(dist, shortest.unwrap()) {
                shortest = Some(dist);
            }
        }
    }
    shortest
}

fn level_1(lines: &[String]) -> ACResult<usize> {
    let links = lines
        .iter()
        .map(|l| parse_link(l).map(|l| l.1).map_err(Error::from))
        .collect::<ACResult<Vec<_>>>()?;
    let links = links
        .iter()
        .cloned()
        .chain(links.iter().map(|l| Link {
            from: l.to.clone(),
            to: l.from.clone(),
            distance: l.distance,
        }))
        .collect::<Vec<_>>();
    let max = links
        .iter()
        .map(|l| l.to.clone())
        .chain(links.iter().map(|l| l.from.clone()))
        .unique()
        .count();
    let mut shortest = None;
    let visited = HashSet::new();
    for start in links.iter().map(|l| &l.from).unique() {
        let res = step(max, &links, &visited, start, &|d1, d2| d1 < d2);
        if let Some(dist) = res {
            if shortest.is_none() || shortest.unwrap() > dist {
                shortest = Some(dist);
            }
        }
    }
    shortest.ok_or_else(|| Error::new_str("No path found"))
}

fn level_2(lines: &[String]) -> ACResult<usize> {
    let links = lines
        .iter()
        .map(|l| parse_link(l).map(|l| l.1).map_err(Error::from))
        .collect::<ACResult<Vec<_>>>()?;
    let links = links
        .iter()
        .cloned()
        .chain(links.iter().map(|l| Link {
            from: l.to.clone(),
            to: l.from.clone(),
            distance: l.distance,
        }))
        .collect::<Vec<_>>();
    let max = links
        .iter()
        .map(|l| l.to.clone())
        .chain(links.iter().map(|l| l.from.clone()))
        .unique()
        .count();
    let mut longest = None;
    let visited = HashSet::new();
    for start in links.iter().map(|l| &l.from).unique() {
        let res = step(max, &links, &visited, start, &|d1, d2| d1 > d2);
        if let Some(dist) = res {
            if longest.is_none() || longest.unwrap() < dist {
                longest = Some(dist);
            }
        }
    }
    longest.ok_or_else(|| Error::new_str("No path found"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_level_1_examples() {
        assert_eq!(
            level_1(&[
                "London to Dublin = 464".to_owned(),
                "London to Belfast = 518".to_owned(),
                "Dublin to Belfast = 141".to_owned(),
            ]),
            Ok(605)
        );
    }

    #[test]
    fn run_level_2_examples() {
        assert_eq!(
            level_2(&[
                "London to Dublin = 464".to_owned(),
                "London to Belfast = 518".to_owned(),
                "Dublin to Belfast = 141".to_owned(),
            ]),
            Ok(982)
        );
    }
}
