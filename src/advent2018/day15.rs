use crate::errors::{ACResult, Error};
use crate::utils::Field;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&crate::utils::read_lines(data)?).map(|r| r.to_string()),
        2 => level_2(&crate::utils::read_lines(data)?).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

#[derive(Clone)]
enum FieldType {
    Wall,
    Empty,
    Elf(Player),
    Goblin(Player),
}

#[derive(Clone)]
struct Player {
    hit_points: u32,
}

#[derive(PartialEq, Debug, Clone, Eq)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Self {
        Point { x, y }
    }

    fn above(&self) -> Option<Self> {
        if self.y > 0 {
            Some(Point {
                x: self.x,
                y: self.y - 1,
            })
        } else {
            None
        }
    }

    fn below(&self) -> Option<Self> {
        if self.y != u32::max_value() {
            Some(Point {
                x: self.x,
                y: self.y + 1,
            })
        } else {
            None
        }
    }

    fn left(&self) -> Option<Self> {
        if self.x > 0 {
            Some(Point {
                x: self.x - 1,
                y: self.y,
            })
        } else {
            None
        }
    }

    fn right(&self) -> Option<Self> {
        if self.x != u32::max_value() {
            Some(Point {
                x: self.x + 1,
                y: self.y,
            })
        } else {
            None
        }
    }
    fn get_adjacent_points(&self) -> Vec<Self> {
        let mut result = Vec::new();
        if let Some(p) = self.above() {
            result.push(p);
        }
        if let Some(p) = self.left() {
            result.push(p);
        }
        if let Some(p) = self.right() {
            result.push(p);
        }
        if let Some(p) = self.below() {
            result.push(p);
        }
        result
    }
}

#[derive(PartialEq, Eq)]
struct PartialPath {
    path_start: Point,
    next_point: Point,
    // path length * 4 + prio of first point
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

static START_HIT_POINTS: u32 = 200;

struct Game {
    field: Field<FieldType>,
    attack_power_goblin: u32,
    attack_power_elf: u32,
}

impl Game {
    fn new(input: &[String], attack_power_elf: u32, attack_power_goblin: u32) -> Self {
        let height = input.len() as u32;
        let width = input[0].len() as u32;
        let field = input
            .iter()
            .flat_map(|l| {
                l.chars()
                    .map(|c| match c {
                        '#' => FieldType::Wall,
                        '.' => FieldType::Empty,
                        'G' => FieldType::Goblin(Player {
                            hit_points: START_HIT_POINTS,
                        }),
                        'E' => FieldType::Elf(Player {
                            hit_points: START_HIT_POINTS,
                        }),
                        _ => panic!("Invalid input"),
                    })
                    .collect::<Vec<FieldType>>()
            })
            .collect();
        Game {
            field: Field::from(field, width, height),
            attack_power_elf,
            attack_power_goblin,
        }
    }

    fn get_point(&self, point: &Point) -> &FieldType {
        &self.field.get(point.x, point.y)
    }

    fn set(&mut self, point: &Point, field: FieldType) {
        *self.field.get_mut(point.x, point.y) = field;
    }
    fn get(&self, x: u32, y: u32) -> &FieldType {
        &self.field.get(x, y)
    }

    fn get_mut(&mut self, point: &Point) -> &mut FieldType {
        self.field.get_mut(point.x, point.y)
    }

    fn get_adjacent(&self, point: &Point) -> Vec<Point> {
        let mut result = Vec::new();
        if let Some(p) = point.above() {
            result.push(p);
        }
        if let Some(p) = point.left() {
            result.push(p);
        }
        if let Some(p) = point.right() {
            if p.y < self.field.height() {
                result.push(p);
            }
        }
        if let Some(p) = point.below() {
            if p.y < self.field.width() {
                result.push(p);
            }
        }
        result
    }

    fn get_victim(&self, point: &Point) -> Option<Point> {
        let in_range = self.get_adjacent(point);
        in_range
            .iter()
            .filter(|f| match self.get_point(f) {
                FieldType::Elf(_) => {
                    if let FieldType::Elf(_) = self.get_point(point) {
                        false
                    } else {
                        true
                    }
                }
                FieldType::Goblin(_) => {
                    if let FieldType::Goblin(_) = self.get_point(point) {
                        false
                    } else {
                        true
                    }
                }
                _ => false,
            })
            .min_by_key(|f| match self.get_point(f) {
                FieldType::Elf(p) => p.hit_points,
                FieldType::Goblin(p) => p.hit_points,
                _ => panic!("Unreachable"),
            })
            .cloned()
    }

    fn attack(&mut self, victim_point: &Point) {
        let victim = self.get_point(victim_point);
        let mut new_hit_points = 0;
        let is_dead = match victim {
            FieldType::Goblin(p) => {
                if p.hit_points > self.attack_power_elf {
                    new_hit_points = p.hit_points - self.attack_power_elf;
                    false
                } else {
                    true
                }
            }
            FieldType::Elf(p) => {
                if p.hit_points > self.attack_power_goblin {
                    new_hit_points = p.hit_points - self.attack_power_goblin;
                    false
                } else {
                    true
                }
            }
            _ => panic!("Unreachable"),
        };
        let victim = self.get_mut(victim_point);
        match victim {
            FieldType::Elf(p) | FieldType::Goblin(p) => {
                p.hit_points = new_hit_points;
            }
            _ => {}
        }
        if is_dead {
            *victim = FieldType::Empty;
        }
    }

    fn find_target_elves(&self) -> Vec<Point> {
        let mut result = Vec::new();
        for y in 0..self.field.height() {
            for x in 0..self.field.width() {
                if let FieldType::Elf(_) = self.get(x, y) {
                    result.push(Point::new(x, y));
                }
            }
        }
        result
    }

