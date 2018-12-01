use errors::{ACResult, Error};
use std::io::{self, BufRead};

pub fn read_stdin_lines() -> ACResult<Vec<String>> {
    let stdin = io::stdin();
    let lines = stdin
        .lock()
        .lines()
        .collect::<Result<Vec<String>, _>>()
        .map_err(|_| Error::new_str("Failed to read lines"))?;
    Ok(lines)
}
