use crate::errors::{ACResult, Error};
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&crate::utils::read_lines(data)?),
        2 => level_2(&crate::utils::read_lines(data)?),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
    .map(|r| r.to_string())
}

fn level_1(lines: &[String]) -> ACResult<u32> {
    let lines = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|c| {
                    c.to_string()
                        .parse()
                        .map_err(|_| Error::new_str("Failed to parse char to number"))
                })
                .collect::<ACResult<Vec<u32>>>()
        })
        .collect::<ACResult<Vec<Vec<u32>>>>()?;

    let mut checksum = 0;
    for numbers in lines.iter() {
        let min = numbers.iter().min().unwrap();
        let max = numbers.iter().max().unwrap();
        checksum += max - min;
    }

    Ok(checksum)
}

fn level_2(lines: &[String]) -> ACResult<u32> {
    let lines = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|c| {
                    c.to_string()
                        .parse()
                        .map_err(|_| Error::new_str("Failed to parse char to number"))
                })
                .collect::<ACResult<Vec<u32>>>()
        })
        .collect::<ACResult<Vec<Vec<u32>>>>()?;

    let mut sum = 0;
    'outer: for numbers in lines.iter() {
        for ia in 0..numbers.len() {
            for ib in ia + 1..numbers.len() {
                let a = numbers[ia];
                let b = numbers[ib];
                if a % b == 0 {
                    sum += a / b;
                    continue 'outer;
                }
                if b % a == 0 {
                    sum += b / a;
                    continue 'outer;
                }
            }
        }
    }

    Ok(sum)
}
