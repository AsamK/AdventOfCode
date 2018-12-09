use crate::errors::{ACResult, Error};
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(crate::utils::read_lines(data)?).map(|r| r.to_string()),
        2 => level_2(crate::utils::read_lines(data)?).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

struct Info {
    player_count: u32,
    last_marble_worth: u32,
}

named!(number<&str, u32>, flat_map!(complete!(take_while!(|c: char| {c.is_digit(10)})), parse_to!(u32)));

named!(info_line<&str, Info>,
  dbg!(do_parse!(
    player_count: number >>
    tag!(" players; last marble is worth ") >>
    last_marble_worth: number >>
    tag!(" points") >>
    (Info { player_count, last_marble_worth })
  ))
);

fn level_1(lines: Vec<String>) -> ACResult<u32> {
    let info = info_line(&lines[0]).unwrap().1;
    Ok(run_game(info.player_count, info.last_marble_worth))
}

fn level_2(lines: Vec<String>) -> ACResult<u32> {
    let info = info_line(&lines[0]).unwrap().1;
    Ok(run_game(info.player_count, info.last_marble_worth * 100))
}

fn run_game(player_count: u32, last_marble_worth: u32) -> u32 {
    let mut circle = Vec::new();
    circle.push(0);
    let mut player: u32 = 0;
    let mut player_points = vec![0; player_count as usize];
    let mut current_marble = 0;
    for i in 1..last_marble_worth {
        if i % 23 == 0 {
            // Special case
            player_points[player as usize] += i;

            let to_be_removed = (current_marble + circle.len() - 7) % circle.len();
            let removed_marble = circle.remove(to_be_removed);
            player_points[player as usize] += removed_marble;
            current_marble = to_be_removed % circle.len();
        } else {
            // Normal case
            let put_right_of = (current_marble + 1) % circle.len();
            let insert_position = put_right_of + 1;
            circle.insert(insert_position, i);
            current_marble = insert_position;
        }
        player = (player + 1) % player_count;
    }
    let mut winners: Vec<_> = player_points.iter().enumerate().collect();
    winners.sort_by_key(|&(_, points)| points);
    let (_winner, points) = winners[winners.len() - 1];
    *points
}
