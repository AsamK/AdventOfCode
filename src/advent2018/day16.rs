use self::interpreter_utils::{Instruction, Opcode, Registers, ALL_OPCODES};
use crate::errors::{ACResult, Error};
use nom::{complete, do_parse, many1, many_m_n, map, named, opt, tag, take_while1, terminated};
use std::io::BufRead;
use std::io::Read;

pub mod interpreter_utils;

pub fn get_result<T: Read + BufRead>(data: T, level: u8) -> ACResult<String> {
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
        .map_err(|e| Error::new(format!("Failed to parse input: {}", e)))
}

const REGISTER_COUNT: usize = 4;

#[derive(Debug)]
struct AnyInstruction {
    opcode: u8,
    input_a: u64,
    input_b: u64,
    output_register: u8,
}

impl AnyInstruction {
    fn to_instruction(&self, opcode: &Opcode) -> Instruction {
        Instruction::new(
            opcode.clone(),
            self.input_a,
            self.input_b,
            self.output_register,
        )
    }
}

#[derive(Debug)]
struct Sample {
    register_before: Registers,
    register_after: Registers,
    instruction: AnyInstruction,
}

#[derive(Debug)]
struct Input {
    samples: Vec<Sample>,
    instructions: Vec<AnyInstruction>,
}

named!(parse_number<&str, u8>,
    complete!(map!(take_while1!(|c: char| c.is_numeric()), |c| c.to_string().parse().unwrap()))
);

named!(parse_number_u64<&str, u64>,
    complete!(map!(take_while1!(|c: char| c.is_numeric()), |c| c.to_string().parse().unwrap()))
);

named!(parse_register_or_value<&str, u64>,
    do_parse!(
        n: parse_number_u64 >>
        opt!(tag!(", ")) >>
        (n)
    )
);

named!(
    parse_register<&str, Registers>,
    do_parse!(
        tag!("[") >>
        registers: many_m_n!(REGISTER_COUNT, REGISTER_COUNT, parse_register_or_value) >>
        tag!("]") >>
        (Registers::new(&registers))
    )
);

named!(
    parse_instruction<&str, AnyInstruction>,
    do_parse!(
        opcode: parse_number >>
        tag!(" ") >>
        input_a: parse_number_u64 >>
        tag!(" ") >>
        input_b: parse_number_u64 >>
        tag!(" ") >>
        output_register: parse_number >>
        (AnyInstruction {
            opcode,
            input_a,
            input_b,
            output_register
        })
    )
);

named!(
    parse_sample<&str, Sample>,
    do_parse!(
        tag!("Before: ") >>
        register_before: parse_register >>
        tag!("\n") >>
        instruction: parse_instruction >>
        tag!("\n") >>
        tag!("After:  ") >>
        register_after: parse_register >>
        tag!("\n") >>
        tag!("\n") >>
        (Sample {
            register_before,
            register_after,
            instruction
        })
    )
);

named!(parse_input<&str, Input>,
    do_parse!(
        samples: many1!(parse_sample) >>
        tag!("\n\n") >>
        instructions: many1!(complete!(terminated!(parse_instruction, tag!("\n")))) >>
        (Input { samples, instructions })
    )
);

fn get_matching_opcodes(sample: &Sample) -> Vec<Opcode> {
    ALL_OPCODES
        .iter()
        .filter(|opcode| {
            let mut result_registers = sample.register_before.clone();
            sample
                .instruction
                .to_instruction(opcode)
                .execute_instruction(&mut result_registers);
            result_registers == sample.register_after
        })
        .cloned()
        .collect()
}

fn get_mapping_from_samples(samples: &[Sample]) -> Vec<Opcode> {
    let possible_matches: Vec<(u8, Vec<Opcode>)> = samples
        .iter()
        .map(|s| (s.instruction.opcode, get_matching_opcodes(s)))
        .collect();
    let mut mapping = vec![Vec::new(); 16];
    for (opcode_id, opcode_matches) in possible_matches {
        let old_matches = &mapping[opcode_id as usize];
        if old_matches.is_empty() {
            mapping[opcode_id as usize] = opcode_matches;
        } else {
            mapping[opcode_id as usize] = old_matches
                .iter()
                .filter(|m| opcode_matches.contains(m))
                .cloned()
                .collect();
        }
    }

    loop {
        let singles: Vec<_> = mapping
            .iter()
            .filter(|l| l.len() == 1)
            .map(|l| l[0].clone())
            .collect();

        if singles.len() == mapping.len() {
            break;
        }

        for m in mapping.iter_mut() {
            if m.len() == 1 {
                continue;
            }
            *m = m.drain(..).filter(|o| !singles.contains(o)).collect();
        }
    }

    mapping.iter().map(|codes| codes[0].clone()).collect()
}

fn level_1(input: &Input) -> ACResult<usize> {
    let result = input
        .samples
        .iter()
        .map(|s| get_matching_opcodes(s))
        .filter(|opcodes| opcodes.len() >= 3)
        .count();
    Ok(result)
}

fn level_2(input: &Input) -> ACResult<u64> {
    let mapping = get_mapping_from_samples(&input.samples);

    let mut registers = Registers::empty(REGISTER_COUNT);

    for instr in input.instructions.iter() {
        instr
            .to_instruction(&mapping[instr.opcode as usize])
            .execute_instruction(&mut registers);
    }

    Ok(*registers.get(0))
}
