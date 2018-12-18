use crate::errors::{ACResult, Error};
use std::collections::HashMap;
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&crate::utils::read_lines(data)?).map(|r| r.to_string()),
        2 => level_2(&crate::utils::read_lines(data)?).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
enum Type {
    Tree,
    Open,
    Lumberyard,
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Field {
    field: Vec<Type>,
    width: usize,
    height: usize,
}

impl Field {
    fn empty(width: usize, height: usize) -> Field {
        Field {
            width,
            height,
            field: vec![Type::Open; width * height],
        }
    }

    fn new(lines: &[String]) -> Field {
        let width = lines.len();
        let height = lines[0].len();

        let field = lines
            .iter()
            .flat_map(|l| {
                l.chars().map(|c| match c {
                    '.' => Type::Open,
                    '|' => Type::Tree,
                    '#' => Type::Lumberyard,
                    _ => panic!("invalid input char"),
                })
            })
            .collect();

        Field {
            field,
            width,
            height,
        }
    }
    fn get(&self, x: usize, y: usize) -> &Type {
        &self.field[x + y * self.width]
    }
    fn get_mut(&mut self, x: usize, y: usize) -> &mut Type {
        &mut self.field[x + y * self.width]
    }
    fn get_opt(&self, x: usize, y: usize) -> Option<&Type> {
        if x >= self.width || y >= self.height {
            return None;
        }
        self.field.get(x + y * self.width)
    }

    fn count(&self, x: usize, y: usize, tree: &mut usize, lumber: &mut usize, open: &mut usize) {
        if let Some(f) = self.get_opt(x, y) {
            match f {
                Type::Lumberyard => *lumber += 1,
                Type::Tree => *tree += 1,
                Type::Open => *open += 1,
            }
        }
    }

    fn get_next(&self, x: usize, y: usize) -> Type {
        let mut tree = 0;
        let mut lumber = 0;
        let mut open = 0;
        self.count(x + 1, y, &mut tree, &mut lumber, &mut open);
        self.count(x + 1, y + 1, &mut tree, &mut lumber, &mut open);
        if x > 0 {
            self.count(x - 1, y, &mut tree, &mut lumber, &mut open);
            self.count(x - 1, y + 1, &mut tree, &mut lumber, &mut open);
            if y > 0 {
                self.count(x - 1, y - 1, &mut tree, &mut lumber, &mut open);
            }
        }
        if y > 0 {
            self.count(x + 1, y - 1, &mut tree, &mut lumber, &mut open);
            self.count(x, y - 1, &mut tree, &mut lumber, &mut open);
        }
        self.count(x, y + 1, &mut tree, &mut lumber, &mut open);

        match self.get(x, y) {
            Type::Open => {
                if tree >= 3 {
                    Type::Tree
                } else {
                    Type::Open
                }
            }
            Type::Tree => {
                if lumber >= 3 {
                    Type::Lumberyard
                } else {
                    Type::Tree
                }
            }
            Type::Lumberyard => {
                if lumber >= 1 && tree >= 1 {
                    Type::Lumberyard
                } else {
                    Type::Open
                }
            }
        }
    }

    fn count_resources(&self) -> usize {
        let lumberyard = self
            .field
            .iter()
            .filter(|f| **f == Type::Lumberyard)
            .count();
        let lumber = self.field.iter().filter(|f| **f == Type::Tree).count();

        lumberyard * lumber
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.height {
            let line: String = (0..self.width)
                .map(|x| {
                    match self.get(x, y) {
                        Type::Tree => '|',
                        Type::Open => '.',
                        Type::Lumberyard => '#',
                    }
                    .to_string()
                })
                .collect();
            println!("{}", line);
        }
    }
}

fn level_1(line: &[String]) -> ACResult<usize> {
    let mut field = Field::new(line);
    let mut field2 = Field::empty(field.width, field.height);

    let mut c = &mut field;
    let mut n = &mut field2;

    for _ in 0..10 {
        for x in 0..c.width {
            for y in 0..c.height {
                *n.get_mut(x, y) = c.get_next(x, y);
            }
        }

        std::mem::swap(&mut c, &mut n)
    }
    // c.print();
    Ok(c.count_resources())
}

fn level_2(line: &[String]) -> ACResult<usize> {
    let mut field = Field::new(line);
    let mut field2 = Field::empty(field.width, field.height);

    let mut c = &mut field;
    let mut n = &mut field2;

    let total_iterations: usize = 1_000_000_000;

    let mut knowns = HashMap::new();
    let mut known_counts = Vec::new();

    for i in 0..total_iterations {
        let pp = c.clone();
        if let Some(offset) = knowns.get(&pp) {
            let diff = i - offset;
            let index = (total_iterations - offset) % diff + offset;
            return Ok(known_counts[index]);
        }
        knowns.insert(pp, i);
        known_counts.push(c.count_resources());

        for x in 0..c.width {
            for y in 0..c.height {
                *n.get_mut(x, y) = c.get_next(x, y);
            }
        }

        // Switch fields
        let x = c;
        c = n;
        n = x;
    }

    Ok(c.count_resources())
}
