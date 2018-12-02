use crate::errors::{ACResult, Error};
use std::io::{BufRead, Read};

mod day01;
mod day02;

pub fn get_result<T: Read + BufRead>(data: T, day: u8, level: u8) -> ACResult<String> {
    let lines = crate::utils::read_lines(data)?;

    match day {
        1 => match level {
            1 => day01::a01_1(lines),
            2 => day01::a01_2(lines),
            _ => Err(Error::new(format!(
                "Level {} not implemented for day {}",
                level, day
            ))),
        }.map(|r| r.to_string()),
        2 => match level {
            1 => day02::a02_1(lines).map(|r| r.to_string()),
            2 => day02::a02_2(lines),
            _ => Err(Error::new(format!(
                "Level {} not implemented for day {}",
                level, day
            ))),
        },
        _ => Err(Error::new(format!("Day {} not implemented", day))),
    }
}
