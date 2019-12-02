use crate::errors::{ACResult, Error};
use nom::{branch::alt, bytes::complete::tag, bytes::complete::take_while1, IResult};
use std::collections::HashMap;
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&crate::utils::read_lines(data)?).map(|r| r.to_string()),
        2 => level_2(&crate::utils::read_lines(data)?).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

enum Input {
    Wire(String),
    Value(u16),
}

enum Instruction {
    Set(Input, String),
    And(Input, Input, String),
    Or(Input, Input, String),
    LShift(Input, u16, String),
    RShift(Input, u16, String),
    Not(Input, String),
}

fn parse_input_value(i: &str) -> IResult<&str, Input> {
    let (i, v) = take_while1(|c: char| c.is_digit(10))(i)?;
    Ok((i, Input::Value(v.parse().unwrap())))
}

fn parse_input_wire(i: &str) -> IResult<&str, Input> {
    let (i, v) = take_while1(|c: char| c.is_alphabetic())(i)?;
    Ok((i, Input::Wire(v.to_owned())))
}

fn parse_input(i: &str) -> IResult<&str, Input> {
    alt((parse_input_value, parse_input_wire))(i)
}

fn parse_instruction_set(i: &str) -> IResult<&str, Instruction> {
    let (i, v) = parse_input(i)?;
    let (i, _) = tag(" -> ")(i)?;
    let (i, o) = take_while1(|c: char| c.is_alphabetic())(i)?;
    Ok((i, Instruction::Set(v, o.to_owned())))
}

fn parse_instruction_and(i: &str) -> IResult<&str, Instruction> {
    let (i, i1) = parse_input(i)?;
    let (i, _) = tag(" AND ")(i)?;
    let (i, i2) = parse_input(i)?;
    let (i, _) = tag(" -> ")(i)?;
    let (i, o) = take_while1(|c: char| c.is_alphabetic())(i)?;
    Ok((i, Instruction::And(i1, i2, o.to_owned())))
}

fn parse_instruction_or(i: &str) -> IResult<&str, Instruction> {
    let (i, i1) = parse_input(i)?;
    let (i, _) = tag(" OR ")(i)?;
    let (i, i2) = parse_input(i)?;
    let (i, _) = tag(" -> ")(i)?;
    let (i, o) = take_while1(|c: char| c.is_alphabetic())(i)?;
    Ok((i, Instruction::Or(i1, i2, o.to_owned())))
}

fn parse_instruction_lshift(i: &str) -> IResult<&str, Instruction> {
    let (i, i1) = parse_input(i)?;
    let (i, _) = tag(" LSHIFT ")(i)?;
    let (i, shift) = take_while1(|c: char| c.is_digit(10))(i)?;
    let (i, _) = tag(" -> ")(i)?;
    let (i, o) = take_while1(|c: char| c.is_alphabetic())(i)?;
    Ok((
        i,
        Instruction::LShift(i1, shift.parse().unwrap(), o.to_owned()),
    ))
}

fn parse_instruction_rshift(i: &str) -> IResult<&str, Instruction> {
    let (i, i1) = parse_input(i)?;
    let (i, _) = tag(" RSHIFT ")(i)?;
    let (i, shift) = take_while1(|c: char| c.is_digit(10))(i)?;
    let (i, _) = tag(" -> ")(i)?;
    let (i, o) = take_while1(|c: char| c.is_alphabetic())(i)?;
    Ok((
        i,
        Instruction::RShift(i1, shift.parse().unwrap(), o.to_owned()),
    ))
}

fn parse_instruction_not(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("NOT ")(i)?;
    let (i, i1) = parse_input(i)?;
    let (i, _) = tag(" -> ")(i)?;
    let (i, o) = take_while1(|c: char| c.is_alphabetic())(i)?;
    Ok((i, Instruction::Not(i1, o.to_owned())))
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    alt((
        parse_instruction_set,
        parse_instruction_and,
        parse_instruction_or,
        parse_instruction_lshift,
        parse_instruction_rshift,
        parse_instruction_not,
    ))(i)
}

