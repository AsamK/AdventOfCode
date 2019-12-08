use crate::errors::{ACResult, Error};
use md5::{Digest, Md5};
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&crate::utils::read_line(data)?).map(|r| r.to_string()),
        2 => level_2(&crate::utils::read_line(data)?).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

fn level_1(input: &str) -> ACResult<u32> {
    let mut i = 1;
    loop {
        let mut hasher = Md5::new();
        let value = input.to_owned() + &i.to_string();
        hasher.input(value.into_bytes());
        let hash = hasher.result();
        let hash = format!("{:x}", hash);
        if hash.chars().take(5).filter(|c| *c == '0').count() == 5 {
            break;
        }
        i += 1;
    }
    Ok(i)
}

fn level_2(input: &str) -> ACResult<u32> {
    let mut i = 1;
    loop {
        let mut hasher = Md5::new();
        let value = input.to_owned() + &i.to_string();
        hasher.input(value.into_bytes());
        let hash = hasher.result();
        let hash = format!("{:x}", hash);
        if hash.chars().take(6).filter(|c| *c == '0').count() == 6 {
            break;
        }
        i += 1;
    }
    Ok(i)
}
