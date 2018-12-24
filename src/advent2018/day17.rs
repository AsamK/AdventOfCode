use crate::errors::{ACResult, Error};
use nom::{alt, call, complete, do_parse, error_position, map, named, tag, take_while1};
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&crate::utils::read_lines(data)?).map(|r| r.to_string()),
        2 => level_2(&crate::utils::read_lines(data)?).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

#[derive(Eq, PartialEq, Clone)]
enum Type {
    Sand,
    Clay,
    Water,
    WaterStill,
}

#[derive(Clone)]
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn down(&self) -> Self {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn left(&self) -> Self {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn right(&self) -> Self {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }
}

struct Field {
    fields: Vec<Type>,
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
    min_y_orig: usize,
    fountain_position: Point,
}

impl Field {
    fn new(scans: &[Scan], fountain_position: &Point) -> Self {
        let min_x = scans.iter().min_by_key(|s| s.x_from).unwrap().x_from - 1;
        let min_y_orig = scans.iter().min_by_key(|s| s.y_from).unwrap().y_from;
        let min_y = if min_y_orig > 0 { 0 } else { min_y_orig };
        let max_x = scans.iter().max_by_key(|s| s.x_to).unwrap().x_to + 1;
        let max_y = scans.iter().max_by_key(|s| s.y_to).unwrap().y_to;
        let fields = vec![Type::Sand; (max_x + 1 - min_x) * (max_y + 1 - min_y)];
        let mut field = Field {
            min_x,
            min_y_orig,
            min_y,
            max_x,
            max_y,
            fields,
            fountain_position: fountain_position.clone(),
        };
        for s in scans.iter() {
            for x in s.x_from..=s.x_to {
                for y in s.y_from..=s.y_to {
                    *field.get_mut(x, y) = Type::Clay;
                }
            }
        }
        field
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut Type {
        let ind = (x - self.min_x) + (y - self.min_y) * (self.max_x + 1 - self.min_x);
        &mut self.fields[ind]
    }

    fn get(&self, x: usize, y: usize) -> &Type {
        &self.fields[(x - self.min_x) + (y - self.min_y) * (self.max_x + 1 - self.min_x)]
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in self.min_y_orig..=self.max_y {
            let line: String = (self.min_x..=self.max_x)
                .map(|x| {
                    match self.get(x, y) {
                        Type::Sand => '.',
                        Type::Clay => '#',
                        Type::Water => '|',
                        Type::WaterStill => '~',
                    }
                    .to_string()
                        + " "
                })
                .collect();
            println!("{}", line);
        }
    }

    fn is_empty(&self, position: &Point) -> bool {
        let t = self.get(position.x, position.y);
        *t == Type::Sand || *t == Type::Water
    }

    fn is_any_water(&self, x: usize, y: usize) -> bool {
        let t = self.get(x, y);
        *t == Type::WaterStill || *t == Type::Water
    }

    fn is_still_water(&self, x: usize, y: usize) -> bool {
        let t = self.get(x, y);
        *t == Type::WaterStill
    }

    fn flow_water(&mut self) {
        self.down(&self.fountain_position.clone());
    }

    fn down(&mut self, pos: &Point) {
        // Go down as far as possible
        let mut stack = Vec::new();
        let mut current_pos = pos.clone();
        loop {
            if self.is_empty(&current_pos) {
                *self.get_mut(current_pos.x, current_pos.y) = Type::Water;
                if current_pos.y == self.max_y {
                    // Reached bottom, finished
                    return;
                }
                let next_pos = current_pos.down();
                stack.push(current_pos);
                current_pos = next_pos;
            } else {
                break;
            }
        }
        while let Some(pos) = stack.pop() {
            // Below this point is clay or fixed water
            let mut left_wall_x = None;
            let mut right_wall_x = None;
            let mut current_pos = pos.clone();
            loop {
                current_pos = current_pos.left();
                if !self.is_empty(&current_pos) {
                    left_wall_x = Some(current_pos.x);
                    break;
                }
                *self.get_mut(current_pos.x, current_pos.y) = Type::Water;
                let below = current_pos.down();
                if self.is_empty(&below) {
                    self.down(&below);
                    if self.is_empty(&below) {
                        // Still empty, flowing down ...
                        break;
                    }
                }
            }
            loop {
                current_pos = current_pos.right();
                if !self.is_empty(&current_pos) {
                    right_wall_x = Some(current_pos.x);
                    break;
                }
                *self.get_mut(current_pos.x, current_pos.y) = Type::Water;
                let below = current_pos.down();
                if self.is_empty(&below) {
                    self.down(&below);
                    if self.is_empty(&below) {
                        // Still empty, flowing down ...
                        break;
                    }
                }
            }
            if let Some(left_x) = left_wall_x {
                if let Some(right_x) = right_wall_x {
                    for x in (left_x + 1)..right_x {
                        *self.get_mut(x, pos.y) = Type::WaterStill;
                    }
                    continue;
                }
            }
            return;
        }
    }

    fn count_any_water(&self) -> usize {
        let mut count = 0;
        for x in self.min_x..=self.max_x {
            for y in self.min_y_orig..=self.max_y {
                if self.is_any_water(x, y) && !(x == 500 && y == 0) {
                    count += 1;
                }
            }
        }
        count
    }
    fn count_still_water(&self) -> usize {
        let mut count = 0;
        for x in self.min_x..=self.max_x {
            for y in self.min_y_orig..=self.max_y {
                if self.is_still_water(x, y) && !(x == 500 && y == 0) {
                    count += 1;
                }
            }
        }
        count
    }
}

struct Scan {
    x_from: usize,
    x_to: usize,
    y_from: usize,
    y_to: usize,
}

named!(parse_number<&str, usize>,
    complete!(map!(take_while1!(|c: char| c.is_numeric()), |c| c.to_string().parse().unwrap()))
);

named!(parse_line<&str, Scan>,
    alt!(
        do_parse!(
            tag!("x=") >>
            x: parse_number >>
            tag!(", y=") >>
            y_from: parse_number >>
            tag!("..") >>
            y_to: complete!(parse_number) >>
            (Scan{x_from:x, x_to:x, y_from, y_to})
        )
    |
        do_parse!(
            tag!("y=") >>
            y: parse_number >>
            tag!(", x=") >>
            x_from: parse_number >>
            tag!("..") >>
            x_to: complete!(parse_number) >>
            (Scan{y_from:y, y_to:y, x_from, x_to})
        )
    )
);

fn level_1(lines: &[String]) -> ACResult<usize> {
    let scans: Vec<Scan> = lines
        .iter()
        .map(|l| parse_line(&(l.to_string() + "\n")).unwrap().1)
        .collect();
    let mut field = Field::new(&scans, &Point { x: 500, y: 0 });
    // field.print();
    field.flow_water();
    // field.print();
    Ok(field.count_any_water())
}

fn level_2(lines: &[String]) -> ACResult<usize> {
    let scans: Vec<Scan> = lines
        .iter()
        .map(|l| parse_line(&(l.to_string() + "\n")).unwrap().1)
        .collect();
    let mut field = Field::new(&scans, &Point { x: 500, y: 0 });
    // field.print();
    field.flow_water();
    // field.print();
    Ok(field.count_still_water())
}
