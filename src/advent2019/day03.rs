use crate::errors::{ACResult, Error};
use std::collections::{HashMap, HashSet};
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&crate::utils::read_lines(data)?).map(|r| r.to_string()),
        2 => level_2(&crate::utils::read_lines(data)?).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

struct Instruction {
    direction: Direction,
    count: usize,
}

fn parse_instruction(input: &str) -> Instruction {
    let d = match input.chars().nth(0).unwrap() {
        'R' => Direction::RIGHT,
        'L' => Direction::LEFT,
        'U' => Direction::UP,
        'D' => Direction::DOWN,
        _ => panic!("invalid direction"),
    };
    let count = input
        .chars()
        .skip(1)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    Instruction {
        direction: d,
        count,
    }
}

fn level_1(lines: &[String]) -> ACResult<usize> {
    let mut wires = Vec::new();
    for l in lines.iter() {
        let (mut x, mut y): (isize, isize) = (0, 0);
        let mut wire = HashSet::new();
        for s in l.split(',') {
            let s = parse_instruction(s);
            match s.direction {
                Direction::RIGHT => {
                    for _px in 0..s.count {
                        x += 1;
                        wire.insert((x, y));
                    }
                }
                Direction::LEFT => {
                    for _px in 0..s.count {
                        x -= 1;
                        wire.insert((x, y));
                    }
                }
                Direction::UP => {
                    for _py in 0..s.count {
                        y -= 1;
                        wire.insert((x, y));
                    }
                }
                Direction::DOWN => {
                    for _py in 0..s.count {
                        y += 1;
                        wire.insert((x, y));
                    }
                }
            }
        }
        wires.push(wire);
    }
    let mut smallest = None;
    for (i, w1) in wires.iter().enumerate() {
        for w2 in wires.iter().skip(i + 1) {
            for i in w1.intersection(w2) {
                let dist = i.0.abs() + i.1.abs();
                if smallest.is_none() || smallest.unwrap() > dist {
                    smallest = Some(dist);
                }
            }
        }
    }
    Ok(smallest.unwrap().abs() as usize)
}

fn level_2(lines: &[String]) -> ACResult<usize> {
    let mut wires = Vec::new();
    for l in lines.iter() {
        let (mut x, mut y): (isize, isize) = (0, 0);
        let mut wire = HashMap::new();
        let mut i = 0usize;
        for s in l.split(',') {
            let s = parse_instruction(s);
            match s.direction {
                Direction::RIGHT => {
                    for _px in 0..s.count {
                        x += 1;
                        i += 1;
                        wire.entry((x, y)).or_insert(i);
                    }
                }
                Direction::LEFT => {
                    for _px in 0..s.count {
                        x -= 1;
                        i += 1;
                        wire.entry((x, y)).or_insert(i);
                    }
                }
                Direction::UP => {
                    for _py in 0..s.count {
                        y -= 1;
                        i += 1;
                        wire.entry((x, y)).or_insert(i);
                    }
                }
                Direction::DOWN => {
                    for _py in 0..s.count {
                        y += 1;
                        i += 1;
                        wire.entry((x, y)).or_insert(i);
                    }
                }
            }
        }
        wires.push(wire);
    }
    let mut smallest = None;
    for (i, w1m) in wires.iter().enumerate() {
        let mut w1 = HashSet::new();
        w1m.keys().for_each(|k| {
            w1.insert(k);
        });
        for w2m in wires.iter().skip(i + 1) {
            let mut w2 = HashSet::new();
            w2m.keys().for_each(|k| {
                w2.insert(k);
            });
            for i in w1.intersection(&w2) {
                let dist = w1m.get(i).unwrap() + w2m.get(i).unwrap();
                if smallest.is_none() || smallest.unwrap() > dist {
                    smallest = Some(dist);
                }
            }
        }
    }
    Ok(smallest.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_level_1_examples() {
        assert_eq!(
            level_1(&["R8,U5,L5,D3".to_owned(), "U7,R6,D4,L4".to_owned()]),
            Ok(6)
        );
        assert_eq!(
            level_1(&[
                "R75,D30,R83,U83,L12,D49,R71,U7,L72".to_owned(),
                "U62,R66,U55,R34,D71,R55,D58,R83".to_owned()
            ]),
            Ok(159)
        );
        assert_eq!(
            level_1(&[
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_owned(),
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_owned()
            ]),
            Ok(135)
        );
    }

    #[test]
    fn run_level_2_examples() {
        assert_eq!(
            level_2(&["R8,U5,L5,D3".to_owned(), "U7,R6,D4,L4".to_owned()]),
            Ok(30)
        );
        assert_eq!(
            level_2(&[
                "R75,D30,R83,U83,L12,D49,R71,U7,L72".to_owned(),
                "U62,R66,U55,R34,D71,R55,D58,R83".to_owned()
            ]),
            Ok(610)
        );
        assert_eq!(
            level_2(&[
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_owned(),
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_owned()
            ]),
            Ok(410)
        );
    }
}
