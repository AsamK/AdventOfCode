use crate::errors::{ACResult, Error};
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&crate::utils::read_line(data)?).map(|r| r.to_string()),
        2 => level_2(&crate::utils::read_line(data)?).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

fn check_image(input: &str, width: u32, height: u32) -> ACResult<u32> {
    let mut i = 0;
    let mut count0 = 0;
    let mut count1 = 0;
    let mut count2 = 0;
    let mut largest = None;
    let mut rest = None;
    for c in input.chars() {
        i += 1;
        if c == '0' {
            count0 += 1;
        } else if c == '1' {
            count1 += 1;
        } else if c == '2' {
            count2 += 1;
        }
        if i == width * height {
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

fn level_1(line: &str) -> ACResult<u32> {
    check_image(line, 25, 6)
}

fn compute_image(input: &str, width: usize, height: usize) -> ACResult<String> {
    let mut matrix = vec!['2'; width * height];
    let mut i = 0;
    for c in input.chars() {
        if matrix[i] == '2' {
            matrix[i] = c;
        }
        i += 1;
        if i == width * height {
            i = 0;
        }
    }
    let mut result = String::with_capacity(width * height + height);
    for i in 0..height {
        for j in 0..width {
            let c = match matrix[(i * width + j) as usize] {
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

fn level_2(line: &str) -> ACResult<String> {
    compute_image(line, 25, 6)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_level_1_examples() {
        assert_eq!(check_image("123456789012", 3, 2), Ok(1),);
    }

    #[test]
    fn run_level_2_examples() {
        assert_eq!(
            compute_image("0222112222120000", 2, 2),
            Ok(" X\nX \n".to_owned())
        );
    }
}
