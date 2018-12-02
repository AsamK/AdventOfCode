use crate::errors::{ACResult, Error};
use std::io::{BufRead, Read};

mod day25;

pub fn get_result<T: Read + BufRead>(data: T, day: u8, level: u8) -> ACResult<String> {
    match day {
        25 => day25::get_result(data, level),
        _ => Err(Error::new(format!("Day {} not implemented", day))),
    }
}
