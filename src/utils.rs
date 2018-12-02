use errors::{ACResult, Error};
use std::io::BufRead;

pub fn read_lines<T: BufRead>(data: T) -> ACResult<Vec<String>> {
    let lines = data
        .lines()
        .collect::<Result<Vec<String>, _>>()
        .map_err(|_| Error::new_str("Failed to read lines"))?;
    Ok(lines)
}
