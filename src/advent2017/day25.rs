mod parser;

use self::parser::{TuringBlueprint, TuringDirection, TuringState};
use crate::errors::{ACResult, Error};
use std::io::Read;

pub fn get_result<T: Read>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&parser::parse_turing(data)?),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
    .map(|r| r.to_string())
}

pub fn level_1(blueprint: &TuringBlueprint) -> ACResult<u32> {
    let mut state_name = &blueprint.initial_state;
    let mut step = 0;
    let mut cursor: isize = 0;

    // Positions 0..inf
    let mut band_right = Vec::new();
    band_right.push(0);
    // Positions -1..-inf
    let mut band_left = Vec::new();

    loop {
        let state = get_state(&blueprint.states, state_name)
            .ok_or_else(|| Error::new_str("Invalid state"))?;
        let value = if cursor >= 0 {
            band_right[cursor as usize]
        } else {
            band_left[(-cursor - 1) as usize]
        };

        let action = if value == 0 {
            &state.action_0
        } else {
            &state.action_1
        };

        // Execute action
        if cursor >= 0 {
            band_right[cursor as usize] = action.write_value;
        } else {
            band_left[(-cursor - 1) as usize] = action.write_value;
        }
        state_name = &action.next_state;
        match action.move_direction {
            TuringDirection::LEFT => {
                cursor -= 1;
                if (-cursor - 1) as usize >= band_left.len() {
                    band_left.push(0);
                }
            }
            TuringDirection::RIGHT => {
                cursor += 1;
                if cursor as usize >= band_right.len() {
                    band_right.push(0);
                }
            }
        }

        step += 1;
        if step == blueprint.checksum_step {
            let checksum = band_left.iter().chain(band_right.iter()).fold(0, |sum, i| {
                if *i == 1 {
                    sum + 1
                } else {
                    sum
                }
            });
            return Ok(checksum);
        }
    }
}

fn get_state<'a>(states: &'a [TuringState], name: &str) -> Option<&'a TuringState> {
    for state in states {
        if state.name == name {
            return Some(&state);
        }
    }
    None
}
