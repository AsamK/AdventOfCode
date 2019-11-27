use crate::advent2018::day16::interpreter_utils::{Instruction, Opcode, Registers};
use crate::errors::{ACResult, Error};
use nom::{complete, do_parse, many1, map, named, preceded, tag, take_while1, terminated};
use std::collections::HashSet;
use std::io::BufRead;
use std::io::Read;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&parse_line(data)?).map(|r| r.to_string()),
        2 => level_2(&parse_line(data)?).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

fn parse_line<T: Read>(mut data: T) -> ACResult<Input> {
    let mut contents = String::new();
    data.read_to_string(&mut contents)
        .map_err(|_| Error::new_str("Failed to read data"))?;

    parse_input(&contents)
        .map(|x| x.1)
        .map_err(|_e| Error::new("Failed to parse input".to_owned()))
}

const REGISTER_COUNT: usize = 6;

#[derive(Debug)]
struct Input {
    ip_register: u8,
    instructions: Vec<Instruction>,
}

named!(parse_number<&str, u8>,
    complete!(map!(take_while1!(|c: char| c.is_numeric()), |c| c.to_string().parse().unwrap()))
);

named!(parse_number_u64<&str, u64>,
    complete!(map!(take_while1!(|c: char| c.is_numeric()), |c| c.to_string().parse().unwrap()))
);

named!(
    parse_instruction<&str, Instruction>,
    do_parse!(
        opcode: map!(take_while1!(|c:char| c != ' '), |c| match c {
            "addr" => Opcode::Addr,
            "addi" => Opcode::Addi,
            "mulr" => Opcode::Mulr,
            "muli" => Opcode::Muli,
            "banr" => Opcode::Banr,
            "bani" => Opcode::Bani,
            "borr" => Opcode::Borr,
            "bori" => Opcode::Bori,
            "setr" => Opcode::Setr,
            "seti" => Opcode::Seti,
            "gtir" => Opcode::Gtir,
            "gtri" => Opcode::Gtri,
            "gtrr" => Opcode::Gtrr,
            "eqir" => Opcode::Eqir,
            "eqri" => Opcode::Eqri,
            "eqrr" => Opcode::Eqrr,
            _ => panic!("invalid opcode {}", c),
        }) >>
        tag!(" ") >>
        input_a: parse_number_u64 >>
        tag!(" ") >>
        input_b: parse_number_u64 >>
        tag!(" ") >>
        output_register: parse_number >>
        (Instruction::new(
            opcode,
            input_a,
            input_b,
            output_register
        ))
    )
);

named!(parse_input<&str, Input>,
    do_parse!(
        ip_register: preceded!(tag!("#ip ") , parse_number) >>
        tag!("\n") >>
        instructions: many1!(complete!(terminated!(parse_instruction, tag!("\n")))) >>
        (Input { ip_register, instructions })
    )
);

fn level_1(input: &Input) -> ACResult<u64> {
    let mut registers = Registers::empty(REGISTER_COUNT);

    let mut ip: usize = 0;
    loop {
        *registers.get_mut(input.ip_register) = ip as u64;

        input.instructions[ip].execute_instruction(&mut registers);

        ip = *registers.get(input.ip_register) as usize;

        ip += 1;
        if ip >= input.instructions.len() {
            break;
        }
        if ip == 28 {
            // Assume that the program checks if the current value in register X is equal to
            // register 0. If so it halts
            // Register X is first parameter in the last eqrr instruction.
            let x = input.instructions[ip].get_input_a();
            return Ok(*registers.get(x as u8));
        }
    }

    Err(Error::new_str("No halt condition found"))
}

fn level_2(input: &Input) -> ACResult<u64> {
    let mut registers = Registers::empty(REGISTER_COUNT);

    let mut ip: usize = 0;
    let mut prev = HashSet::new();
    let mut prevs = Vec::new();
    loop {
        *registers.get_mut(input.ip_register) = ip as u64;

        input.instructions[ip].execute_instruction(&mut registers);

        ip = *registers.get(input.ip_register) as usize;

        ip += 1;
        if ip == 28 {
            // Assume that the program checks if the current value in register X is equal to
            // register 0. If so it halts.
            // Register X is first parameter in the last eqrr instruction.
            let x = input.instructions[ip].get_input_a();
            // We need the value that executes the most instructions.
            // So we take the last value, before they start repeating themselves
            let val = *registers.get(x as u8);
            if prev.contains(&val) {
                return Ok(*prevs.last().unwrap());
            }
            prev.insert(val);
            prevs.push(val);
        }
        if ip >= input.instructions.len() {
            break;
        }
    }

    Err(Error::new_str("No halt condition found"))
}
