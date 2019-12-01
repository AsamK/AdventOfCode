use crate::errors::{ACResult, Error};
use itertools::Itertools;
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&crate::utils::read_lines(data)?[0]).map(|r| r.to_string()),
        2 => level_2(&crate::utils::read_lines(data)?[0]).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

fn level_1(line: &str) -> ACResult<usize> {
    let mut houses = Vec::with_capacity(line.len());
    let mut x = 0;
    let mut y = 0;
    houses.push((x, y));
    for d in line.chars() {
        match d {
            '<' => x -= 1,
            '>' => x += 1,
            '^' => y -= 1,
            'v' => y += 1,
            _ => return Err(Error::new_str("Invalid input")),
        }
        houses.push((x, y));
    }
    Ok(houses.iter().unique().count())
}

fn level_2(line: &str) -> ACResult<usize> {
    let mut houses = Vec::with_capacity(line.len());
    let mut x1 = 0;
    let mut y1 = 0;
    let mut x2 = 0;
    let mut y2 = 0;
    houses.push((x1, y1));
    houses.push((x2, y2));
    for d in line.chars().step_by(2) {
        match d {
            '<' => x1 -= 1,
            '>' => x1 += 1,
            '^' => y1 -= 1,
            'v' => y1 += 1,
            _ => return Err(Error::new_str("Invalid input")),
        }
        houses.push((x1, y1));
    }
    for d in line.chars().skip(1).step_by(2) {
        match d {
            '<' => x2 -= 1,
            '>' => x2 += 1,
            '^' => y2 -= 1,
            'v' => y2 += 1,
            _ => return Err(Error::new_str("Invalid input")),
        }
        houses.push((x2, y2));
    }
    Ok(houses.iter().unique().count())
}
