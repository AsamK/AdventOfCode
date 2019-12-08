use crate::errors::{ACResult, Error};
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&crate::utils::read_lines(data)?[0]).map(|r| r.to_string()),
        2 => level_2(&crate::utils::read_lines(data)?[0]).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

fn level_1(line: &str) -> ACResult<u32> {
    let mut i = 0;
    let mut count0 = 0;
    let mut count1 = 0;
    let mut count2 = 0;
    let mut largest = None;
    let mut rest = None;
    for c in line.chars() {
        i += 1;
        if c == '0' {
            count0 += 1;
        } else if c == '1' {
            count1 += 1;
        } else if c == '2' {
            count2 += 1;
        }
        if i == 25 * 6 {
            if let Some(l) = largest {
                if count0 < l {
                    largest = Some(count0);
                    rest = Some(count1 * count2);
                }
            } else {
                largest = Some(count0);
                rest = Some(count1 * count2);
            }
            i = 0;
            count0 = 0;
            count1 = 0;
            count2 = 0;
        }
    }
    Ok(rest.unwrap())
}

fn level_2(line: &str) -> ACResult<String> {
    let mut matrix = ['2'; 25 * 6];
    let mut i = 0;
    for c in line.chars() {
        if matrix[i] == '2' {
            matrix[i] = c;
        }
        i += 1;
        if i == 25 * 6 {
            i = 0;
        }
    }
    let mut result = String::with_capacity(25 * 6 + 6);
    for i in 0..6 {
        for j in 0..25 {
            let c = match matrix[(i * 25 + j) as usize] {
                '0' => ' ',
                '1' => 'X',
                c => c,
            };
            result.push(c);
        }
        result.push('\n');
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_level_1_examples() {
        assert_eq!(
            level_1(&["12".to_owned()]),
            Err(Error::new_str("Not implemented"))
        );
    }

    #[test]
    fn run_level_2_examples() {
        assert_eq!(
            level_2(&["14".to_owned()]),
            Err(Error::new_str("Not implemented"))
        );
    }
}
