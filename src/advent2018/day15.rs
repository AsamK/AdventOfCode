use crate::errors::{ACResult, Error};
use rayon::prelude::*;
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
    x: u8,
    y: u8,
}

impl Point {
    fn new(x: u8, y: u8) -> Self {
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
        if self.y != u8::max_value() {
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
        if self.x != u8::max_value() {
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

    fn get_distance(&self, point: &Point) -> u8 {
        (if self.y > point.y {
            self.y - point.y
        } else {
            point.y - self.y
        }) + (if self.x > point.x {
            self.x - point.x
        } else {
            point.x - self.x
        })
    }
}

#[derive(PartialEq, Eq)]
struct PartialPath {
    path_start: Vec<Point>,
    next_point: Point,
    // path length + distance from next_point to target
    ord: usize,
}

impl PartialOrd for PartialPath {
    fn partial_cmp(&self, other: &PartialPath) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for PartialPath {
    fn cmp(&self, other: &PartialPath) -> Ordering {
        match self.ord.cmp(&other.ord) {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
        }
    }
}

static START_HIT_POINTS: u32 = 200;

struct Game {
    width: u8,
    height: u8,
    field: Vec<FieldType>,
    attack_power_goblin: u32,
    attack_power_elf: u32,
}

impl Game {
    fn new(input: &[String], attack_power_elf: u32, attack_power_goblin: u32) -> Self {
        let height = input.len() as u8;
        let width = input[0].len() as u8;
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
            height,
            width,
            field,
            attack_power_elf,
            attack_power_goblin,
        }
    }

    fn get_point(&self, point: &Point) -> &FieldType {
        &self.field[point.x as usize + point.y as usize * self.width as usize]
    }

    fn set(&mut self, point: &Point, field: FieldType) {
        self.field[point.x as usize + point.y as usize * self.width as usize] = field;
    }
    fn get(&self, x: u8, y: u8) -> &FieldType {
        &self.field[x as usize + y as usize * self.width as usize]
    }

    fn get_mut(&mut self, point: &Point) -> &mut FieldType {
        &mut self.field[point.x as usize + point.y as usize * self.width as usize]
    }

    fn get_adjacent(&self, point: &Point) -> Vec<Point> {
        let mut result = Vec::new();
        if let Some(p) = point.above() {
            result.push(p);
        }
        if let Some(p) = point.below() {
            if p.y < self.width {
                result.push(p);
            }
        }
        if let Some(p) = point.left() {
            result.push(p);
        }
        if let Some(p) = point.right() {
            if p.y < self.height {
                result.push(p);
            }
        }
        result
    }

    fn get_victim(&self, point: &Point) -> Option<Point> {
        let in_range = self.get_adjacent(point);
        let min_point = in_range
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
            .map(|p| p.clone());
        min_point
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
        for y in 0..self.height {
            for x in 0..self.width {
                if let FieldType::Elf(_) = self.get(x, y) {
                    result.push(Point::new(x, y));
                }
            }
        }
        result
    }

    fn find_target_goblins(&self) -> Vec<Point> {
        let mut result = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if let FieldType::Goblin(_) = self.get(x, y) {
                    result.push(Point::new(x, y));
                }
            }
        }
        result
    }

    fn get_adjacent_empty(&self, point: &Point) -> Vec<Point> {
        self.get_adjacent(point)
            .iter()
            .filter(|f| match self.get_point(f) {
                FieldType::Empty => true,
                _ => false,
            })
            .map(|p| p.clone())
            .collect()
    }

    fn sort_by_nearest(points: Vec<Point>, target: &Point) -> Vec<Point> {
        let mut points = points;
        points.sort_unstable_by(|a, b| {
            let a = target.get_distance(a);
            let b = target.get_distance(b);
            b.cmp(&a)
        });
        points
    }

    fn get_paths(
        &self,
        from: &Point,
        to: &Point,
        shortest_path_len: &std::sync::atomic::AtomicUsize,
    ) -> Vec<Vec<Point>> {
        let mut paths = Vec::new();

        let mut shortests = vec![usize::max_value(); self.width as usize * self.height as usize];

        let mut partials = BinaryHeap::new();

        for p in Game::sort_by_nearest(self.get_adjacent_empty(from), to) {
            let dist = p.get_distance(to);
            partials.push(PartialPath {
                path_start: vec![from.clone()],
                next_point: p,
                ord: 1 + dist as usize,
            });
        }

        loop {
            if let Some(last) = partials.pop() {
                let shortest = shortest_path_len.load(std::sync::atomic::Ordering::Relaxed);
                if last.path_start.len() + 1 > shortest {
                    continue;
                }
                if last.next_point == *to {
                    let mut path = last.path_start.clone();
                    path.push(last.next_point.clone());
                    shortests[last.next_point.x as usize
                        + last.next_point.y as usize * self.width as usize] = path.len();
                    if shortest > path.len() {
                        let shortest = shortest_path_len.load(std::sync::atomic::Ordering::Relaxed);
                        if shortest > path.len() {
                            shortest_path_len
                                .compare_exchange(
                                    shortest,
                                    path.len(),
                                    std::sync::atomic::Ordering::Relaxed,
                                    std::sync::atomic::Ordering::Relaxed,
                                )
                                .unwrap();
                        }
                    }
                    paths.push(path);
                } else {
                    if last.path_start.len() + 1 >= shortest {
                        continue;
                    }
                    if shortests[last.next_point.x as usize
                        + last.next_point.y as usize * self.width as usize]
                        <= last.path_start.len()
                    {
                        continue;
                    }
                    if to.get_distance(&last.next_point) as usize >= shortest {
                        continue;
                    }
                    let mut path = last.path_start.clone();
                    path.push(last.next_point.clone());
                    shortests[last.next_point.x as usize
                        + last.next_point.y as usize * self.width as usize] = path.len();
                    for p in Game::sort_by_nearest(self.get_adjacent_empty(&last.next_point), &to) {
                        if last.path_start.contains(&p) {
                            continue;
                        }
                        if shortests[p.x as usize + p.y as usize * self.width as usize]
                            <= path.len()
                        {
                            continue;
                        }

                        let dist = p.get_distance(to);
                        partials.push(PartialPath {
                            path_start: path.clone(),
                            next_point: p,
                            ord: path.len() + dist as usize,
                        });
                    }
                }
            } else {
                break;
            }
        }

        paths
    }

    fn get_first_route_in_reading_order<'a>(
        point: &Point,
        routes: &'a Vec<&'a Vec<Point>>,
    ) -> &'a Vec<Point> {
        for adj in point.get_adjacent_points().iter() {
            for p in routes.iter() {
                if p[1] == *adj {
                    return p;
                }
            }
        }
        panic!("Shouldn't happen");
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

