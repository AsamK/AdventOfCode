use crate::errors::{ACResult, Error};
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&crate::utils::read_lines(data)?).map(|r| r.to_string()),
        2 => level_2(&crate::utils::read_lines(data)?).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

fn check_string(input: &str) -> bool {
    let mut prev_char = None;

    let mut double_char = false;
    let mut vowel_count = 0;
    let forbidden = [('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')];
    for c in input.chars() {
        if let Some(p) = prev_char {
            if p == c {
                double_char = true;
            }
            if forbidden.iter().any(|f| f.0 == p && f.1 == c) {
                return false;
            }
        }
        prev_char = Some(c);
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => vowel_count += 1,
            _ => {}
        };
    }
    double_char && vowel_count >= 3
}

fn level_1(lines: &[String]) -> ACResult<usize> {
    Ok(lines.iter().filter(|l| check_string(l)).count())
}

fn check_string2(input: &str) -> bool {
    let mut prev_prev_char = None;
    let mut prev_char = None;
    let mut next_pair = None;
    let mut pairs = Vec::with_capacity(input.len());

    let mut double_char = false;
    let mut has_pair = false;
    for c in input.chars() {
        if let (Some(pp), Some(p)) = (prev_prev_char, prev_char) {
            if p != c && pp == c {
                double_char = true;
            }
        }
        if let Some(p) = prev_char {
            let pair = format!("{}{}", p, c);
            if pairs.contains(&pair) {
                has_pair = true
            }
            if let Some(next_pair) = next_pair {
                pairs.push(next_pair);
            }
            next_pair = Some(pair);
        }

        prev_prev_char = prev_char;
        prev_char = Some(c);
    }
    double_char && has_pair
}

fn level_2(lines: &[String]) -> ACResult<usize> {
    Ok(lines.iter().filter(|l| check_string2(l)).count())
}
