use crate::errors::{ACResult, Error};
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(crate::utils::read_lines(data)?).map(|r| r.to_string()),
        2 => level_2(crate::utils::read_lines(data)?).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

fn level_1(lines: Vec<String>) -> ACResult<usize> {
    let line = lines.get(0).unwrap();
    Ok(get_reacted_len(line))
}

fn level_2(lines: Vec<String>) -> ACResult<usize> {
    let line = lines.get(0).unwrap();

    let alphabet = (b'A'..b'Z' + 1).map(|c| c as char);

    let min = alphabet
        .map(|l| {
            let l = line
                .chars()
                .filter(|c| c.to_lowercase().to_string() != l.to_lowercase().to_string())
                .collect::<String>();
            get_reacted_len(&l)
        })
        .min()
        .unwrap();
    Ok(min)
}

fn get_reacted_len(line: &str) -> usize {
    let mut result = Vec::new();
    for c in line.chars() {
        if let Some(p) = result.last() {
            if c != *p && c.to_lowercase().to_string() == p.to_lowercase().to_string() {
                result.pop();
            } else {
                result.push(c);
            }
        } else {
            result.push(c);
        }
    }
    result.len()
}
