use crate::errors::{ACResult, Error};
use std::io::BufRead;
use std::io::Read;

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

#[derive(Debug, PartialEq, Eq, Clone)]
struct Registers {
    data: [usize; REGISTER_COUNT],
}

impl Registers {
    fn new(registers: &[u8]) -> Self {
        if registers.len() != REGISTER_COUNT {
            panic!("Needs to be four elements long");
        }
        Registers {
            data: [
                registers[0] as usize,
                registers[1] as usize,
                registers[2] as usize,
                registers[3] as usize,
            ],
        }
    }

    fn empty() -> Self {
        Registers { data: [0, 0, 0, 0] }
    }

    fn get(&self, i: u8) -> &usize {
        &self.data[i as usize]
    }

    fn get_mut(&mut self, i: u8) -> &mut usize {
        &mut self.data[i as usize]
    }
}

fn execute_instruction(i: &Instruction, reg: &mut Registers) {
    match i.opcode {
        Opcode::Addr => {
            *reg.get_mut(i.output_register) = reg.get(i.input_a) + reg.get(i.input_b);
        }
        Opcode::Addi => {
            *reg.get_mut(i.output_register) = reg.get(i.input_a) + i.input_b as usize;
        }
        Opcode::Mulr => {
            *reg.get_mut(i.output_register) = reg.get(i.input_a) * reg.get(i.input_b);
        }
        Opcode::Muli => {
            *reg.get_mut(i.output_register) = reg.get(i.input_a) * i.input_b as usize;
        }
        Opcode::Banr => {
            *reg.get_mut(i.output_register) = reg.get(i.input_a) & reg.get(i.input_b);
        }
        Opcode::Bani => {
            *reg.get_mut(i.output_register) = reg.get(i.input_a) & i.input_b as usize;
        }
        Opcode::Borr => {
            *reg.get_mut(i.output_register) = reg.get(i.input_a) | reg.get(i.input_b);
        }
        Opcode::Bori => {
            *reg.get_mut(i.output_register) = reg.get(i.input_a) | i.input_b as usize;
        }
        Opcode::Setr => {
            *reg.get_mut(i.output_register) = *reg.get(i.input_a);
        }
        Opcode::Seti => {
            *reg.get_mut(i.output_register) = i.input_a as usize;
        }
        Opcode::Gtir => {
            *reg.get_mut(i.output_register) = if i.input_a as usize > *reg.get(i.input_b) {
                1
            } else {
                0
            };
        }
        Opcode::Gtri => {
            *reg.get_mut(i.output_register) = if *reg.get(i.input_a) > i.input_b as usize {
                1
            } else {
                0
            };
        }
        Opcode::Gtrr => {
            *reg.get_mut(i.output_register) = if *reg.get(i.input_a) > *reg.get(i.input_b) {
                1
            } else {
                0
            };
        }
        Opcode::Eqir => {
            *reg.get_mut(i.output_register) = if i.input_a as usize == *reg.get(i.input_b) {
                1
            } else {
                0
            };
        }
        Opcode::Eqri => {
            *reg.get_mut(i.output_register) = if *reg.get(i.input_a) == i.input_b as usize {
                1
            } else {
                0
            };
        }
        Opcode::Eqrr => {
            *reg.get_mut(i.output_register) = if *reg.get(i.input_a) == *reg.get(i.input_b) {
                1
            } else {
                0
            };
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

const ALL_OPCODES: [Opcode; 16] = [
    Opcode::Addr,
    Opcode::Addi,
    Opcode::Mulr,
    Opcode::Muli,
    Opcode::Banr,
    Opcode::Bani,
    Opcode::Borr,
    Opcode::Bori,
    Opcode::Setr,
    Opcode::Seti,
    Opcode::Gtir,
    Opcode::Gtri,
    Opcode::Gtrr,
    Opcode::Eqir,
    Opcode::Eqri,
    Opcode::Eqrr,
];

#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    input_a: u8,
    input_b: u8,
    output_register: u8,
}

impl Instruction {
    fn from_any_instruction(opcode: &Opcode, instruction: &AnyInstruction) -> Self {
        Instruction {
            opcode: opcode.clone(),
            input_a: instruction.input_a,
            input_b: instruction.input_b,
            output_register: instruction.output_register,
        }
    }
}

#[derive(Debug)]
struct AnyInstruction {
    opcode: u8,
    input_a: u8,
    input_b: u8,
    output_register: u8,
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

named!(parse_register_or_value<&str, u8>,
    do_parse!(
        n: parse_number >>
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
        input_a: parse_number >>
        tag!(" ") >>
        input_b: parse_number >>
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
            execute_instruction(
                &Instruction::from_any_instruction(opcode, &sample.instruction),
                &mut result_registers,
            );
            result_registers == sample.register_after
        })
        .map(|opcode| opcode.clone())
        .collect()
}

fn get_mapping_from_samples(samples: &Vec<Sample>) -> Vec<Opcode> {
    let possible_matches: Vec<(u8, Vec<Opcode>)> = samples
        .iter()
        .map(|s| (s.instruction.opcode, get_matching_opcodes(s)))
        .collect();
    let mut mapping = vec![Vec::new(); 16];
    for (opcode_id, opcode_matches) in possible_matches {
        let old_matches = &mapping[opcode_id as usize];
        if old_matches.len() == 0 {
            mapping[opcode_id as usize] = opcode_matches;
        } else {
            mapping[opcode_id as usize] = old_matches
                .iter()
                .filter(|m| opcode_matches.contains(m))
                .map(|c| c.clone())
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

fn level_2(input: &Input) -> ACResult<usize> {
    let mapping = get_mapping_from_samples(&input.samples);

    let mut registers = Registers::empty();

    for instr in input.instructions.iter() {
        execute_instruction(
            &Instruction::from_any_instruction(&mapping[instr.opcode as usize], &instr),
            &mut registers,
        );
    }

    Ok(*registers.get(0))
}
