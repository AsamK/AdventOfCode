use crate::errors::{ACResult, Error};
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&crate::utils::read_lines(data)?[0]).map(|r| r.to_string()),
        2 => level_2(&crate::utils::read_lines(data)?[0]).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

fn get_digits(i: usize) -> Vec<u8> {
    i.to_string()
        .chars()
        .map(|c| c.to_string().parse::<u8>().unwrap())
        .collect()
}

fn check1(i: usize) -> bool {
    let mut prev = None;
    let mut same = false;
    let mut decreased = false;
    for n in get_digits(i).iter() {
        if prev == Some(n) {
            same = true;
        }
        if let Some(prev) = prev {
            if prev > n {
                decreased = true;
                break;
            }
        }

        prev = Some(n);
    }
    same && !decreased
}

fn level_1(line: &str) -> ACResult<usize> {
    let parts = line
        .split('-')
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let (from, to) = (parts[0], parts[1]);

    Ok((from..=to).filter(|i| check1(*i)).count())
}

fn check2(i: usize) -> bool {
    let mut prev = None;
    let mut sames = 0;
    let mut same = false;
    let mut decreased = false;
    for n in get_digits(i).iter() {
        if prev == Some(n) {
            sames += 1;
        } else if sames == 1 {
            sames = 0;
            same = true;
        } else {
            sames = 0;
        }
        if let Some(prev) = prev {
            if prev > n {
                decreased = true;
                break;
            }
        }

        prev = Some(n);
    }
    (same || sames == 1) && !decreased
}

fn level_2(line: &str) -> ACResult<usize> {
    let parts = line
        .split('-')
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let (from, to) = (parts[0], parts[1]);

    Ok((from..=to).filter(|i| check2(*i)).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_level_1_examples() {
        assert_eq!(check1(111111), true);
        assert_eq!(check1(223450), false);
        assert_eq!(check1(123789), false);
    }

    #[test]
    fn run_level_2_examples() {
        assert_eq!(check2(112233), true);
        assert_eq!(check2(123444), false);
        assert_eq!(check2(111122), true);
    }
}
