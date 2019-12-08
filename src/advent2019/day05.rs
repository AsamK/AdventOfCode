use crate::errors::{ACResult, Error};
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&crate::utils::read_line(data)?).map(|r| r.to_string()),
        2 => level_2(&crate::utils::read_line(data)?).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

enum Opcode {
    Add(isize, isize, usize),
    Multiply(isize, isize, usize),
    Input(usize),
    Output(isize),
    JumpIfTrue(isize, usize),
    JumpIfFalse(isize, usize),
    LessThan(isize, isize, usize),
    Equals(isize, isize, usize),
    Break,
}

struct Command {
    opcode: Opcode,
    command_length: usize,
}
struct IntCodeComputer {
    memory: Vec<isize>,
    pos: usize,
}

impl IntCodeComputer {
    #[allow(dead_code)]
    fn new(memory: &[isize]) -> Self {
        IntCodeComputer {
            memory: Vec::from(memory),
            pos: 0,
        }
    }

    fn parse_command(&self) -> ACResult<Command> {
        let params = self.memory[self.pos];
        let opcode = self.memory[self.pos] % 100;
        match opcode {
            1 => {
                let (i1, i2, o) = (
                    self.memory[self.pos + 1],
                    self.memory[self.pos + 2],
                    self.memory[self.pos + 3] as usize,
                );
                let (i1, i2) = (
                    if params / 100 % 10 == 0 {
                        self.memory[i1 as usize]
                    } else {
                        i1
                    },
                    if params / 1000 % 10 == 0 {
                        self.memory[i2 as usize]
                    } else {
                        i2
                    },
                );
                Ok(Command {
                    opcode: Opcode::Add(i1, i2, o),
                    command_length: 4,
                })
            }
            2 => {
                let (i1, i2, o) = (
                    self.memory[self.pos + 1],
                    self.memory[self.pos + 2],
                    self.memory[self.pos + 3] as usize,
                );
                let (i1, i2) = (
                    if params / 100 % 10 == 0 {
                        self.memory[i1 as usize]
                    } else {
                        i1
                    },
                    if params / 1000 % 10 == 0 {
                        self.memory[i2 as usize]
                    } else {
                        i2
                    },
                );
                Ok(Command {
                    opcode: Opcode::Multiply(i1, i2, o),
                    command_length: 4,
                })
            }
            3 => {
                let o = self.memory[self.pos + 1] as usize;
                Ok(Command {
                    opcode: Opcode::Input(o),
                    command_length: 2,
                })
            }
            4 => {
                let i = self.memory[self.pos + 1];
                let i = if params / 100 % 10 == 0 {
                    self.memory[i as usize]
                } else {
                    i
                };
                Ok(Command {
                    opcode: Opcode::Output(i),
                    command_length: 2,
                })
            }
            5 => {
                let (i1, i2) = (self.memory[self.pos + 1], self.memory[self.pos + 2]);
                let (i1, i2) = (
                    if params / 100 % 10 == 0 {
                        self.memory[i1 as usize]
                    } else {
                        i1
                    },
                    if params / 1000 % 10 == 0 {
                        self.memory[i2 as usize] as usize
                    } else {
                        i2 as usize
                    },
                );
                Ok(Command {
                    opcode: Opcode::JumpIfTrue(i1, i2),
                    command_length: 3,
                })
            }
            6 => {
                let (i1, i2) = (self.memory[self.pos + 1], self.memory[self.pos + 2]);
                let (i1, i2) = (
                    if params / 100 % 10 == 0 {
                        self.memory[i1 as usize]
                    } else {
                        i1
                    },
                    if params / 1000 % 10 == 0 {
                        self.memory[i2 as usize] as usize
                    } else {
                        i2 as usize
                    },
                );
                Ok(Command {
                    opcode: Opcode::JumpIfFalse(i1, i2),
                    command_length: 3,
                })
            }
            7 => {
                let (i1, i2, o) = (
                    self.memory[self.pos + 1],
                    self.memory[self.pos + 2],
                    self.memory[self.pos + 3] as usize,
                );
                let (i1, i2) = (
                    if params / 100 % 10 == 0 {
                        self.memory[i1 as usize]
                    } else {
                        i1
                    },
                    if params / 1000 % 10 == 0 {
                        self.memory[i2 as usize]
                    } else {
                        i2
                    },
                );
                Ok(Command {
                    opcode: Opcode::LessThan(i1, i2, o),
                    command_length: 4,
                })
            }
            8 => {
                // equals
                let (i1, i2, o) = (
                    self.memory[self.pos + 1],
                    self.memory[self.pos + 2],
                    self.memory[self.pos + 3] as usize,
                );
                let (i1, i2) = (
                    if params / 100 % 10 == 0 {
                        self.memory[i1 as usize]
                    } else {
                        i1
                    },
                    if params / 1000 % 10 == 0 {
                        self.memory[i2 as usize]
                    } else {
                        i2
                    },
                );
                Ok(Command {
                    opcode: Opcode::Equals(i1, i2, o),
                    command_length: 4,
                })
            }
            99 => Ok(Command {
                opcode: Opcode::Break,
                command_length: 1,
            }),
            value => Err(Error::new(format!(
                "Invalid machine state at pos {}: {}",
                self.pos, value
            ))),
        }
    }

