use crate::errors::{ACResult, Error};
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(crate::utils::read_lines(data)?),
        2 => level_2(crate::utils::read_lines(data)?),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

#[derive(Debug, Clone)]
enum TrackType {
    Horizontal,
    Vertical,
    TopRightBottomLeft,
    TopLeftBottomRight,
    Intersection,
}

#[derive(Debug, Clone)]
enum CartDirection {
    Left,
    Right,
    Up,
    Down,
}

impl From<&char> for CartDirection {
    fn from(s: &char) -> CartDirection {
        match s {
            '<' => CartDirection::Left,
            '>' => CartDirection::Right,
            'v' => CartDirection::Down,
            '^' => CartDirection::Up,
            s => panic!(format!("Invalid cart direction: {}", s)),
        }
    }
}

impl From<&char> for TrackType {
    fn from(s: &char) -> TrackType {
        match s {
            '-' | '>' | '<' => TrackType::Horizontal,
            '|' | 'v' | '^' => TrackType::Vertical,
            '/' => TrackType::TopRightBottomLeft,
            '\\' => TrackType::TopLeftBottomRight,
            '+' => TrackType::Intersection,
            s => panic!(format!("Invalid track type: {}", s)),
        }
    }
}

#[derive(Clone)]
enum TurnType {
    Left,
    Right,
    Straight,
}

#[derive(Clone)]
struct Cart {
    direction: CartDirection,
    next_turn_type: TurnType,
}

impl Cart {
    fn update_next_direction(&mut self, track: &TrackType) {
        self.direction = match track {
            TrackType::Horizontal | TrackType::Vertical => self.direction.clone(),
            TrackType::TopLeftBottomRight => match self.direction {
                CartDirection::Left => CartDirection::Up,
                CartDirection::Right => CartDirection::Down,
                CartDirection::Up => CartDirection::Left,
                CartDirection::Down => CartDirection::Right,
            },
            TrackType::TopRightBottomLeft => match self.direction {
                CartDirection::Left => CartDirection::Down,
                CartDirection::Right => CartDirection::Up,
                CartDirection::Up => CartDirection::Right,
                CartDirection::Down => CartDirection::Left,
            },
            TrackType::Intersection => {
                let next_turn_type = self.next_turn_type.clone();
                self.next_turn_type = match next_turn_type {
                    TurnType::Left => TurnType::Straight,
                    TurnType::Straight => TurnType::Right,
                    TurnType::Right => TurnType::Left,
                };
                match next_turn_type {
                    TurnType::Left => match self.direction {
                        CartDirection::Left => CartDirection::Down,
                        CartDirection::Right => CartDirection::Up,
                        CartDirection::Up => CartDirection::Left,
                        CartDirection::Down => CartDirection::Right,
                    },
                    TurnType::Straight => self.direction.clone(),
                    TurnType::Right => match self.direction {
                        CartDirection::Left => CartDirection::Up,
                        CartDirection::Right => CartDirection::Down,
                        CartDirection::Up => CartDirection::Right,
                        CartDirection::Down => CartDirection::Left,
                    },
                }
            }
        };
    }
}

struct Game {
    width: usize,
    height: usize,
    tracks: Vec<Vec<Option<TrackType>>>,
    carts: Vec<Vec<Option<Cart>>>,
}

impl Game {
    fn new(field: &Vec<Vec<char>>) -> Self {
        let width = field[0].len();
        let height = field.len();
        let tracks = field
            .iter()
            .map(|l| {
                l.iter()
                    .map(|c| if *c == ' ' { None } else { Some(c.into()) })
                    .collect()
            })
            .collect();

        let carts = field
            .iter()
            .map(|l| {
                l.iter()
                    .map(|c| {
                        if *c != '<' && *c != '>' && *c != 'v' && *c != '^' {
                            None
                        } else {
                            Some(Cart {
                                direction: c.into(),
                                next_turn_type: TurnType::Left,
                            })
                        }
                    })
                    .collect()
            })
            .collect();

        Game {
            width,
            height,
            tracks,
            carts,
        }
    }

    fn tick(&mut self) -> Option<(usize, usize)> {
        let mut carts_handled = std::collections::HashSet::new();

        let mut first_collision = None;

        for y in 0..self.height {
            for x in 0..self.width {
                if self.carts[y][x].is_none() {
                    continue;
                }
                if carts_handled.contains(&(x, y)) {
                    continue;
                }

                let (new_x, new_y) =
                    next_position(x, y, &self.carts[y][x].as_ref().unwrap().direction);
                if self.move_cart(x, y, new_x, new_y).is_err() {
                    if first_collision.is_none() {
                        first_collision = Some((new_x, new_y));
                    }
                } else {
                    self.carts[new_y][new_x]
                        .as_mut()
                        .unwrap()
                        .update_next_direction(self.tracks[new_y][new_x].as_ref().unwrap());
                    carts_handled.insert((new_x, new_y));
                }
            }
        }

        first_collision
    }

    fn move_cart(
        &mut self,
        old_x: usize,
        old_y: usize,
        new_x: usize,
        new_y: usize,
    ) -> ACResult<()> {
        if self.carts[new_y][new_x].is_some() {
            self.carts[old_y][old_x] = None;
            self.carts[new_y][new_x] = None;
            return Err(Error::new(format!(
                "Not empty, removing both carts: {},{}",
                new_x, new_y
            )));
        }

        self.carts[new_y][new_x] = self.carts[old_y][old_x].clone();
        self.carts[old_y][old_x] = None;
        Ok(())
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.height {
            let line: String = (0..self.width)
                .map(|x| {
                    if let Some(cart) = self.carts[y][x].as_ref() {
                        match cart.direction {
                            CartDirection::Down => 'v',
                            CartDirection::Up => '^',
                            CartDirection::Left => '<',
                            CartDirection::Right => '>',
                        }
                    } else {
                        if let Some(track) = self.tracks[y][x].as_ref() {
                            match track {
                                TrackType::Horizontal => '-',
                                TrackType::Vertical => '|',
                                TrackType::TopLeftBottomRight => '\\',
                                TrackType::TopRightBottomLeft => '/',
                                TrackType::Intersection => '+',
                            }
                        } else {
                            ' '
                        }
                    }
                })
                .collect();
            println!("{}", line);
        }
    }

    fn get_last_cart(&self) -> Option<(usize, usize)> {
        let mut last_pos = None;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.carts[y][x].is_some() {
                    if last_pos.is_some() {
                        return None;
                    }
                    last_pos = Some((x, y));
                }
            }
        }
        last_pos
    }
}

fn next_position(x: usize, y: usize, direction: &CartDirection) -> (usize, usize) {
    match direction {
        CartDirection::Left => (x - 1, y),
        CartDirection::Right => (x + 1, y),
        CartDirection::Up => (x, y - 1),
        CartDirection::Down => (x, y + 1),
    }
}

fn level_1(lines: Vec<String>) -> ACResult<String> {
    let field = lines
        .iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut game = Game::new(&field);
    loop {
        if let Some(collision) = game.tick() {
            return Ok(format!("{},{}", collision.0, collision.1));
        }
    }
}
fn level_2(lines: Vec<String>) -> ACResult<String> {
    let field = lines
        .iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut game = Game::new(&field);
    loop {
        game.tick();

        let last_pos = game.get_last_cart();
        if let Some(a) = last_pos {
            return Ok(format!("{},{}", a.0, a.1));
        }
    }
}