    fn find_target_goblins(&self) -> Vec<Point> {
        let mut result = Vec::new();
        for y in 0..self.field.height() {
            for x in 0..self.field.width() {
                if let FieldType::Goblin(_) = self.get(x, y) {
                    result.push(Point::new(x, y));
                }
            }
        }
        result
    }

    fn get_adjacent_empty(&self, point: &Point) -> Vec<(usize, Point)> {
        self.get_adjacent(point)
            .iter()
            .enumerate()
            .filter(|(_, f)| match self.get_point(f) {
                FieldType::Empty => true,
                _ => false,
            })
            .map(|(i, p)| (i, p.clone()))
            .collect()
    }

    fn get_shortest_path(&self, from: &Point, to: &[Point]) -> Option<Point> {
        let mut shortests: Field<Option<usize>> =
            Field::new(self.field.width(), self.field.height());

        let mut partials = BinaryHeap::new();

        for (i, p) in self.get_adjacent_empty(from) {
            partials.push(PartialPath {
                path_start: p.clone(),
                next_point: p,
                dist: 4 + i,
            });
        }

        let mut shortest = usize::max_value();
        let mut result = None;

        while let Some(last) = partials.pop() {
            if last.dist > shortest {
                break;
            }
            if let Some(s) = *shortests.get(last.next_point.x, last.next_point.y) {
                if s <= last.dist {
                    continue;
                }
            }
            *shortests.get_mut(last.next_point.x, last.next_point.y) = Some(last.dist);

            if to.contains(&last.next_point) {
                if shortest > last.dist {
                    shortest = last.dist;
                    result = Some(last.path_start);
                } else if shortest == last.dist && result != Some(last.path_start) {
                    panic!("Invalid state");
                }
                continue;
            }
            let next_dist = last.dist + 4;
            for (_, p) in self.get_adjacent_empty(&last.next_point) {
                if let Some(s) = *shortests.get(p.x, p.y) {
                    if s <= next_dist {
                        continue;
                    }
                }

                partials.push(PartialPath {
                    path_start: last.path_start.clone(),
                    next_point: p,
                    dist: next_dist,
                });
            }
        }

        result
    }

    fn handle_player(&mut self, point: &Point) -> bool {
        if let Some(victim_point) = self.get_victim(point) {
            self.attack(&victim_point);
            return false;
        }

        let field = self.get_point(point);
        let targets = match field {
            FieldType::Goblin(_) => self.find_target_elves(),
            FieldType::Elf(_) => self.find_target_goblins(),
            _ => panic!("Unreachable"),
        };

        if targets.is_empty() {
            return true;
        }

        let target_adjacent_points: Vec<_> = targets
            .iter()
            .flat_map(|p| p.get_adjacent_points())
            .filter(|f| match self.get_point(f) {
                FieldType::Empty => true,
                _ => false,
            })
            .collect();

        let route = self.get_shortest_path(point, &target_adjacent_points);

        if route.is_none() {
            return false;
        }

        let move_to = route.unwrap();

        let new_field = field.clone();

        self.set(&move_to, new_field);
        self.set(point, FieldType::Empty);

        if let Some(victim_point) = self.get_victim(&move_to) {
            self.attack(&victim_point);
        }

        false
    }

    fn remaining_hit_power(&self) -> u32 {
        let mut count = 0;
        for y in 0..self.field.height() {
            for x in 0..self.field.width() {
                count += match self.get(x, y) {
                    FieldType::Elf(p) | FieldType::Goblin(p) => p.hit_points,
                    _ => 0,
                };
            }
        }

        count
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.field.height() {
            let line: String = (0..self.field.width())
                .map(|x| {
                    match self.get(x, y) {
                        FieldType::Empty => '.',
                        FieldType::Wall => '#',
                        FieldType::Elf(_) => 'E',
                        FieldType::Goblin(_) => 'G',
                    }
                    .to_string()
                        + " "
                })
                .collect();
            println!("{}", line);
        }
    }

    fn count_elves(&self) -> usize {
        let mut count = 0;
        for y in 0..self.field.height() {
            for x in 0..self.field.width() {
                if let FieldType::Elf(_) = self.get(x, y) {
                    count += 1;
                }
            }
        }
        count
    }

    fn tick(&mut self) -> bool {
        let mut player_order = Vec::new();
        for y in 0..self.field.height() {
            for x in 0..self.field.width() {
                match self.get(x, y) {
                    FieldType::Elf(_) | FieldType::Goblin(_) => {
                        player_order.push(Point::new(x, y));
                    }
                    _ => {}
                };
            }
        }

        for p in player_order.iter() {
            match self.get_point(p) {
                FieldType::Elf(_) | FieldType::Goblin(_) => {
                    if self.handle_player(p) {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }
}

fn level_1(line: &[String]) -> ACResult<u32> {
    let mut game = Game::new(line, 3, 3);
    let mut round = 0;
    loop {
        // println!("{}", round);
        // game.print();
        if game.tick() {
            break;
        }
        round += 1;
    }
    let hit_power = game.remaining_hit_power();
    Ok(hit_power * round)
}

fn level_2(line: &[String]) -> ACResult<u32> {
    let goblin_attack = 3;
    let mut power = goblin_attack + 1;
    'outer: loop {
        let mut game = Game::new(line, power, goblin_attack);
        let mut round = 0;
        let elve_count = game.count_elves();
        loop {
            let finished = game.tick();
            if game.count_elves() < elve_count {
                power += 1;
                continue 'outer;
            }
            if finished {
                break;
            }
            round += 1;
        }
        let hit_power = game.remaining_hit_power();
        return Ok(hit_power * round);
    }
}
