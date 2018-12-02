use errors::{ACResult, Error};

mod advent2018;
mod errors;
mod utils;

fn main() -> ACResult<()> {
    let day: u8 = std::env::args()
        .nth(1)
        .expect("no day given")
        .parse()
        .map_err(|_| Error::new_str("Failed to parse day"))?;
    let level: u8 = std::env::args()
        .nth(2)
        .expect("no level given")
        .parse()
        .map_err(|_| Error::new_str("Failed to parse level"))?;

    let stdin = std::io::stdin();
    let data = stdin.lock();

    let result = advent2018::get_result(data, day, level)?;

    println!("{}", result);

    Ok(())
}
