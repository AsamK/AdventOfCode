use errors::{ACResult, Error};
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&crate::utils::read_lines(data)?[0]),
        // 2 => level_2(crate::utils::read_lines(data)?),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
    .map(|r| r.to_string())
}

fn level_1(line: &str) -> ACResult<u32> {
    let mut first = Option::None::<u32>;
    let mut previous = Option::None::<u32>;
    let mut sum = 0;
    for c in line.chars() {
        let digit = c.to_string().parse();
        match digit {
            Err(_) => {}
            Ok(digit) => {
                match first {
                    Some(_) => {}
                    None => {
                        first = Some(digit);
                    }
                }
                match previous {
                    Some(p) => {
                        if p == digit {
                            sum += p;
                        }
                    }
                    None => {}
                }
                previous = Some(digit);
            }
        }
    }

    let c = previous.unwrap();
    let p = first.unwrap();

    if p == c {
        sum += p;
    }
    Ok(sum)
}
