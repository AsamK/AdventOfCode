use crate::errors::{ACResult, Error};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(crate::utils::read_lines(data)?).map(|r| r.to_string()),
        2 => level_2(crate::utils::read_lines(data)?).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct GuardLine {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    event: GuardEvent,
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
enum GuardEvent {
    Begin(u32),
    WakeUp,
    Asleep,
}

impl Ord for GuardLine {
    fn cmp(&self, other: &GuardLine) -> Ordering {
        if self.year > other.year
            || (self.year == other.year && self.month > other.month)
            || (self.year == other.year && self.month == other.month && self.day > other.day)
            || (self.year == other.year
                && self.month == other.month
                && self.day == other.day
                && self.hour > other.hour)
            || (self.year == other.year
                && self.month == other.month
                && self.day == other.day
                && self.hour == other.hour
                && self.minute > other.minute)
        {
            std::cmp::Ordering::Greater
        } else if self.year == other.year
            && self.month == other.month
            && self.day == other.day
            && self.hour == other.hour
            && self.minute == other.minute
        {
            std::cmp::Ordering::Equal
        } else {
            std::cmp::Ordering::Less
        }
    }
}

named!(number<&str, u32>, flat_map!(complete!(take_while!(|c: char| {c.is_digit(10)})), parse_to!(u32)));

named!(guard_event<&str, GuardEvent>,
  map!(alt!(tag!("wakes up")| tag!("falls asleep")| delimited!(tag!("Guard #"), dbg!(complete!(take_while!(|c: char| {c.is_digit(10)}))), tag!(" begins shift"))),
    |dir| {match dir {
        "wakes up" => GuardEvent::WakeUp,
        "falls asleep" => GuardEvent::Asleep,
        g => GuardEvent::Begin(g.parse().unwrap()),
    }}
  )
);

named!(info_line<&str, GuardLine>,
  dbg!(do_parse!(
    tag!("[") >>
    year: dbg!(number) >>
    tag!("-") >>
    month: dbg!(number) >>
    tag!("-") >>
    day: dbg!(number) >>
    tag!(" ") >>
    hour: dbg!(number) >>
    tag!(":") >>
    minute: dbg!(number) >>
    tag!("] ") >>
    event: dbg!(guard_event) >>
    (GuardLine {year, month, day, hour, minute, event})
  ))
);

fn level_1(lines: Vec<String>) -> ACResult<u32> {
    let mut infos = lines
        .iter()
        .map(|line| info_line(&line).map(|x| x.1))
        .collect::<Result<Vec<GuardLine>, _>>()
        .map_err(|_| Error::new_str("Failed to parse guard line"))?;

    infos.sort();
    let mut guard_id_to_minute_map = HashMap::new();
    let mut current_guard = if let GuardEvent::Begin(id) = infos[0].event {
        id
    } else {
        return Err(Error::new_str("Invalid input, first event must be begin"));
    };
    let mut begin_sleep_minute = 0;
    for i in infos {
        match i.event {
            GuardEvent::Begin(id) => current_guard = id,
            GuardEvent::Asleep => begin_sleep_minute = i.minute,
            GuardEvent::WakeUp => {
                let x = guard_id_to_minute_map
                    .entry(current_guard)
                    .or_insert(vec![0; 60]);
                for l in begin_sleep_minute..i.minute {
                    (*x)[l as usize] += 1;
                }
            }
        }
    }

    let (guard_id, minutes) = guard_id_to_minute_map
        .iter()
        .max_by_key(|(_, minutes)| minutes.iter().sum::<u32>())
        .unwrap();

    let (minute_id, _) = minutes
        .iter()
        .enumerate()
        .max_by_key(|&(_, count)| count)
        .unwrap();

    Ok(minute_id as u32 * guard_id)
}

fn level_2(lines: Vec<String>) -> ACResult<u32> {
    let mut infos = lines
        .iter()
        .map(|line| info_line(&line).map(|x| x.1))
        .collect::<Result<Vec<GuardLine>, _>>()
        .map_err(|_| Error::new_str("Failed to parse guard line"))?;

    infos.sort();
    let mut guard_id_to_minute_map = HashMap::new();
    let mut current_guard = if let GuardEvent::Begin(id) = infos[0].event {
        id
    } else {
        return Err(Error::new_str("Invalid input, first event must be begin"));
    };
    let mut begin_sleep_minute = 0;
    for i in infos {
        match i.event {
            GuardEvent::Begin(id) => current_guard = id,
            GuardEvent::Asleep => begin_sleep_minute = i.minute,
            GuardEvent::WakeUp => {
                let x = guard_id_to_minute_map
                    .entry(current_guard)
                    .or_insert(vec![0; 60]);
                for l in begin_sleep_minute..i.minute {
                    (*x)[l as usize] += 1;
                }
            }
        }
    }
    let mut max_count = 0;
    let mut minute_id: u32 = 0;
    let mut guard_id: u32 = 0;
    for (id, x) in guard_id_to_minute_map {
        for index in 0..60 {
            let count = x[index];
            if count > max_count {
                max_count = count;
                minute_id = index as u32;
                guard_id = id;
            }
        }
    }
    Ok(minute_id * guard_id)
}