    fn compute(mut self, input: isize) -> ACResult<Vec<isize>> {
        let mut outputs = Vec::new();
        loop {
            let Command {
                opcode,
                command_length,
            } = self.parse_command()?;
            match opcode {
                Opcode::Add(i1, i2, o) => {
                    self.memory[o] = i1 + i2;
                    self.pos += command_length;
                }
                Opcode::Multiply(i1, i2, o) => {
                    self.memory[o] = i1 * i2;
                    self.pos += command_length;
                }
                Opcode::Input(o) => {
                    self.memory[o] = input;
                    self.pos += command_length;
                }
                Opcode::Output(i) => {
                    outputs.push(i);
                    self.pos += command_length;
                }
                Opcode::JumpIfTrue(i1, i2) => {
                    if i1 != 0 {
                        self.pos = i2;
                    } else {
                        self.pos += command_length;
                    }
                }
                Opcode::JumpIfFalse(i1, i2) => {
                    if i1 == 0 {
                        self.pos = i2;
                    } else {
                        self.pos += command_length;
                    }
                }
                Opcode::LessThan(i1, i2, o) => {
                    self.memory[o] = if i1 < i2 { 1 } else { 0 };
                    self.pos += command_length;
                }
                Opcode::Equals(i1, i2, o) => {
                    self.memory[o] = if i1 == i2 { 1 } else { 0 };
                    self.pos += command_length;
                }
                Opcode::Break => {
                    break;
                }
            }
        }
        Ok(outputs)
    }
}

fn parse_intcode(input: &str) -> ACResult<Vec<isize>> {
    input
        .split(',')
        .map(|c| {
            c.parse::<isize>()
                .map_err(|e| Error::new(format!("Invalid opcode: {}", e)))
        })
        .collect()
}

fn level_1(line: &str) -> ACResult<isize> {
    let ops = parse_intcode(line)?;
    let computer = IntCodeComputer::new(&ops);
    let outputs = computer.compute(1)?;
    let has_leading_non_zero = outputs
        .iter()
        .take(outputs.len() - 2)
        .filter(|o| **o != 0)
        .count()
        > 0;
    if has_leading_non_zero {
        return Err(Error::new_str("Validation failed"));
    }
    outputs
        .last()
        .copied()
        .ok_or_else(|| Error::new_str("Missing output"))
}

fn level_2(line: &str) -> ACResult<isize> {
    let ops = parse_intcode(line)?;
    let computer = IntCodeComputer::new(&ops);
    let outputs = computer.compute(5)?;
    if outputs.len() != 1 {
        return Err(Error::new_str("Invalid computation"));
    }
    outputs
        .get(0)
        .copied()
        .ok_or_else(|| Error::new_str("Missing output"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_level_2_examples() {
        let program ="3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let computer = IntCodeComputer::new(&parse_intcode(program).unwrap());
        assert_eq!(computer.compute(7), Ok(vec![999]));

        let computer = IntCodeComputer::new(&parse_intcode(program).unwrap());
        assert_eq!(computer.compute(8), Ok(vec![1000]));

        let computer = IntCodeComputer::new(&parse_intcode(program).unwrap());
        assert_eq!(computer.compute(9), Ok(vec![1001]));
    }
}
