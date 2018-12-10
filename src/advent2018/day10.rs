use crate::errors::{ACResult, Error};
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(crate::utils::read_lines(data)?),
        2 => level_2(crate::utils::read_lines(data)?).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

#[derive(Debug)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Velocity {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Point {
    position: Position,
    velocity: Velocity,
}

named!(number<&str, i64>, map!(
    complete!(take_while!(|c: char| {c == '-' || c == ' '  || c.is_digit(10)})),
    |s| s.trim().parse().unwrap()
));

// position=< 9,  1> velocity=< 0,  2>
named!(info_line<&str, Point>,
  dbg!(do_parse!(
    tag!("position=<") >>
    x: number >>
    tag!(", ") >>
    y: number >>
    tag!("> velocity=<") >>
    vx: number >>
    tag!(", ") >>
    vy: number >>
    tag!(">") >>
    (Point {position: Position{x,y}, velocity: Velocity{x:vx,y:vy}})
  ))
);

fn level_1(lines: Vec<String>) -> ACResult<String> {
    let mut points: Vec<_> = lines.iter().map(|l| info_line(&l).unwrap().1).collect();

    for _ in 0..30000 {
        if possible_message(&points) {
            let field = assemble_points(&points);
            return Ok(field);
        }
        simulate_second(&mut points);
    }
    panic!("Not found");
}

fn simulate_second(points: &mut Vec<Point>) {
    for p in points.iter_mut() {
        p.position.x += p.velocity.x;
        p.position.y += p.velocity.y;
    }
}

fn possible_message(points: &Vec<Point>) -> bool {
    let mut xes = std::collections::HashMap::new();
    for p in points.iter() {
        let xe = xes.entry(p.position.x).or_insert(0);
        *xe += 1;
    }
    for (_, count) in xes.iter() {
        if *count > 20 {
            return true;
        }
    }
    false
}

fn assemble_points(points: &Vec<Point>) -> String {
    let mut offset_x = i64::max_value();
    let mut offset_y = i64::max_value();
    let mut width = 0;
    let mut height = 0;
    for p in points.iter() {
        if p.position.x > width {
            width = p.position.x;
        }
        if p.position.y > height {
            height = p.position.y;
        }
        if p.position.x < offset_x {
            offset_x = p.position.x
        }
        if p.position.y < offset_y {
            offset_y = p.position.y
        }
    }
    let width = (width - offset_x) as usize + 1;
    let height = (height - offset_y) as usize + 1;
    let mut field: Vec<u8> = vec![b'.'; (width + 1) * height];
    for y in 0..height {
        field[(y * (width + 1)) + width] = b'\n';
    }
    for p in points.iter() {
        let x = p.position.x - offset_x;
        let y = p.position.y - offset_y;
        if x >= width as i64 || y >= height as i64 {
            continue;
        }
        field[(x + y * (width as i64 + 1)) as usize] = b'#';
    }
    String::from_utf8(field).unwrap()
}

fn level_2(lines: Vec<String>) -> ACResult<u64> {
    let mut points: Vec<_> = lines.iter().map(|l| info_line(&l).unwrap().1).collect();

    for i in 0..30000 {
        if possible_message(&points) {
            return Ok(i);
        }
        simulate_second(&mut points);
    }
    panic!("Not found");
}
