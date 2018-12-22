use crate::errors::{ACResult, Error};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::io::BufRead;
use std::io::Read;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&parse_line(data)?).map(|r| r.to_string()),
        2 => level_2(&parse_line(data)?).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

fn parse_line<T: Read>(mut data: T) -> ACResult<Input> {
    let mut contents = String::new();
    data.read_to_string(&mut contents)
        .map_err(|_| Error::new_str("Failed to read data"))?;

    parse_input(&contents)
        .map(|x| x.1)
        .map_err(|e| Error::new(format!("Failed to parse input: {}", e)))
}

#[derive(Debug)]
struct Input {
    depth: u64,
    target: Point,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    fn new(x: u64, y: u64) -> Self {
        Point { x, y }
    }
}

named!(parse_number_u64<&str, u64>,
    complete!(map!(take_while1!(|c: char| c.is_numeric()), |c| c.to_string().parse().unwrap()))
);

named!(parse_point<&str, Point>,
  do_parse!(
    x: parse_number_u64 >>
    tag!(",") >>
    y: parse_number_u64 >>
    (Point {x, y})
  )
);

named!(parse_input<&str, Input>,
    do_parse!(
        depth: preceded!(tag!("depth: "), parse_number_u64) >>
        tag!("\n") >>
        target: preceded!(tag!("target: "), parse_point) >>
        (Input { depth, target })
    )
);

struct Field<T> {
    field: Vec<T>,
    depth: u64,
}

impl<T: Default + Clone> Field<T> {
    fn new(depth: u64) -> Self {
        Field {
            field: vec![T::default(); (depth * depth) as usize],
            depth,
        }
    }

    fn get(&self, x: u64, y: u64) -> &T {
        &self.field[(y * self.depth + x) as usize]
    }

    fn get_mut(&mut self, x: u64, y: u64) -> &mut T {
        &mut self.field[(y * self.depth + x) as usize]
    }
}

impl Field<Type> {
    #[allow(dead_code)]
    fn print(&self, max_x: u64, max_y: u64) {
        for y in 0..=max_y {
            let line: String = (0..=max_x)
                .map(|x| {
                    "  ".to_owned()
                        + &match self.get(x, y) {
                            Type::Rocky => '.',
                            Type::Narrow => '|',
                            Type::Wet => '=',
                            Type::Mouth => 'M',
                            Type::Target => 'T',
                        }
                        .to_string()
                })
                .collect();
            println!("{}", line);
        }
    }
}

impl Field<Option<Type>> {
    fn into(self) -> Field<Type> {
        Field {
            field: self.field.into_iter().map(|c| c.unwrap()).collect(),
            depth: self.depth,
        }
    }
}

