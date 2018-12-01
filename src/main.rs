use errors::{ACResult, Error};

mod errors;
mod utils;

fn main() -> ACResult<()> {
    let day: i32 = std::env::args()
        .nth(1)
        .expect("no day given")
        .parse()
        .map_err(|_| Error::new_str("Failed to parse day"))?;
    let level: i32 = std::env::args()
        .nth(2)
        .expect("no level given")
        .parse()
        .map_err(|_| Error::new_str("Failed to parse level"))?;

    let lines = utils::read_stdin_lines()?;

    let result: String = match day {
        _ => Err(Error::new(format!("Day {} not implemented", day))),
    }?;

    println!("{}", result);

    Ok(())
}
