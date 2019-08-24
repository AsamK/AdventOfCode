use crate::errors::{ACResult, Error};
use nom::{
    alt, complete, delimited, do_parse, flat_map, many1, map, named, parse_to, tag, take_while,
};
use std::io::Read;

#[derive(Debug)]
pub struct TuringBlueprint {
    pub initial_state: String,
    pub checksum_step: u32,
    pub states: Vec<TuringState>,
}

#[derive(Debug)]
pub struct TuringState {
    pub name: String,
    pub action_0: TuringAction,
    pub action_1: TuringAction,
}

#[derive(Debug)]
pub struct TuringAction {
    pub write_value: u8,
    pub move_direction: TuringDirection,
    pub next_state: String,
}

#[derive(Debug)]
pub enum TuringDirection {
    LEFT,
    RIGHT,
}

named!(turing_state_name<&str, &str>, take_while!(|c: char| {c.is_alphabetic()}));

named!(turing_initial_state<&str, &str>,
    delimited!(tag!("Begin in state "), turing_state_name, tag!(".\n"))
);

named!(number<&str, u32>, flat_map!(take_while!(|c: char| {c.is_digit(10)}), parse_to!(u32)));

named!(turing_checksum_step<&str, u32>,
    delimited!(tag!("Perform a diagnostic checksum after "), number, tag!(" steps.\n"))
);

named!(turing_direction<&str, TuringDirection>,
    map!(alt!(tag!("left") | tag!("right")), |dir| {if dir=="left" { TuringDirection::LEFT} else {TuringDirection::RIGHT}})
);

named!(turing_value<&str, u8>,
    flat_map!(alt!(tag!("0") | tag!("1")), parse_to!(u8))
);

named!(turing_action<&str, TuringAction>,
    do_parse!(
        write_value: delimited!(tag!("        - Write the value "), turing_value, tag!(".\n")) >>
        move_direction: delimited!(tag!("        - Move one slot to the "), turing_direction, tag!(".\n")) >>
        next_state: delimited!(tag!("        - Continue with state "), turing_state_name, tag!(".\n")) >>
        (TuringAction { write_value, move_direction, next_state: next_state.to_string() })
    )
);

named!(turing_state<&str, TuringState>,
    do_parse!(
        tag!("\n") >>
        name: delimited!(tag!("In state "), turing_state_name, tag!(":\n")) >>
        tag!("    If the current value is 0:\n") >>
        action_0:    turing_action >>
        tag!("    If the current value is 1:\n") >>
        action_1:    turing_action >>
        (TuringState { name: name.to_string(), action_0, action_1 })
    )
);

named!(turing_blueprint<&str, TuringBlueprint>,
    do_parse!(
        initial_state: turing_initial_state >>
        checksum_step: turing_checksum_step >>
        states:    many1!(complete!(turing_state)) >>
        (TuringBlueprint { initial_state: initial_state.to_string(), checksum_step, states })
    )
);

pub fn parse_turing<T: Read>(mut data: T) -> ACResult<TuringBlueprint> {
    let mut contents = String::new();
    data.read_to_string(&mut contents)
        .map_err(|_| Error::new_str("Failed to read stdin"))?;

    turing_blueprint(&contents)
        .map(|x| x.1)
        .map_err(|_| Error::new_str("Failed to parse blueprint"))
}
