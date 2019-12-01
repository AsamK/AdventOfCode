use crate::errors::{ACResult, Error};
use nom::{bytes::complete::take_while1, character::complete::char, IResult};
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&crate::utils::read_lines(data)?).map(|r| r.to_string()),
        2 => level_2(&crate::utils::read_lines(data)?).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

struct Present {
    width: u64,
    height: u64,
    length: u64,
}

fn parse_present(i: &str) -> IResult<&str, Present> {
    let (i, l) = take_while1(|c: char| c.is_digit(10))(i)?;
    let (i, _) = char('x')(i)?;
    let (i, w) = take_while1(|c: char| c.is_digit(10))(i)?;
    let (i, _) = char('x')(i)?;
    let (i, h) = take_while1(|c: char| c.is_digit(10))(i)?;

    Ok((
        i,
        Present {
            width: w.parse().unwrap(),
            length: l.parse().unwrap(),
            height: h.parse().unwrap(),
        },
    ))
}

fn level_1(line: &[String]) -> ACResult<u64> {
    let mut fabric_required = 0;
    for p in line
        .iter()
        .map(|l| -> ACResult<Present> { Ok(parse_present(l)?.1) })
    {
        let p = p?;
        let sides = [p.width * p.height, p.height * p.length, p.length * p.width];
        fabric_required +=
            sides.iter().map(|side| 2 * side).sum::<u64>() + sides.iter().min().unwrap();
    }
    Ok(fabric_required)
}

fn level_2(line: &[String]) -> ACResult<u64> {
    let mut ribbon_required = 0;
    for p in line
        .iter()
        .map(|l| -> ACResult<Present> { Ok(parse_present(l)?.1) })
    {
        let p = p?;
        let side_faces = [
            2 * (p.width + p.height),
            2 * (p.height + p.length),
            2 * (p.length + p.width),
        ];
        let volume = p.width * p.height * p.length;
        ribbon_required += side_faces.iter().min().unwrap() + volume;
    }
    Ok(ribbon_required)
}
