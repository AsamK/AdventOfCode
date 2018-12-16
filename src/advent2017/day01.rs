use crate::errors::{ACResult, Error};
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&crate::utils::read_lines(data)?[0]),
        2 => level_2(&crate::utils::read_lines(data)?[0]),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
    .map(|r| r.to_string())
}

fn level_1(line: &str) -> ACResult<u32> {
    let numbers = line
        .chars()
        .map(|c| {
            c.to_string()
                .parse()
                .map_err(|_| Error::new_str("Failed to parse char to number"))
        })
        .collect::<ACResult<Vec<u8>>>()?;

    let mut sum: u32 = 0;
    for (i, digit) in numbers.iter().enumerate() {
        let comp_index = (i + 1) % numbers.len();
        let p = numbers.get(comp_index).unwrap();
        if *p == *digit {
            sum += *p as u32;
        }
    }

    Ok(sum)
}

fn level_2(line: &str) -> ACResult<u32> {
    let numbers = line
        .chars()
        .map(|c| {
            c.to_string()
                .parse()
                .map_err(|_| Error::new_str("Failed to parse char to number"))
        })
        .collect::<ACResult<Vec<u8>>>()?;

    let mut sum: u32 = 0;
    let skip = numbers.len() / 2;
    for (i, digit) in numbers.iter().enumerate() {
        let comp_index = (i + skip) % numbers.len();
        let p = numbers.get(comp_index).unwrap();
        if *p == *digit {
            sum += *p as u32;
        }
    }

    Ok(sum)
}
