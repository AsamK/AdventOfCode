use crate::errors::{ACResult, Error};
use std::io::BufRead;
use std::sync::mpsc::channel;
use std::thread;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&crate::utils::read_lines(data)?[0]).map(|r| r.to_string()),
        2 => level_2(&crate::utils::read_lines(data)?[0]).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}
#[derive(Debug)]
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

    fn compute(&mut self, inputs: &[isize]) -> ACResult<Vec<isize>> {
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
        input: &mut dyn FnMut() -> isize,
        output: &mut dyn FnMut(isize) -> (),
    ) -> ACResult<()> {
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
                    self.memory[o] = input();
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
        Ok(())
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
    let mut permutations = Vec::<(isize, isize, isize, isize, isize)>::new();

    for a1 in 0..5 {
        for a2 in 0..5 {
            if a2 == a1 {
                continue;
            }
            for a3 in 0..5 {
                if a3 == a1 || a3 == a2 {
                    continue;
                }
                for a4 in 0..5 {
                    if a4 == a1 || a4 == a2 || a4 == a3 {
                        continue;
                    }
                    for a5 in 0..5 {
                        if a5 == a1 || a5 == a2 || a5 == a3 || a5 == a4 {
                            continue;
                        }
                        permutations.push((a1, a2, a3, a4, a5));
                    }
                }
            }
        }
    }
    let mut largest = None;
    for (p1, p2, p3, p4, p5) in permutations.iter() {
        let mut computer = IntCodeComputer::new(&ops);
        let o = computer.compute(&[*p1, 0])?[0];
        let mut computer = IntCodeComputer::new(&ops);
        let o = computer.compute(&[*p2, o])?[0];
        let mut computer = IntCodeComputer::new(&ops);
        let o = computer.compute(&[*p3, o])?[0];
        let mut computer = IntCodeComputer::new(&ops);
        let o = computer.compute(&[*p4, o])?[0];
        let mut computer = IntCodeComputer::new(&ops);
        let o = computer.compute(&[*p5, o])?[0];
        if let Some(l) = largest {
            if l < o {
                largest = Some(o);
            }
        } else {
            largest = Some(o);
        }
    }
    Ok(largest.unwrap())
}

fn level_2(line: &str) -> ACResult<isize> {
    let ops = parse_intcode(line)?;
    let mut permutations = Vec::<(isize, isize, isize, isize, isize)>::new();

    for a1 in 5..10 {
        for a2 in 5..10 {
            if a2 == a1 {
                continue;
            }
            for a3 in 5..10 {
                if a3 == a1 || a3 == a2 {
                    continue;
                }
                for a4 in 5..10 {
                    if a4 == a1 || a4 == a2 || a4 == a3 {
                        continue;
                    }
                    for a5 in 5..10 {
                        if a5 == a1 || a5 == a2 || a5 == a3 || a5 == a4 {
                            continue;
                        }
                        permutations.push((a1, a2, a3, a4, a5));
                    }
                }
            }
        }
    }

    let mut largest = None;
    for (p1, p2, p3, p4, p5) in permutations.iter() {
        let (tx1, rx1) = channel();
        let (tx2, rx2) = channel();
        let (tx3, rx3) = channel();
        let (tx4, rx4) = channel();
        let (tx5, rx5) = channel();

        tx1.send(*p1).unwrap();
        tx2.send(*p2).unwrap();
        tx3.send(*p3).unwrap();
        tx4.send(*p4).unwrap();
        tx5.send(*p5).unwrap();

        tx1.send(0).unwrap();

        let (result_tx, result_rx) = channel();
        let channels = vec![
            (tx2, rx1),
            (tx3, rx2),
            (tx4, rx3),
            (tx5, rx4),
            (result_tx, rx5),
        ];

        let mut threads = Vec::new();
        for (tx, rx) in channels.into_iter() {
            let ops = ops.clone();
            let thread = thread::spawn(move || {
                let mut computer = IntCodeComputer::new(&ops);
                computer
                    .compute_thread(&mut || rx.recv().unwrap(), &mut |o| tx.send(o).unwrap())
                    .unwrap();
            });
            threads.push(thread);
        }

        let mut output = 0;
        while let Ok(o) = result_rx.recv() {
            // Ignoring the send error here, because thread 1 could be already finished
            output = o;
            let _ignored = tx1.send(o);
        }

        for t in threads.into_iter() {
            t.join().unwrap();
        }

        if let Some(l) = largest {
            if l < output {
                largest = Some(output);
            }
        } else {
            largest = Some(output);
        }
    }
    Ok(largest.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_level_1_examples() {
        assert_eq!(
            level_1(&"3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0".to_owned()),
            Ok(43210),
        );
        assert_eq!(
            level_1(
                &"3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"
                    .to_owned()
            ),
            Ok(54321),
        );
        assert_eq!(
            level_1(&"3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0".to_owned()),
            Ok(65210),
        );
    }

    #[test]
    fn run_level_2_examples() {
        assert_eq!(level_2(&"3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"), Ok(139629729));
        assert_eq!(
            level_2(&"3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"),
            Ok(18216)
        );
    }
}