fn get_value(cables: &HashMap<String, u16>, input: &Input) -> Option<u16> {
    match input {
        Input::Value(v) => Some(*v),
        Input::Wire(w) => cables.get(w).copied(),
    }
}

fn compute(
    cables: &mut HashMap<String, u16>,
    instructions: &[Instruction],
    overriden: Option<&String>,
) {
    let mut modified;
    loop {
        modified = false;
        for i in instructions.iter() {
            match i {
                Instruction::Set(v, out) => {
                    let v = get_value(&cables, v);
                    if let Some(v) = v {
                        let c = cables.get(out);
                        if Some(out) != overriden && (c.is_none() || *c.unwrap() != v) {
                            modified = true;
                            cables.insert(out.to_owned(), v);
                        }
                    }
                }
                Instruction::And(i1, i2, out) => {
                    let i1 = get_value(&cables, i1);
                    let i2 = get_value(&cables, i2);
                    if let (Some(i1), Some(i2)) = (i1, i2) {
                        let result = i1 & i2;
                        let c = cables.get(out);
                        if Some(out) != overriden && (c.is_none() || *c.unwrap() != result) {
                            modified = true;
                            cables.insert(out.to_owned(), result);
                        }
                    }
                }
                Instruction::Or(i1, i2, out) => {
                    let i1 = get_value(&cables, i1);
                    let i2 = get_value(&cables, i2);
                    if let (Some(i1), Some(i2)) = (i1, i2) {
                        let result = i1 | i2;
                        let c = cables.get(out);
                        if Some(out) != overriden && (c.is_none() || *c.unwrap() != result) {
                            modified = true;
                            cables.insert(out.to_owned(), result);
                        }
                    }
                }
                Instruction::LShift(i, shift, out) => {
                    let i = get_value(&cables, i);
                    if let Some(i) = i {
                        let result = i << shift;
                        let c = cables.get(out);
                        if Some(out) != overriden && (c.is_none() || *c.unwrap() != result) {
                            modified = true;
                            cables.insert(out.to_owned(), result);
                        }
                    }
                }
                Instruction::RShift(i, shift, out) => {
                    let i = get_value(&cables, i);
                    if let Some(i) = i {
                        let result = i >> shift;
                        let c = cables.get(out);
                        if Some(out) != overriden && (c.is_none() || *c.unwrap() != result) {
                            modified = true;
                            cables.insert(out.to_owned(), result);
                        }
                    }
                }
                Instruction::Not(i, out) => {
                    let i = get_value(&cables, i);
                    if let Some(i) = i {
                        let result = !i;
                        let c = cables.get(out);
                        if Some(out) != overriden && (c.is_none() || *c.unwrap() != result) {
                            modified = true;
                            cables.insert(out.to_owned(), result);
                        }
                    }
                }
            }
        }
        if !modified {
            break;
        }
    }
}

fn level_1(lines: &[String]) -> ACResult<u16> {
    let instructions = lines
        .iter()
        .map(|l| -> ACResult<Instruction> { Ok(parse_instruction(l)?.1) })
        .collect::<ACResult<Vec<_>>>()?;
    let mut cables = HashMap::<String, u16>::new();
    compute(&mut cables, &instructions, None);
    Ok(*cables.get("a").unwrap())
}

fn level_2(lines: &[String]) -> ACResult<u16> {
    let instructions = lines
        .iter()
        .map(|l| -> ACResult<Instruction> { Ok(parse_instruction(l)?.1) })
        .collect::<ACResult<Vec<_>>>()?;
    let mut cables = HashMap::<String, u16>::new();
    compute(&mut cables, &instructions, None);
    let a = *cables.get("a").unwrap();

    let mut cables = HashMap::<String, u16>::new();
    cables.insert("b".to_owned(), a);
    compute(&mut cables, &instructions, Some(&"b".to_owned()));

    Ok(*cables.get("a").unwrap())
}
