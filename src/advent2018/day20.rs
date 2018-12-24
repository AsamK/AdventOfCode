use crate::errors::{ACResult, Error};
use nom::{
    alt, call, do_parse, error_position, many0, many1, map, named, opt, tag, take_while,
    take_while1,
};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&crate::utils::read_lines(data)?[0]).map(|r| r.to_string()),
        2 => level_2(&crate::utils::read_lines(data)?[0]).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl From<char> for Dir {
    fn from(c: char) -> Dir {
        match c {
            'N' => Dir::North,
            'E' => Dir::East,
            'S' => Dir::South,
            'W' => Dir::West,
            _ => panic!("Invalid direction"),
        }
    }
}

named!(parse_dir<&str, Vec<Dir>>,
    map!(
        take_while!(|c| { c=='N' || c=='E' || c=='S' || c=='W' }),
        |l| l.chars().map(|c| c.into()).collect()
    )
);

named!(parse_dir1<&str, Vec<Dir>>,
    map!(
        take_while1!(|c| { c=='N' || c=='E' || c=='S' || c=='W' }),
        |l| l.chars().map(|c| c.into()).collect()
    )
);

enum Dirs {
    Alt(Vec<Dirs>),
    List(Vec<Dirs>),
    Some(Vec<Dir>),
}

named!(parse_alt<&str, Dirs>,
    do_parse!(
        tag!("(") >>
        n: many0!(
            do_parse!(
                dirs: parse_sub >>
                opt!(tag!("|")) >>
                (dirs)
            )
        ) >>
        tag!(")") >>
        (Dirs::Alt(n))
    )
);

named!(parse_sub<&str, Dirs>,
    map!(
        many1!(alt!(
            parse_alt |
            map!(parse_dir1, |dirs| Dirs::Some(dirs))
        )),
        |d| Dirs::List(d)
    )
);

named!(parse_input<&str, Dirs>,
    do_parse!(
        tag!("^") >>
        subs: parse_sub >>
        tag!("$") >>
        (subs)
    )
);

