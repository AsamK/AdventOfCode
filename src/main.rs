use errors::{ACResult, Error};

mod day01;
mod day02;
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
        1 => match level {
            1 => day01::a01_1(lines),
            2 => day01::a01_2(lines),
            _ => Err(Error::new(format!(
                "Level {} not implemented for day {}",
                level, day
            ))),
        }.map(|r| r.to_string()),
        2 => match level {
            1 => day02::a02_1(lines).map(|r| r.to_string()),
            2 => day02::a02_2(lines),
            _ => Err(Error::new(format!(
                "Level {} not implemented for day {}",
                level, day
            ))),
        },
        _ => Err(Error::new(format!("Day {} not implemented", day))),
    }?;

    println!("{}", result);

    Ok(())
}