impl Field<HashMap<Tool, u64>> {
    #[allow(dead_code)]
    fn print(&self, max_x: u64, max_y: u64) {
        for y in 0..=max_y {
            let line: String = (0..=max_x)
                .map(|x| {
                    format!(
                        "{: >7}",
                        self.get(x, y)
                            .iter()
                            .map(|(t, d)| format!(
                                "{}{}",
                                match t {
                                    Tool::Gear => "G",
                                    Tool::Neither => "N",
                                    Tool::Torch => "T",
                                },
                                d
                            ))
                            .collect::<String>()
                    )
                    .to_string()
                })
                .collect();
            println!("{}", line);
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
enum Type {
    Rocky,
    Wet,
    Narrow,
    Mouth,
    Target,
}

impl Default for Type {
    fn default() -> Type {
        Type::Rocky
    }
}

impl Type {
    fn get_risk(&self) -> u64 {
        match self {
            Type::Rocky => 0,
            Type::Wet => 1,
            Type::Narrow => 2,
            Type::Mouth => 0,
            Type::Target => 0,
        }
    }
}

fn build_field(start: &Point, target: &Point, depth: u64) -> Field<Type> {
    let mut field_geologic = Field::<u64>::new(depth);
    let mut field = Field::new(depth);
    *field.get_mut(0, 0) = Some(Type::Mouth);
    *field.get_mut(target.x, target.y) = Some(Type::Target);
    for y in 0..depth {
        for x in 0..depth {
            let geologic_index = if x == start.x && y == start.y {
                0
            } else if x == target.x && y == target.y {
                0
            } else if y == 0 {
                x * 16807
            } else if x == 0 {
                y * 48271
            } else {
                *field_geologic.get(x - 1, y) * *field_geologic.get(x, y - 1)
            };

            let erosion_level = (geologic_index + depth) % 20183;

            *field_geologic.get_mut(x, y) = erosion_level;

            if (x == start.x && y == start.y) || (x == target.x && y == target.y) {
                continue;
            }
            let field_type = match erosion_level % 3 {
                0 => Type::Rocky,
                1 => Type::Wet,
                2 => Type::Narrow,
                _ => panic!("Unreachable"),
            };
            *field.get_mut(x, y) = Some(field_type);
        }
    }

    field.into()
}

#[derive(PartialEq, Eq, Debug)]
struct PartialPath {
    next_point: Point,
    dist: u64,
    tool: Tool,
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

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Tool {
    Gear,
    Torch,
    Neither,
}

struct Game {
    field: Field<Type>,
    shortests: Field<HashMap<Tool, u64>>,
    next: BinaryHeap<PartialPath>,
    target: Point,
}

impl Game {
    fn new(field: Field<Type>, start: &Point, target: &Point) -> Self {
        let mut next = BinaryHeap::new();
        next.push(PartialPath {
            next_point: start.clone(),
            tool: Tool::Torch,
            dist: 0,
        });

        let shortests = Field::new(field.depth);

        Game {
            field,
            shortests,
            next,
            target: target.clone(),
        }
    }

    fn get_shortest_minutes(mut self) -> u64 {
        while let Some(next) = self.next.pop() {
            if let Some(dist) = self
                .shortests
                .get(self.target.x, self.target.y)
                .get(&Tool::Torch)
            {
                if *dist < next.dist {
                    // self.shortests.print(self.target.x + 100, self.target.y + 100);
                    return *dist;
                }
            }

            let other_tool = Self::get_other_tool(
                self.field.get(next.next_point.x, next.next_point.y),
                &next.tool,
            );
            let tools =
            // if *self.field.get(next.next_point.x, next.next_point.y) == Type::Mouth {
            //     vec![Tool::Torch, Tool::Gear, Tool::Neither]
            // } else {
                vec![next.tool.clone(), other_tool.clone()]
            // }
            ;
            for tool in &tools {
                let next_dist = if *tool == next.tool {
                    next.dist
                } else {
                    next.dist + 7
                };

                let shortest = self.shortests.get(next.next_point.x, next.next_point.y);
                if let Some(dist) = shortest.get(tool) {
                    if *dist <= next_dist {
                        continue;
                    }
                }

                self.shortests
                    .get_mut(next.next_point.x, next.next_point.y)
                    .insert((*tool).clone(), next_dist);

                if next.next_point.x == self.target.x && next.next_point.y == self.target.y {
                    continue;
                }

                if next.next_point.x > 0 {
                    self.add_if_shorter(
                        Point::new(next.next_point.x - 1, next.next_point.y),
                        &tool,
                        next_dist,
                    );
                }
                if next.next_point.y > 0 {
                    self.add_if_shorter(
                        Point::new(next.next_point.x, next.next_point.y - 1),
                        &tool,
                        next_dist,
                    );
                }
                self.add_if_shorter(
                    Point::new(next.next_point.x + 1, next.next_point.y),
                    &tool,
                    next_dist,
                );
                self.add_if_shorter(
                    Point::new(next.next_point.x, next.next_point.y + 1),
                    &tool,
                    next_dist,
                );
            }
        }
        panic!("Shouldn't happen")
    }

    fn add_if_shorter(&mut self, point: Point, tool: &Tool, dist: u64) {
        let f = self.field.get(point.x, point.y);
        let tools = Self::get_necessary_tool(f);
        if !tools.contains(tool) {
            return;
        }

        let shortest = self.shortests.get(point.x, point.y);

        let new_dist = 1 + dist;
        if let Some(dist) = shortest.get(&tool) {
            if *dist <= new_dist {
                return;
            }
        }
        self.next.push(PartialPath {
            next_point: point.clone(),
            dist: new_dist,
            tool: tool.clone(),
        })
    }

    fn get_other_tool(field_type: &Type, tool: &Tool) -> Tool {
        let tools = Self::get_necessary_tool(field_type);
        if tools[0] == *tool {
            tools[1].clone()
        } else {
            tools[0].clone()
        }
    }

    fn get_necessary_tool(field_type: &Type) -> [Tool; 2] {
        match field_type {
            Type::Narrow => [Tool::Neither, Tool::Torch],
            Type::Wet => [Tool::Neither, Tool::Gear],
            Type::Rocky | Type::Mouth | Type::Target => [Tool::Gear, Tool::Torch],
        }
    }
}

fn level_1(input: &Input) -> ACResult<u64> {
    let start = Point::new(0, 0);

    let field = build_field(&start, &input.target, input.depth);

    // field.print(input.target.x, input.target.y);

    let mut risk_level = 0;
    for y in 0..=input.target.y {
        for x in 0..=input.target.x {
            risk_level += field.get(x, y).get_risk();
        }
    }
    Ok(risk_level)
}

fn level_2(input: &Input) -> ACResult<u64> {
    let start = Point::new(0, 0);

    let field = build_field(&start, &input.target, input.depth);

    // field.print(input.target.x + 5, input.target.y + 5);
    let game = Game::new(field, &start, &input.target);

    Ok(game.get_shortest_minutes())
}
