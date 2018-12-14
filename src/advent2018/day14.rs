use crate::errors::{ACResult, Error};
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&crate::utils::read_lines(data)?[0]),
        2 => level_2(&crate::utils::read_lines(data)?[0]).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

fn level_1(line: &str) -> ACResult<String> {
    let count: usize = line.parse().unwrap();

    let mut recipes: Vec<u8> = Vec::new();
    recipes.push(3);
    recipes.push(7);

    let mut elve_positions: Vec<usize> = Vec::new();
    elve_positions.push(0);
    elve_positions.push(1);

    loop {
        let sum: u64 = elve_positions.iter().map(|i| recipes[*i] as u64).sum();

        let mut new_recipes: Vec<u8> = sum
            .to_string()
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();

        recipes.append(&mut new_recipes);

        for p in elve_positions.iter_mut() {
            *p = (*p + (recipes[*p] + 1) as usize) % recipes.len();
        }

        if recipes.len() >= count + 10 {
            break;
        }
    }

    let result: String = recipes[count..count + 10]
        .iter()
        .map(|n| n.to_string())
        .collect();

    Ok(result)
}

fn level_2(line: &str) -> ACResult<usize> {
    let mut recipes: Vec<u8> = Vec::new();
    recipes.push(3);
    recipes.push(7);

    let mut elve_positions: Vec<usize> = Vec::new();
    elve_positions.push(0);
    elve_positions.push(1);
    let line: Vec<u8> = line
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();

    let mut next_search_i = 0;
    loop {
        let sum: u64 = elve_positions.iter().map(|i| recipes[*i] as u64).sum();

        let mut new_recipes: Vec<u8> = sum
            .to_string()
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();

        recipes.append(&mut new_recipes);

        for p in elve_positions.iter_mut() {
            *p = (*p + (recipes[*p] + 1) as usize) % recipes.len();
        }

        if recipes.len() < line.len() {
            continue;
        }

        'outer: for i in next_search_i..recipes.len() - line.len() {
            for j in 0..line.len() {
                if recipes[i + j] != line[j] {
                    continue 'outer;
                }
            }
            return Ok(i);
        }
        next_search_i = recipes.len() - line.len();
    }
}
