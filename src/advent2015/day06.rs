use crate::errors::{ACResult, Error};
use nom::{
    branch::alt, bytes::complete::tag, bytes::complete::take_while1, character::complete::char,
    IResult,
};
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&crate::utils::read_lines(data)?).map(|r| r.to_string()),
        2 => level_2(&crate::utils::read_lines(data)?).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Instruction {
    action: Action,
    corner1: (usize, usize),
    corner2: (usize, usize),
}

fn parse_action(i: &str) -> IResult<&str, Action> {
    let (i, a) = alt((tag("turn on"), tag("turn off"), tag("toggle")))(i)?;
    Ok((
        i,
        match a {
            "turn on" => Action::TurnOn,
            "turn off" => Action::TurnOff,
            "toggle" => Action::Toggle,
            _ => panic!("foo"),
        },
    ))
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    let (i, a) = parse_action(i)?;
    let (i, _) = char(' ')(i)?;
    let (i, x1) = take_while1(|c: char| c.is_digit(10))(i)?;
    let (i, _) = char(',')(i)?;
    let (i, y1) = take_while1(|c: char| c.is_digit(10))(i)?;
    let (i, _) = tag(" through ")(i)?;
    let (i, x2) = take_while1(|c: char| c.is_digit(10))(i)?;
    let (i, _) = char(',')(i)?;
    let (i, y2) = take_while1(|c: char| c.is_digit(10))(i)?;

    Ok((
        i,
        Instruction {
            action: a,
            corner1: (x1.parse().unwrap(), y1.parse().unwrap()),
            corner2: (x2.parse().unwrap(), y2.parse().unwrap()),
        },
    ))
}

fn level_1(lines: &[String]) -> ACResult<u32> {
    let mut grid = [[false; 1000]; 1000];
    for i in lines
        .iter()
        .map(|l| -> ACResult<Instruction> { Ok(parse_instruction(l)?.1) })
    {
        let i = i?;
        grid.iter_mut()
            .skip(i.corner1.0)
            .take(i.corner2.0 - i.corner1.0 + 1)
            .for_each(|column| {
                column
                    .iter_mut()
                    .skip(i.corner1.1)
                    .take(i.corner2.1 - i.corner1.1 + 1)
                    .for_each(|light| match i.action {
                        Action::Toggle => *light = !*light,
                        Action::TurnOn => *light = true,
                        Action::TurnOff => *light = false,
                    })
            });
    }
    Ok(grid
        .iter()
        .map(|l| {
            l.iter()
                .fold(0, |count, c| if *c { count + 1 } else { count })
        })
        .sum())
}

fn level_2(lines: &[String]) -> ACResult<u32> {
    let mut grid = [[0; 1000]; 1000];
    for i in lines
        .iter()
        .map(|l| -> ACResult<Instruction> { Ok(parse_instruction(l)?.1) })
    {
        let i = i?;
        grid.iter_mut()
            .skip(i.corner1.0)
            .take(i.corner2.0 - i.corner1.0 + 1)
            .for_each(|column| {
                column
                    .iter_mut()
                    .skip(i.corner1.1)
                    .take(i.corner2.1 - i.corner1.1 + 1)
                    .for_each(|light| match i.action {
                        Action::Toggle => *light += 2,
                        Action::TurnOn => *light += 1,
                        Action::TurnOff => *light = if *light == 0 { 0 } else { *light - 1 },
                    })
            });
    }
    Ok(grid.iter().map(|l| l.iter().sum::<u32>()).sum())
}