        if targets.len() == 0 {
            return true;
        }

        let mut target_adjacent_points: Vec<_> = targets
            .iter()
            .flat_map(|p| p.get_adjacent_points())
            .filter(|f| match self.get_point(f) {
                FieldType::Empty => true,
                _ => false,
            })
            .collect();
        target_adjacent_points.sort_unstable_by_key(|p| point.get_distance(p));

        let shortest_path = std::sync::atomic::AtomicUsize::new(usize::max_value());

        let mut target_adjacent_points: Vec<_> = target_adjacent_points
            .par_iter()
            .flat_map(|p| {
                if point.get_distance(&p) as usize
                    > shortest_path.load(std::sync::atomic::Ordering::Relaxed)
                {
                    Vec::new()
                } else {
                    self.get_paths(point, &p, &shortest_path)
                }
            })
            .collect();

        target_adjacent_points.sort_by_key(|p| p.len());

        let routes: Vec<_> = target_adjacent_points
            .iter()
            .filter(|p| p.len() == target_adjacent_points[0].len())
            .collect();

        if routes.len() == 0 {
            return false;
        }

        let route = if routes.len() > 1 {
            Game::get_first_route_in_reading_order(point, &routes)
        } else {
            routes[0]
        };

        let move_to = &route[1];

        self.set(move_to, field.clone());
        self.set(point, FieldType::Empty);

        if let Some(victim_point) = self.get_victim(move_to) {
            self.attack(&victim_point);
        }

        false
    }

    fn remaining_hit_power(&self) -> u32 {
        let mut count = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                count += match self.get(x, y) {
                    FieldType::Elf(p) | FieldType::Goblin(p) => p.hit_points,
                    _ => 0,
                };
            }
        }

        return count;
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.height {
            let line: String = (0..self.width)
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
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get(x, y) {
                    FieldType::Elf(_) => count += 1,
                    _ => {}
                };
            }
        }
        count
    }

    fn tick(&mut self) -> bool {
        let mut player_order = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
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
        return false;
    }
}

fn level_1(line: &Vec<String>) -> ACResult<u32> {
    let mut game = Game::new(line, 3, 3);
    let mut round = 0;
    loop {
        println!("{}", round);
        if game.tick() {
            break;
        }
        round += 1;
    }
    let hit_power = game.remaining_hit_power();
    Ok(hit_power * round)
}

fn level_2(line: &Vec<String>) -> ACResult<u32> {
    let goblin_attack = 3;
    let mut power = goblin_attack + 1;
    'outer: loop {
        println!(" {}", power);

        let mut game = Game::new(line, power, goblin_attack);
        let mut round = 0;
        let elve_count = game.count_elves();
        loop {
            println!("{}", round);
            let finished = game.tick();
            if game.count_elves() < elve_count {
                println!("An elve died :(");
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
