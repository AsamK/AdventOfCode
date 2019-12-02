use crate::errors::{ACResult, Error};
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&crate::utils::read_lines(data)?[0]).map(|r| r.to_string()),
        2 => level_2(&crate::utils::read_lines(data)?[0]).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

struct IntCodeComputer {
    memory: Vec<usize>,
    pos: usize,
}

impl IntCodeComputer {
    #[allow(dead_code)]
    fn new(memory: &[usize]) -> Self {
        IntCodeComputer {
            memory: Vec::from(memory),
            pos: 0,
        }
    }

    fn new_fixed(memory: &[usize], noun: usize, verb: usize) -> Self {
        let mut memory = Vec::from(memory);
        memory[1] = noun;
        memory[2] = verb;
        IntCodeComputer { memory, pos: 0 }
    }

    fn compute(mut self) -> ACResult<usize> {
        loop {
            match self.memory[self.pos] {
                1 => {
                    let (i1, i2, o) = (
                        self.memory[self.pos + 1],
                        self.memory[self.pos + 2],
                        self.memory[self.pos + 3],
                    );
                    self.memory[o] = self.memory[i1] + self.memory[i2];
                    self.pos += 4;
                }
                2 => {
                    let (i1, i2, o) = (
                        self.memory[self.pos + 1],
                        self.memory[self.pos + 2],
                        self.memory[self.pos + 3],
                    );
                    self.memory[o] = self.memory[i1] * self.memory[i2];
                    self.pos += 4;
                }
                99 => {
                    break;
                }
                value => {
                    return Err(Error::new(format!(
                        "Invalid machine state at pos {}: {}",
                        self.pos, value
                    )))
                }
            }
        }
        Ok(self.memory[0])
    }
}

fn parse_intcode(input: &str) -> ACResult<Vec<usize>> {
    input
        .split(',')
        .map(|c| {
            c.parse::<usize>()
                .map_err(|e| Error::new(format!("Invalid opcode: {}", e)))
        })
        .collect()
}

fn level_1(line: &str) -> ACResult<usize> {
    let ops = parse_intcode(line)?;
    let computer = IntCodeComputer::new_fixed(&ops, 12, 2);
    computer.compute()
}

fn level_2(line: &str) -> ACResult<usize> {
    let ops = parse_intcode(line)?;
    for i in 0..100 {
        for j in 0..100 {
            let computer = IntCodeComputer::new_fixed(&ops, i, j);
            let result = computer.compute()?;
            if result == 19_690_720 {
                return Ok(100 * i + j);
            }
        }
    }
    Err(Error::new_str("Failed to find input parameters"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_int_code_examples() {
        assert_eq!(
            IntCodeComputer::new(&parse_intcode("1,9,10,3,2,3,11,0,99,30,40,50").unwrap())
                .compute(),
            Ok(3500)
        );
        assert_eq!(
            IntCodeComputer::new(&parse_intcode("1,0,0,0,99").unwrap()).compute(),
            Ok(2)
        );
        assert_eq!(
            IntCodeComputer::new(&parse_intcode("2,3,0,3,99").unwrap()).compute(),
            Ok(2)
        );
        assert_eq!(
            IntCodeComputer::new(&parse_intcode("2,4,4,5,99,0").unwrap()).compute(),
            Ok(2)
        );
        assert_eq!(
            IntCodeComputer::new(&parse_intcode("1,1,1,4,99,5,6,0,99").unwrap()).compute(),
            Ok(30)
        );
    }
}
