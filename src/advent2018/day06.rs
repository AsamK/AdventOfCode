use crate::errors::{ACResult, Error};
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(crate::utils::read_lines(data)?).map(|r| r.to_string()),
        2 => level_2(crate::utils::read_lines(data)?).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

struct Point {
    i: usize,
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Field {
    pub inner: Vec<usize>,
    width: usize,
    height: usize,
}

impl Field {
    fn new(width: usize, height: usize) -> Self {
        let fab = vec![0; width * height];
        Field {
            inner: fab,
            width,
            height,
        }
    }

    fn get(&self, x: usize, y: usize) -> usize {
        self.inner[x + y * self.width]
    }

    fn set(&mut self, x: usize, y: usize, value: usize) {
        self.inner[x + y * self.width] = value;
    }

    fn remove_type(&mut self, value: usize) {
        for i in 0..self.inner.len() {
            if self.inner[i] == value {
                self.inner[i] = 0;
            }
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("{:?}", self.inner);
        for y in 0..self.height {
            let res: String = (0..self.width)
                .map(|i| self.get(i, y).to_string())
                .collect::<Vec<_>>()
                .join(" ");
            println!("{}", res);
        }
    }
}

fn level_1(lines: Vec<String>) -> ACResult<usize> {
    let infos = lines
        .iter()
        .enumerate()
        .map(|(i, line)| -> ACResult<_> {
            let s: Vec<&str> = line.split(", ").collect();
            let x = s[0].parse().unwrap();
            let y = s[1].parse().unwrap();
            Ok(Point { i: i + 1, x, y })
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| Error::new_str("Failed to parse line"))?;

    let max_width = 4 * (infos.iter().max_by_key(|i| i.x).unwrap().x + 1);
    let max_height = 4 * (infos.iter().max_by_key(|i| i.y).unwrap().y + 1);
    let mut field = Field::new(max_width, max_height);
    for x in 0..max_width {
        for y in 0..max_height {
            if let Some(i) = get_nearest(
                &infos,
                x as isize,
                y as isize,
                max_width / 2,
                max_height / 2,
            ) {
                field.set(x, y, i);
            } else {
                field.set(x, y, 0);
            }
        }
    }

    // Exclude indefinite sizes
    for x in 0..max_width {
        let i = field.get(x, 0);
        if i != 0 {
            field.remove_type(i);
        }
        let i = field.get(x, max_height - 1);
        if i != 0 {
            field.remove_type(i);
        }
    }
    for y in 0..max_height {
        let i = field.get(0, y);
        if i != 0 {
            field.remove_type(i);
        }
        let i = field.get(max_width - 1, y);
        if i != 0 {
            field.remove_type(i);
        }
    }
    let largest = infos
        .iter()
        .max_by_key(|p| field.inner.iter().filter(|f| **f == p.i).count())
        .unwrap();

    let max = field.inner.iter().filter(|f| **f == largest.i).count();
    Ok(max)
}

fn manhatten(x1: isize, y1: isize, x2: isize, y2: isize) -> usize {
    ((if x1 > x2 { x1 - x2 } else { x2 - x1 }) + (if y1 > y2 { y1 - y2 } else { y2 - y1 })).abs()
        as usize
}

fn get_nearest(
    infos: &Vec<Point>,
    x: isize,
    y: isize,
    x_offset: usize,
    y_offset: usize,
) -> Option<usize> {
    let mut points: Vec<_> = infos
        .iter()
        .map(|p| {
            (
                p,
                manhatten(x, y, (p.x + x_offset) as isize, (p.y + y_offset) as isize),
            )
        })
        .collect();
    points.sort_unstable_by_key(|&(_, size)| size);
    if points[0].1 == points[1].1 {
        return None;
    }
    Some(points[0].0.i)
}

fn level_2(lines: Vec<String>) -> ACResult<usize> {
    let infos = lines
        .iter()
        .enumerate()
        .map(|(i, line)| -> ACResult<_> {
            let s: Vec<&str> = line.split(", ").collect();
            let x = s[0].parse().unwrap();
            let y = s[1].parse().unwrap();
            Ok(Point { i: i + 1, x, y })
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| Error::new_str("Failed to parse line"))?;

    let max_width = 4 * (infos.iter().max_by_key(|i| i.x).unwrap().x + 1);
    let max_height = 4 * (infos.iter().max_by_key(|i| i.y).unwrap().y + 1);
    let mut field = Field::new(max_width, max_height);
    let mut count = 0;
    for x in 0..max_width {
        for y in 0..max_height {
            let sum = infos
                .iter()
                .map(|p| {
                    manhatten(
                        x as isize,
                        y as isize,
                        (p.x + max_width / 2) as isize,
                        (p.y + max_height / 2) as isize,
                    )
                })
                .sum();
            field.set(x, y, sum);
            if sum < 10000 {
                count += 1;
            }
        }
    }

    Ok(count)
}
