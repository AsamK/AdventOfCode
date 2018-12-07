use errors::{ACResult, Error};
use std::collections::HashSet;
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(crate::utils::read_lines(data)?),
        2 => level_2(crate::utils::read_lines(data)?),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
    .map(|r| r.to_string())
}

fn level_1(lines: Vec<String>) -> ACResult<i32> {
    let mut freq: i32 = 0;

    for line in lines.iter() {
        let (sign, number) = line.split_at(1);

        let number: i32 = number
            .parse()
            .map_err(|_| Error::new(format!("Invalid input: {}", line)))?;

        match sign {
            "-" => freq -= number,
            "+" => freq += number,
            _ => {
                return Err(Error::new(format!("Invalid input: {}", line)));
            }
        }
    }
    Ok(freq)
}

fn level_2(lines: Vec<String>) -> ACResult<i32> {
    let mut freq = 0;

    let mut freqs = HashSet::new();
    freqs.insert(freq);

    loop {
        for ref line in lines.iter() {
            let (sign, number) = line.split_at(1);

            let number: i32 = number
                .parse()
                .map_err(|_| Error::new(format!("Invalid input: {}", line)))?;

            match sign {
                "-" => freq -= number,
                "+" => freq += number,
                _ => {
                    return Err(Error::new(format!("Invalid input: {}", line)));
                }
            }

            if freqs.contains(&freq) {
                return Ok(freq);
            }

            freqs.insert(freq);
        }
    }
}
