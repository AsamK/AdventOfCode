use crate::errors::{ACResult, Error};
use std::io::{BufRead, Read};

mod day01;
mod day02;
mod day03;
mod day04;

pub fn get_result<T: Read + BufRead>(data: T, day: u8, level: u8) -> ACResult<String> {
    match day {
        1 => day01::get_result(data, level),
        2 => day02::get_result(data, level),
        3 => day03::get_result(data, level),
        4 => day04::get_result(data, level),
        _ => Err(Error::new(format!("Day {} not implemented", day))),
    }
}
