use crate::errors::{ACResult, Error};
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&crate::utils::read_lines(data)?),
        2 => level_2(&crate::utils::read_lines(data)?),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

fn level_1(lines: &[String]) -> ACResult<String> {
    let serial: i32 = lines[0].parse().unwrap();

    let size = 300;

    let mut grid = Vec::new();

    for x in 1..=size {
        for y in 1..=size {
            let rack_id = x + 10;
            let power = (rack_id * y + serial) * rack_id;
            let power = (power / 100) % 10 - 5;
            grid.push(power);
        }
    }
    let grid = grid;
    let mut squares = Vec::new();
    for x in 1..=300 - 2 {
        for y in 1..=300 - 2 {
            let square_power = get(&grid, x, y)
                + get(&grid, x + 1, y)
                + get(&grid, x + 2, y)
                + get(&grid, x, y + 1)
                + get(&grid, x + 1, y + 1)
                + get(&grid, x + 2, y + 1)
                + get(&grid, x, y + 2)
                + get(&grid, x + 1, y + 2)
                + get(&grid, x + 2, y + 2);
            squares.push(square_power);
        }
    }
    let (i, _max) = squares.iter().enumerate().max_by_key(|&(_, l)| l).unwrap();

    let x = i % 298 + 1;
    let y = i / 298 + 1;

    Ok(format!("{},{}", x, y))
}

fn get(grid: &[i32], x: u32, y: u32) -> i32 {
    grid[((x - 1) + (y - 1) * 300) as usize]
}

#[allow(dead_code)]
fn print(grid: &[i32]) {
    for x in 0..5 {
        let mut line = "".to_owned();
        for y in 0..5 {
            line += &format!(" {}", grid[x + y * 300]);
        }
        println!("{}", line);
    }
}

fn level_2(lines: &[String]) -> ACResult<String> {
    let serial: i32 = lines[0].parse().unwrap();

    let size = 300;

    let mut grid = Vec::new();

    for x in 1..=size {
        for y in 1..=size {
            let rack_id = x + 10;
            let power = (rack_id * y + serial) * rack_id;
            let power = (power / 100) % 10 - 5;
            grid.push(power);
        }
    }
    let grid = grid;

    let mut max = i32::min_value();
    let mut result = "".to_owned();

    for s in 1..=300 {
        let mut squares = Vec::new();
        for x in 1..=300 - s + 1 {
            for y in 1..=300 - s + 1 {
                let mut square_power = 0;
                for sx in 0..s {
                    for sy in 0..s {
                        square_power += get(&grid, x + sx, y + sy);
                    }
                }
                squares.push(square_power);
            }
        }
        let (i, smax) = squares.iter().enumerate().max_by_key(|&(_, l)| l).unwrap();
        if *smax > max {
            max = *smax;
            let x = i as u32 % (300 - s + 1) + 1;
            let y = i as u32 / (300 - s + 1) + 1;
            result = format!("{},{},{}", x, y, s);
        }
    }

    Ok(result)
}
