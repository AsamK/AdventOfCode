use crate::errors::{ACResult, Error};
use std::io::{BufRead, Read};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

pub fn get_result<T: Read + BufRead>(data: T, day: u8, level: u8) -> ACResult<String> {
    match day {
        1 => day01::get_result(data, level),
        2 => day02::get_result(data, level),
        3 => day03::get_result(data, level),
        4 => day04::get_result(data, level),
        5 => day05::get_result(data, level),
        6 => day06::get_result(data, level),
        7 => day07::get_result(data, level),
        8 => day08::get_result(data, level),
        9 => day09::get_result(data, level),
        10 => day10::get_result(data, level),
        11 => day11::get_result(data, level),
        12 => day12::get_result(data, level),
        13 => day13::get_result(data, level),
        14 => day14::get_result(data, level),
        15 => day15::get_result(data, level),
        16 => day16::get_result(data, level),
        17 => day17::get_result(data, level),
        18 => day18::get_result(data, level),
        19 => day19::get_result(data, level),
        20 => day20::get_result(data, level),
        21 => day21::get_result(data, level),
        22 => day22::get_result(data, level),
        23 => day23::get_result(data, level),
        24 => day24::get_result(data, level),
        25 => day25::get_result(data, level),
        _ => Err(Error::new(format!("Day {} not implemented", day))),
    }
}