#[derive(Clone, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn down(&self) -> Self {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn up(&self) -> Self {
        Point {
            x: self.x,
            y: self.y - 1,
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

#[derive(Debug, Clone, PartialEq)]
enum Type {
    Room,
    Door,
    Wall,
}

#[allow(dead_code)]
fn print_field(field: &Vec<Vec<Type>>) {
    for l in field.iter() {
        let mut skip = true;
        let res: String = l
            .iter()
            .map(|c| match c {
                Type::Wall => "#",
                Type::Room => {
                    skip = false;
                    "."
                }
                Type::Door => {
                    skip = false;
                    "|"
                }
            })
            .collect();
        if skip {
            continue;
        }
        println!("{}", res);
    }
}

#[derive(PartialEq, Eq)]
struct PartialPath {
    next_point: Point,
    // shortest path to this point
    dist: usize,
}

impl PartialOrd for PartialPath {
    fn partial_cmp(&self, other: &PartialPath) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for PartialPath {
    fn cmp(&self, other: &PartialPath) -> Ordering {
        match self.dist.cmp(&other.dist) {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
        }
    }
}

fn get_longest_shortest_path_room(field: &Vec<Vec<Type>>, pos: &Point) -> usize {
    let shortests = get_shortest_paths(field, pos);

    shortests
        .iter()
        .map(|l| {
            l.iter()
                .map(|x| match x {
                    None => 0,
                    Some(s) => *s,
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

fn get_shortest_paths(field: &Vec<Vec<Type>>, pos: &Point) -> Vec<Vec<Option<usize>>> {
    let mut shortests = vec![vec![None; field.len()]; field.len()];

    *get_mut(&mut shortests, pos) = Some(0);

    let mut partials = BinaryHeap::new();
    partials.push(PartialPath {
        next_point: pos.clone(),
        dist: 0,
    });

    while let Some(part) = partials.pop() {
        let p = part.next_point.up();
        let p2 = p.up();
        if *get(field, &p) == Type::Door && get_mut(&mut shortests, &p2).is_none() {
            let dist = part.dist + 1;
            *get_mut(&mut shortests, &p2) = Some(dist);
            partials.push(PartialPath {
                next_point: p2,
                dist,
            })
        }

        let p = part.next_point.down();
        let p2 = p.down();
        if *get(field, &p) == Type::Door && get_mut(&mut shortests, &p2).is_none() {
            let dist = part.dist + 1;
            *get_mut(&mut shortests, &p2) = Some(dist);
            partials.push(PartialPath {
                next_point: p2,
                dist,
            })
        }

        let p = part.next_point.left();
        let p2 = p.left();
        if *get(field, &p) == Type::Door && get_mut(&mut shortests, &p2).is_none() {
            let dist = part.dist + 1;
            *get_mut(&mut shortests, &p2) = Some(dist);
            partials.push(PartialPath {
                next_point: p2,
                dist,
            })
        }

        let p = part.next_point.right();
        let p2 = p.right();
        if *get(field, &p) == Type::Door && get_mut(&mut shortests, &p2).is_none() {
            let dist = part.dist + 1;
            *get_mut(&mut shortests, &p2) = Some(dist);
            partials.push(PartialPath {
                next_point: p2,
                dist,
            })
        }
    }
    shortests
}

fn get_next_positions(field: &mut Vec<Vec<Option<Type>>>, pos: &Point, dirs: &Dirs) -> Vec<Point> {
    match dirs {
        Dirs::Alt(alts) => alts
            .iter()
            .flat_map(|alt| get_next_positions(field, pos, alt))
            .collect(),
        Dirs::List(list) => list.iter().fold(vec![pos.clone()], |prev, list| {
            prev.iter()
                .flat_map(|p| get_next_positions(field, p, list))
                .collect()
        }),
        Dirs::Some(dirs) => vec![get_next_position(field, pos, dirs)],
    }
}

fn get_mut<'a, T>(field: &'a mut Vec<Vec<T>>, pos: &Point) -> &'a mut T {
    let offset = field.len() as isize / 2;
    &mut field[(pos.y + offset) as usize][(pos.x + offset) as usize]
}

fn get<'a, T>(field: &'a Vec<Vec<T>>, pos: &Point) -> &'a T {
    let offset = field.len() as isize / 2;
    &field[(pos.y + offset) as usize][(pos.x + offset) as usize]
}

fn get_next_position(field: &mut Vec<Vec<Option<Type>>>, pos: &Point, dirs: &[Dir]) -> Point {
    let mut pos = pos.clone();
    for d in dirs {
        match d {
            Dir::North => {
                let next_door = pos.up();
                let next_pos = next_door.up();
                *get_mut(field, &next_door) = Some(Type::Door);
                *get_mut(field, &next_pos) = Some(Type::Room);
                pos = next_pos;
            }
            Dir::East => {
                let next_door = pos.right();
                let next_pos = next_door.right();
                *get_mut(field, &next_door) = Some(Type::Door);
                *get_mut(field, &next_pos) = Some(Type::Room);
                pos = next_pos;
            }
            Dir::South => {
                let next_door = pos.down();
                let next_pos = next_door.down();
                *get_mut(field, &next_door) = Some(Type::Door);
                *get_mut(field, &next_pos) = Some(Type::Room);
                pos = next_pos;
            }
            Dir::West => {
                let next_door = pos.left();
                let next_pos = next_door.left();
                *get_mut(field, &next_door) = Some(Type::Door);
                *get_mut(field, &next_pos) = Some(Type::Room);
                pos = next_pos;
            }
        }
    }
    pos
}

fn discover_map(input: &Dirs, start: &Point) -> Vec<Vec<Type>> {
    let size = 1000;
    let mut field = vec![vec![None; size]; size];

    *get_mut(&mut field, &start) = Some(Type::Room);

    let _end_points = get_next_positions(&mut field, &start, &input);

    // Make all unknown fields into a wall
    field
        .into_iter()
        .map(|l| {
            l.into_iter()
                .map(|c| match c {
                    None => Type::Wall,
                    Some(x) => x,
                })
                .collect()
        })
        .collect()
}

fn level_1(line: &str) -> ACResult<usize> {
    let input = parse_input(line).unwrap().1;
    let start = Point { x: 0, y: 0 };

    let field = discover_map(&input, &start);

    let dist = get_longest_shortest_path_room(&field, &start);

    Ok(dist)
}

fn level_2(line: &str) -> ACResult<usize> {
    let input = parse_input(line).unwrap().1;
    let start = Point { x: 0, y: 0 };

    let field = discover_map(&input, &start);

    let shortests = get_shortest_paths(&field, &start);

    let sum: usize = shortests
        .iter()
        .map(|l| {
            l.iter()
                .filter(|x| match x {
                    None => false,
                    Some(s) => *s >= 1000,
                })
                .count()
        })
        .sum();

    Ok(sum)
}
