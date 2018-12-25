use crate::errors::{ACResult, Error};
use nom::{call, complete, do_parse, error_position, flat_map, named, parse_to, tag, take_while};
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&crate::utils::read_lines(data)?).map(|r| r.to_string()),
        2 => level_2(&crate::utils::read_lines(data)?).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

struct Info {
    player_count: u32,
    last_marble_worth: u32,
}

named!(number<&str, u32>, flat_map!(complete!(take_while!(|c: char| {c.is_digit(10)})), parse_to!(u32)));

named!(info_line<&str, Info>,
  do_parse!(
    player_count: number >>
    tag!(" players; last marble is worth ") >>
    last_marble_worth: number >>
    tag!(" points") >>
    (Info { player_count, last_marble_worth })
  )
);

fn level_1(lines: &[String]) -> ACResult<u32> {
    let info = info_line(&lines[0]).unwrap().1;
    Ok(run_game(info.player_count, info.last_marble_worth))
}

fn level_2(lines: &[String]) -> ACResult<u32> {
    let info = info_line(&lines[0]).unwrap().1;
    Ok(run_game(info.player_count, info.last_marble_worth * 100))
}

fn run_game(player_count: u32, last_marble_worth: u32) -> u32 {
    let mut circle = Circle::new(0);
    let mut player: u32 = 0;
    let mut player_points = vec![0; player_count as usize];
    for i in 1..last_marble_worth {
        if i % 23 == 0 {
            // Special case
            player_points[player as usize] += i;

            circle.navigate_by(-7);
            let removed_marble = circle.remove();
            player_points[player as usize] += removed_marble;
        } else {
            // Normal case
            circle.navigate_by(1);
            circle.append(i);
        }
        player = (player + 1) % player_count;

        // circle.print();
    }
    let mut winners: Vec<_> = player_points.into_iter().enumerate().collect();
    winners.sort_by_key(|&(_, points)| points);
    let (_winner, points) = winners.last().unwrap();
    *points
}

struct Node {
    value: u32,
    prev: Option<*mut Node>,
    next: Option<*mut Node>,
}

impl Node {
    fn new(value: u32) -> Self {
        Node {
            value,
            prev: None,
            next: None,
        }
    }
}

struct Circle {
    current: Option<*mut Node>,
}

impl Circle {
    fn new(value: u32) -> Self {
        let n = Box::new(Node::new(value));
        Circle {
            current: Some(Box::into_raw(n)),
        }
    }

    fn navigate_by(&mut self, nodes: i32) {
        if let Some(current) = self.current {
            unsafe {
                if (*current).prev.is_none() {
                    return;
                }
                let mut current = current;
                for _ in 0..nodes.abs() {
                    if nodes < 0 {
                        current = (*current).prev.unwrap();
                    } else {
                        current = (*current).next.unwrap();
                    }
                }
                self.current = Some(current);
            }
        }
    }

    fn remove(&mut self) -> u32 {
        if let Some(current) = self.current {
            unsafe {
                let mut current = Box::from_raw(current);
                if current.prev == current.next {
                    if let Some(other) = current.prev {
                        (*other).next = None;
                        (*other).prev = None;
                    }
                } else {
                    if let Some(prev) = current.prev {
                        (*prev).next = current.next;
                    }
                    if let Some(next) = current.next {
                        (*next).prev = current.prev;
                    }
                }
                self.current = current.next;
                current.next = None;
                current.prev = None;
                current.value
            }
        } else {
            panic!("Circle has no more nodes");
        }
    }

    fn append(&mut self, value: u32) {
        let node = Box::into_raw(Box::new(Node::new(value)));
        if let Some(current) = self.current {
            unsafe {
                (*node).prev = Some(current);
                if let Some(next) = (*current).next {
                    (*node).next = Some(next);
                    (*next).prev = Some(node);
                } else {
                    (*node).next = Some(current);
                    (*current).prev = Some(node);
                }
                (*current).next = Some(node);
                self.current = (*current).next;
            }
        } else {
            self.current = Some(node);
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        let mut result = String::new();
        let mut cur = self.current;
        while let Some(c) = cur {
            result += " ";
            unsafe {
                result += &(*c).value.to_string();
                cur = (*c).next;
            }
            if cur.is_none() || cur == self.current {
                break;
            }
        }
        println!("{}", result);
    }
}

impl Drop for Circle {
    fn drop(&mut self) {
        if let Some(current) = self.current {
            let mut cur = self.current;
            while let Some(c) = cur {
                unsafe {
                    cur = (*c).next;
                    Box::from_raw(c);
                }
                if cur.is_none() || cur == self.current {
                    break;
                }
            }
            unsafe {
                Box::from_raw(current);
            }
        }
    }
}
