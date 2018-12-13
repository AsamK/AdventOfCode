use crate::errors::{ACResult, Error};
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&parse(&crate::utils::read_all(data).unwrap()).unwrap().1)
            .map(|r| r.to_string()),
        2 => level_2(&parse(&crate::utils::read_all(data).unwrap()).unwrap().1)
            .map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

struct Input {
    // true means a pot has a plant
    initial_state: Vec<bool>,
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
struct Instruction {
    pattern: Vec<bool>,
    new_state: bool,
}

named!(parse_state<&str, Vec<bool>>,
     map!(take_while!(|c: char| {c == '#' || c == '.'}), |c| c.chars().map(|c| c=='#').collect())
);

named!(parse_instruction<&str, Instruction>,
  do_parse!(
      pattern: parse_state >>
      tag!(" => ") >>
      new_state: take!(1) >>
      tag!("\n") >>
      (Instruction{ pattern, new_state: new_state == "#" })
  )
);

named!(parse<&str, Input>,
  do_parse!(
    tag!("initial state: ") >>
    initial_state: parse_state >> tag!("\n") >>
    tag!("\n") >>
    instructions: many1!(complete!(parse_instruction)) >>
    (Input { initial_state, instructions })
  )
);

#[derive(Clone)]
struct Tunnel {
    pots_left: Vec<bool>,
    // pot zero is in pots_right
    pots_right: Vec<bool>,
}

impl Tunnel {
    fn new(initial_state: &[bool]) -> Tunnel {
        let mut pots_right = vec![false; initial_state.len() * 2];
        let pots_left = vec![false; initial_state.len()];

        for (i, p) in initial_state.iter().enumerate() {
            pots_right[i] = *p;
        }
        Tunnel {
            pots_left,
            pots_right,
        }
    }

    fn get(&self, i: isize) -> bool {
        if i < 0 {
            self.pots_left[-i as usize - 1]
        } else {
            self.pots_right[i as usize]
        }
    }

    fn set(&mut self, i: isize, v: bool) {
        if i < 0 {
            self.pots_left[-i as usize - 1] = v;
        } else {
            self.pots_right[i as usize] = v;
        }
    }

    fn tick(&mut self, instructions: &[Instruction]) {
        let old_tunnel = self.clone();
        for i in -(self.pots_left.len() as isize) + 3..(self.pots_right.len() as isize - 2) {
            let inst = instructions.iter().find(|inst| {
                for j in 0..inst.pattern.len() {
                    if inst.pattern[j] != old_tunnel.get(i + j as isize - 2) {
                        return false;
                    }
                }
                return true;
            });
            if inst.is_none() {
                self.set(i, false);
                eprintln!("No instruction found ... removing the plant");
                continue;
            }
            self.set(i, inst.unwrap().new_state);
        }
        let len = self.pots_left.len();
        if self.pots_left[len - 3] || self.pots_left[len - 4] {
            self.pots_left.resize(len * 2, false);
        }
        let len = self.pots_right.len();
        if self.pots_right[len - 3] || self.pots_right[len - 4] {
            self.pots_right.resize(len * 2, false);
        }
    }

    fn sum(&self) -> isize {
        self.pots_left
            .iter()
            .enumerate()
            .map(|(i, p)| if *p { -(i as isize) - 1 } else { 0 })
            .sum::<isize>()
            + self
                .pots_right
                .iter()
                .enumerate()
                .map(|(i, p)| if *p { i as isize } else { 0 })
                .sum::<isize>()
    }

    #[allow(dead_code)]
    fn print_pots(&self) {
        let line: String = self
            .pots_left
            .iter()
            .chain(self.pots_right.iter())
            .map(|p| if *p { '#' } else { '.' })
            .collect();
        println!("{}", line);
    }
}

fn level_1(input: &Input) -> ACResult<isize> {
    let mut tunnel = Tunnel::new(&input.initial_state);

    for _ in 0..20 {
        tunnel.tick(&input.instructions);
    }
    let sum = tunnel.sum();
    Ok(sum)
}

fn level_2(input: &Input) -> ACResult<isize> {
    let mut tunnel = Tunnel::new(&input.initial_state);

    let part_iterations = 1000;
    let total_iterations: isize = 50_000_000_000;

    let mut p = 0;
    let mut n = 0;
    for _ in 0..=part_iterations {
        tunnel.tick(&input.instructions);

        p = n;
        n = tunnel.sum();
    }
    let sum = p + (n - p) * (total_iterations - part_iterations);
    Ok(sum)
}
