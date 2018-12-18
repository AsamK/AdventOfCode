use crate::errors::{ACResult, Error};
use std::collections::HashMap;
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(crate::utils::read_lines(data)?),
        2 => level_2(crate::utils::read_lines(data)?).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

struct Info {
    name: char,
    dep: char,
}

named!(info_line<&str, Info>,
  dbg!(do_parse!(
    tag!("Step ") >>
    dep: take!(1)  >>
    tag!(" must be finished before step ") >>
    name: take!(1) >>
    tag!(" can begin.") >>
    (Info {dep: dep.chars().next().unwrap(), name:name.chars().next().unwrap()})
  ))
);

// Step F must be finished before step E can begin.
fn level_1(lines: Vec<String>) -> ACResult<String> {
    let infos = lines
        .iter()
        .map(|l| info_line(&l).map(|x| x.1))
        .collect::<Result<Vec<Info>, _>>()
        .map_err(|_| Error::new_str("Failed to parse line"))?;
    let mut name_to_dependencies_map = HashMap::new();
    for l in infos {
        let entry = name_to_dependencies_map.entry(l.name).or_insert(Vec::new());
        entry.push(l.dep);
        name_to_dependencies_map.entry(l.dep).or_insert(Vec::new());
    }
    let mut order = "".to_string();
    loop {
        let mut available = Vec::new();
        if name_to_dependencies_map.len() == 0 {
            break;
        }
        for (name, deps) in &name_to_dependencies_map {
            if deps.len() == 0 {
                available.push(name.clone());
            }
        }
        available.sort();
        if available.len() == 0 {
            panic!("invalid input")
        }
        let element = available[0];
        for (_, deps) in &mut name_to_dependencies_map {
            let mut index = None;
            for (i, x) in deps.iter().enumerate() {
                if *x == element {
                    index = Some(i);
                    break;
                }
            }
            if index == None {
                continue;
            }
            deps.remove(index.unwrap());
        }
        name_to_dependencies_map.remove(&element);
        order += &element.to_string();
    }
    Ok(order)
}

#[derive(Debug)]
struct Worker {
    finished_second: u32,
    work: Option<char>,
}

fn level_2(lines: Vec<String>) -> ACResult<u32> {
    let infos = lines
        .iter()
        .map(|l| info_line(&l).map(|x| x.1))
        .collect::<Result<Vec<Info>, _>>()
        .map_err(|_| Error::new_str("Failed to parse guard line"))?;
    let mut deps = HashMap::new();
    for l in infos {
        let entry = deps.entry(l.name).or_insert(Vec::new());
        entry.push(l.dep);
        deps.entry(l.dep).or_insert(Vec::new());
    }
    let mut workers = Vec::new();
    for _ in 0..5 {
        workers.push(Worker {
            finished_second: 0,
            work: None,
        });
    }
    let mut second = 0;
    loop {
        // println!("{:?}", deps);
        // println!("{:?}", workers);
        let mut available = Vec::new();
        if deps.len() == 0 {
            break;
        }

        for w in workers.iter_mut() {
            if w.work == None {
                continue;
            }
            if w.finished_second == second {
                for (_, deps) in &mut deps {
                    let mut index = None;
                    for (i, x) in deps.iter().enumerate() {
                        if *x == w.work.unwrap() {
                            index = Some(i);
                            break;
                        }
                    }
                    if let Some(i) = index {
                        deps.remove(i);
                    }
                }
                w.work = None;
            }
        }
        for (name, deps) in &deps {
            if deps.len() == 0 {
                available.push(name.clone());
            }
        }
        available.sort();
        if available.len() == 0 {
            second += 1;
            continue;
        }

        let mut availables = available.iter();
        for w in workers.iter_mut() {
            if w.work != None {
                continue;
            }
            if let Some(element) = availables.next() {
                deps.remove(&element);
                w.work = Some(*element);
                let mut b: [u8; 1] = [0; 1];
                element.encode_utf8(&mut b);
                w.finished_second = second + 60 + b[0] as u32 - b'A' as u32 + 1;
            }
        }

        second += 1;
    }
    let mut max_second = second;
    for w in workers {
        if let Some(_) = w.work {
            if w.finished_second > max_second {
                max_second = w.finished_second;
            }
        }
    }
    Ok(max_second)
}
