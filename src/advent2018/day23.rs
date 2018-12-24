use crate::errors::{ACResult, Error};
use nom::{call, complete, do_parse, error_position, map, named, tag, take_while};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(crate::utils::read_lines(data)?).map(|r| r.to_string()),
        2 => level_2(crate::utils::read_lines(data)?).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

impl Position {
    fn dist(&self, p: &Position) -> i64 {
        (self.x - p.x).abs() + (self.y - p.y).abs() + (self.z - p.z).abs()
    }

    fn dist_i(&self, x: i64, y: i64, z: i64) -> i64 {
        (self.x - x).abs() + (self.y - y).abs() + (self.z - z).abs()
    }

    fn div(&self, div: i64) -> Position {
        Position {
            x: self.x / div,
            y: self.y / div,
            z: self.z / div,
        }
    }
}

#[derive(Debug)]
struct Bot {
    position: Position,
    range: i64,
}

named!(
    number<nom::types::CompleteStr<'_>, i64>,
    map!(
        complete!(take_while!(|c: char| c == '-'
            || c == ' '
            || c.is_digit(10))),
        |s| s.trim().parse().unwrap()
    )
);

// pos=<9,-1>, r=432
named!(
    parse_bot<nom::types::CompleteStr<'_>, Bot>,
    do_parse!(
        tag!("pos=<")
            >> x: number
            >> tag!(",")
            >> y: number
            >> tag!(",")
            >> z: number
            >> tag!(">, r=")
            >> range: number
            >> (Bot {
                position: Position { x, y, z },
                range
            })
    )
);

fn level_1(lines: Vec<String>) -> ACResult<usize> {
    let bots: Vec<_> = lines
        .iter()
        .map(|l| parse_bot(nom::types::CompleteStr(l)).unwrap().1)
        .collect();

    let max_bot = bots.iter().max_by_key(|b| b.range).unwrap();

    let bots_in_range = bots
        .iter()
        .filter(|bot| max_bot.position.dist(&bot.position) <= max_bot.range)
        .count();

    Ok(bots_in_range)
}

fn level_2(lines: Vec<String>) -> ACResult<i64> {
    let bots: Vec<_> = lines
        .iter()
        .map(|l| parse_bot(nom::types::CompleteStr(l)).unwrap().1)
        .collect();

    let min = Position {
        x: bots.iter().min_by_key(|b| b.position.x).unwrap().position.x,
        y: bots.iter().min_by_key(|b| b.position.y).unwrap().position.y,
        z: bots.iter().max_by_key(|b| b.position.z).unwrap().position.z,
    };
    let max = Position {
        x: bots.iter().max_by_key(|b| b.position.x).unwrap().position.x,
        y: bots.iter().max_by_key(|b| b.position.y).unwrap().position.y,
        z: bots.iter().max_by_key(|b| b.position.z).unwrap().position.z,
    };

    let mut heap = BinaryHeap::new();
    {
        let mut div: i64 = 1;
        while div < max.x - min.x {
            div *= 2;
        }
        heap.push(Block {
            min: min.div(div),
            max: max.div(div),
            overlap_count: bots.len(),
            div,
        });
    }

    let mut largest = None;
    let mut largest_points = Vec::new();
    while let Some(block) = heap.pop() {
        if let Some(l) = largest {
            if l > block.overlap_count {
                break;
            }
        }
        if block.div == 0 {
            largest = Some(block.overlap_count);
            largest_points.push(block.min);
            continue;
        }
        let bots: Vec<_> = bots
            .iter()
            .map(|b| Bot {
                position: b.position.div(block.div),
                range: b.range / block.div,
            })
            .collect();
        for x in block.min.x..=block.max.x {
            for y in block.min.y..=block.max.y {
                for z in block.min.z..=block.max.z {
                    let overlap_count = bots
                        .iter()
                        .filter(|b| b.position.dist_i(x, y, z) <= b.range)
                        .count();

                    if block.div == 1 {
                        heap.push(Block {
                            min: Position { x, y, z },
                            max: Position { x, y, z },
                            overlap_count,
                            div: 0,
                        })
                    } else {
                        heap.push(Block {
                            min: Position {
                                x: (x - 1) * 2,
                                y: (y - 1) * 2,
                                z: (z - 1) * 2,
                            },
                            max: Position {
                                x: (x + 1) * 2,
                                y: (y + 1) * 2,
                                z: (z + 1) * 2,
                            },
                            overlap_count,
                            div: block.div / 2,
                        })
                    }
                }
            }
        }
    }
    let result = largest_points
        .iter()
        .min_by_key(|p| p.dist_i(0, 0, 0))
        .unwrap();

    Ok(result.dist_i(0, 0, 0))
}

#[derive(PartialEq, Eq, Debug)]
struct Block {
    min: Position,
    max: Position,
    overlap_count: usize,
    div: i64,
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Block) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Block {
    fn cmp(&self, other: &Block) -> Ordering {
        self.overlap_count.cmp(&other.overlap_count)
    }
}
