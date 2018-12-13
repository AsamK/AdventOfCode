use crate::errors::{ACResult, Error};
use std::io::{BufRead, Read};

pub fn read_lines<T: BufRead>(data: T) -> ACResult<Vec<String>> {
    let lines = data
        .lines()
        .collect::<Result<Vec<String>, _>>()
        .map_err(|_| Error::new_str("Failed to read lines"))?;
    Ok(lines)
}

pub fn read_all<T: Read>(mut data: T) -> ACResult<String> {
    let mut contents = String::new();
    data.read_to_string(&mut contents)
        .map_err(|_| Error::new_str("Failed to read stdin"))?;
    Ok(contents)
}
