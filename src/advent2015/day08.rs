use crate::errors::{ACResult, Error};
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&crate::utils::read_lines(data)?).map(|r| r.to_string()),
        2 => level_2(&crate::utils::read_lines(data)?).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

fn unescape(input: &str) -> ACResult<String> {
    let mut result = String::with_capacity(input.len());
    let chars = input.chars().collect::<Vec<_>>();
    if chars.len() < 2 || chars[0] != '"' || chars[chars.len() - 1] != '"' {
        return Err(Error::new_str("Invalid escaped string"));
    }
    let mut i = 1;
    while i < chars.len() - 1 {
        if chars[i] == '\\' {
            i += 1;
            match chars[i] {
                '\\' => result.push('\\'),
                '"' => result.push('"'),
                'x' => {
                    let hex_string = format!("{}{}", chars[i + 1], chars[i + 2]);
                    i += 2;
                    let c = char::from(u8::from_str_radix(&hex_string, 16).unwrap());
                    result.push(c);
                }
                _ => return Err(Error::new_str("invalid escape sequence")),
            }
        } else {
            result.push(chars[i]);
        }
        i += 1;
    }
    Ok(result)
}

fn level_1(lines: &[String]) -> ACResult<usize> {
    Ok(lines
        .iter()
        .map(|l| l.chars().count() - unescape(l).unwrap().chars().count())
        .sum())
}

fn escape(input: &str) -> ACResult<String> {
    let mut result = String::with_capacity(2 * input.len());
    result.push('"');
    for c in input.chars() {
        match c {
            '\\' => result += "\\\\",
            '"' => result += "\\\"",
            c => result.push(c),
        }
    }
    result.push('"');
    Ok(result)
}

fn level_2(lines: &[String]) -> ACResult<usize> {
    Ok(lines
        .iter()
        .map(|l| escape(l).unwrap().chars().count() - l.chars().count())
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_level_1_examples() {
        assert_eq!(
            level_1(&[
                "\"\"".to_owned(),
                "\"abc\"".to_owned(),
                "\"aaa\\\"aaa\"".to_owned(),
                "\"\\x27\"".to_owned()
            ]),
            Ok(12)
        );
    }

    #[test]
    fn run_level_2_examples() {
        assert_eq!(
            level_2(&[
                "\"\"".to_owned(),
                "\"abc\"".to_owned(),
                "\"aaa\\\"aaa\"".to_owned(),
                "\"\\x27\"".to_owned()
            ]),
            Ok(19)
        );
    }
}
