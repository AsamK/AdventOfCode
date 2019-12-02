use crate::errors::{ACResult, Error};
use itertools::Itertools;
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&crate::utils::read_lines(data)?).map(|r| r.to_string()),
        2 => level_2(&crate::utils::read_lines(data)?).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

fn level_1(line: &[String]) -> ACResult<u64> {
    line.iter()
        .map(|l| {
            l.parse::<u64>()
                .map_err(|_e| Error::new_str("Failed to parse mass input"))
        })
        .fold_results(0, |fuel, mass| fuel + (mass / 3 - 2))
}

fn level_2(line: &[String]) -> ACResult<u64> {
    line.iter()
        .map(|l| {
            l.parse::<u64>()
                .map_err(|_e| Error::new_str("Failed to parse mass input"))
        })
        .fold_results(0, |fuel, mass| fuel + compute_fuel(mass))
}

fn compute_fuel(mass: u64) -> u64 {
    let tmp_fuel = mass / 3;
    if tmp_fuel <= 2 {
        return 0;
    }
    let fuel = tmp_fuel - 2;

    fuel + compute_fuel(fuel)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_level_1_examples() {
        assert_eq!(level_1(&["12".to_owned()]), Ok(2));
        assert_eq!(level_1(&["14".to_owned()]), Ok(2));
        assert_eq!(level_1(&["1969".to_owned()]), Ok(654));
        assert_eq!(level_1(&["100756".to_owned()]), Ok(33583));
    }

    #[test]
    fn run_level_2_examples() {
        assert_eq!(level_2(&["14".to_owned()]), Ok(2));
        assert_eq!(level_2(&["1969".to_owned()]), Ok(966));
        assert_eq!(level_2(&["100756".to_owned()]), Ok(50346));
    }
}
