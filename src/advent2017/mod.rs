use crate::errors::{ACResult, Error};
use std::io::{BufRead, Read};

mod day25;

pub fn get_result<T: Read + BufRead>(data: T, day: u8, level: u8) -> ACResult<String> {
    match day {
        25 => match level {
            1 => day25::a25_1(&day25::parser::parse_turing(data)?).map(|r| r.to_string()),
            _ => Err(Error::new(format!(
                "Level {} not implemented for day {}",
                level, day
            ))),
        },
        _ => Err(Error::new(format!("Day {} not implemented", day))),
    }
}
