use crate::errors::{ACResult, Error};
use std::collections::HashMap;
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&crate::utils::read_line(data)?).map(|r| r.to_string()),
        2 => level_2(&crate::utils::read_line(data)?).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

#[derive(Debug)]
enum Opcode {
    Add(i64, i64, usize),
    Multiply(i64, i64, usize),
    Input(usize),
    Output(i64),
    JumpIfTrue(i64, usize),
    JumpIfFalse(i64, usize),
    LessThan(i64, i64, usize),
    Equals(i64, i64, usize),
    Offset(i64),
    Break,
}

struct Command {
    opcode: Opcode,
    command_length: usize,
}

struct IntCodeComputer {
    memory: HashMap<usize, i64>,
    pos: usize,
    relative_base: usize,
}

impl IntCodeComputer {
    fn new(ops: &[i64]) -> Self {
        let mut memory = HashMap::new();
        for (i, m) in ops.iter().enumerate() {
            memory.insert(i, *m);
        }
        IntCodeComputer {
            memory,
            pos: 0,
            relative_base: 0,
        }
    }

    fn read_memory(&self, pos: usize) -> i64 {
        *self.memory.get(&pos).unwrap_or(&0)
    }

    fn get_mode(&self, parameter_i: usize) -> u8 {
        let params = self.read_memory(self.pos);

        let mode = match parameter_i {
            0 => params / 100 % 10,
            1 => params / 1000 % 10,
            2 => params / 10000 % 10,
            _ => panic!("Inavlid parameter index"),
        };
        mode as u8
    }

    fn get_output(&self, parameter_i: usize) -> usize {
        let i = self.read_memory(self.pos + 1 + parameter_i);
        match self.get_mode(parameter_i) {
            0 => i as usize,
            2 => (self.relative_base as i64 + i) as usize,
            _ => panic!("Invalid mode"),
        }
    }

    fn get_input(&self, parameter_i: usize) -> i64 {
        let i = self.read_memory(self.pos + 1 + parameter_i);
        match self.get_mode(parameter_i) {
            0 => self.read_memory(i as usize),
            1 => i,
            2 => self.read_memory((self.relative_base as i64 + i) as usize),
            _ => panic!("Invalid mode"),
        }
    }

    fn parse_command(&self) -> ACResult<Command> {
        let opcode = self.read_memory(self.pos) % 100;
        match opcode {
            1 => {
                let (i1, i2, o) = (self.get_input(0), self.get_input(1), self.get_output(2));
                Ok(Command {
                    opcode: Opcode::Add(i1, i2, o),
                    command_length: 4,
                })
            }
            2 => {
                let (i1, i2, o) = (self.get_input(0), self.get_input(1), self.get_output(2));
                Ok(Command {
                    opcode: Opcode::Multiply(i1, i2, o),
                    command_length: 4,
                })
            }
            3 => {
                let o = self.get_output(0);
                Ok(Command {
                    opcode: Opcode::Input(o),
                    command_length: 2,
                })
            }
            4 => {
                let i = self.get_input(0);
                Ok(Command {
                    opcode: Opcode::Output(i),
                    command_length: 2,
                })
            }
            5 => {
                let (i1, i2) = (self.get_input(0), self.get_input(1) as usize);
                Ok(Command {
                    opcode: Opcode::JumpIfTrue(i1, i2),
                    command_length: 3,
                })
            }
            6 => {
                let (i1, i2) = (self.get_input(0), self.get_input(1) as usize);
                Ok(Command {
                    opcode: Opcode::JumpIfFalse(i1, i2),
                    command_length: 3,
                })
            }
            7 => {
                let (i1, i2, o) = (self.get_input(0), self.get_input(1), self.get_output(2));
                Ok(Command {
                    opcode: Opcode::LessThan(i1, i2, o),
                    command_length: 4,
                })
            }
            8 => {
                // equals
                let (i1, i2, o) = (self.get_input(0), self.get_input(1), self.get_output(2));
                Ok(Command {
                    opcode: Opcode::Equals(i1, i2, o),
                    command_length: 4,
                })
            }
            9 => {
                let i = self.get_input(0);
                Ok(Command {
                    opcode: Opcode::Offset(i),
                    command_length: 2,
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

    fn compute(&mut self, inputs: &[i64]) -> ACResult<Vec<i64>> {
        let mut outputs = Vec::new();
        let mut input_index = 0;
        self.compute_thread(
            &mut || {
                let i = inputs[input_index];
                input_index += 1;
                i
            },
            &mut |o| outputs.push(o),
        )?;
        Ok(outputs)
    }

    fn compute_thread(
        &mut self,
        input: &mut dyn FnMut() -> i64,
        output: &mut dyn FnMut(i64) -> (),
    ) -> ACResult<()> {
        loop {
            let Command {
                opcode,
                command_length,
            } = self.parse_command()?;
            match opcode {
                Opcode::Add(i1, i2, o) => {
                    self.memory.insert(o, i1 + i2);
                    self.pos += command_length;
                }
                Opcode::Multiply(i1, i2, o) => {
                    self.memory.insert(o, i1 * i2);
                    self.pos += command_length;
                }
                Opcode::Input(o) => {
                    self.memory.insert(o, input());
                    self.pos += command_length;
                }
                Opcode::Output(i) => {
                    output(i);
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
                    self.memory.insert(o, if i1 < i2 { 1 } else { 0 });
                    self.pos += command_length;
                }
                Opcode::Equals(i1, i2, o) => {
                    self.memory.insert(o, if i1 == i2 { 1 } else { 0 });
                    self.pos += command_length;
                }
                Opcode::Offset(i) => {
                    self.relative_base = (self.relative_base as i64 + i) as usize;
                    self.pos += command_length;
                }
                Opcode::Break => {
                    break;
                }
            }
        }
        Ok(())
    }
}

fn parse_intcode(input: &str) -> ACResult<Vec<i64>> {
    input
        .split(',')
        .map(|c| {
            c.parse::<i64>()
                .map_err(|e| Error::new(format!("Invalid opcode: {}", e)))
        })
        .collect()
}

fn level_1(line: &str) -> ACResult<i64> {
    let ops = parse_intcode(line)?;
    let mut computer = IntCodeComputer::new(&ops);
    let output = computer.compute(&[1])?;
    if output.len() != 1 {
        return Err(Error::new(format!("Invalid output: {:?}", output)));
    }
    Ok(output[0])
}

fn level_2(line: &str) -> ACResult<i64> {
    let ops = parse_intcode(line)?;
    let mut computer = IntCodeComputer::new(&ops);
    let output = computer.compute(&[2])?;
    if output.len() != 1 {
        return Err(Error::new(format!("Invalid output: {:?}", output)));
    }
    Ok(output[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_level_1_examples() {
        let ops =
            parse_intcode("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99").unwrap();
        let mut computer = IntCodeComputer::new(&ops);
        let largest = computer.compute(&[]);
        assert_eq!(
            largest,
            Ok(vec![
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99
            ]),
        );
    }

    #[test]
    fn run_level_1_examples_2() {
        let ops = parse_intcode("1102,34915192,34915192,7,4,7,99,0").unwrap();
        let mut computer = IntCodeComputer::new(&ops);
        let largest = computer.compute(&[]);
        assert_eq!(largest, Ok(vec![1219070632396864]),);
    }

    #[test]
    fn run_level_1_examples_3() {
        let ops = parse_intcode("104,1125899906842624,99").unwrap();
        let mut computer = IntCodeComputer::new(&ops);
        let largest = computer.compute(&[]);
        assert_eq!(largest, Ok(vec![1125899906842624]),);
    }
}
