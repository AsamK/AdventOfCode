use errors::{ACResult, Error};
use std::collections::HashSet;

pub fn a01_1(lines: Vec<String>) -> ACResult<i32> {
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

pub fn a01_2(lines: Vec<String>) -> ACResult<i32> {
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
