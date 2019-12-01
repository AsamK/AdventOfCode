use crate::errors::{ACResult, Error};
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&crate::utils::read_lines(data)?[0]).map(|r| r.to_string()),
        2 => level_2(&crate::utils::read_lines(data)?[0]).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

fn level_1(line: &str) -> ACResult<i64> {
    line.chars()
        .try_fold(0, |state, direction| match direction {
            '(' => Ok(state + 1),
            ')' => Ok(state - 1),
            _ => Err(Error::new_str("Invalid input char")),
        })
}

fn level_2(line: &str) -> ACResult<usize> {
    let mut state = 0;
    for (i, direction) in line.chars().enumerate() {
        match direction {
            '(' => state += 1,
            ')' => state -= 1,
            _ => return Err(Error::new_str("Invalid input char")),
        };
        if state == -1 {
            return Ok(i + 1);
        }
    }
    Err(Error::new_str("Basement never entered"))
}
