use errors::{ACResult, Error};
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(crate::utils::read_lines(data)?).map(|r| r.to_string()),
        2 => level_2(crate::utils::read_lines(data)?).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}

fn level_1(lines: Vec<String>) -> ACResult<u32> {
    let numbers: Vec<u32> = lines[0]
        .split(" ")
        .map(|s| s.parse().map_err(|_| Error::new_str("Failed to parse")))
        .collect::<ACResult<_>>()?;
    let node = parse_node(&mut numbers.iter());
    let count = count_node(&node);

    Ok(count)
}

fn count_node(node: &Node) -> u32 {
    node.children.iter().map(|c| count_node(c)).sum::<u32>() + node.metadata.iter().sum::<u32>()
}

fn parse_node<'a, T: std::iter::Iterator<Item = &'a u32>>(numbers: &mut T) -> Node {
    let child_count = numbers.next().unwrap();

    let meta_data_count = numbers.next().unwrap();

    let mut children = Vec::new();
    for _ in 0..*child_count {
        children.push(parse_node(numbers));
    }
    let mut metadata = Vec::new();
    for _ in 0..*meta_data_count {
        metadata.push(*numbers.next().unwrap());
    }
    Node { children, metadata }
}

fn count_node_2(node: &Node) -> u32 {
    if node.children.len() == 0 {
        node.metadata.iter().sum()
    } else {
        node.metadata
            .iter()
            .map(|m| {
                let m = *m as usize - 1;
                if m < node.children.len() {
                    count_node_2(&node.children[m])
                } else {
                    0
                }
            })
            .sum()
    }
}

fn level_2(lines: Vec<String>) -> ACResult<u32> {
    let numbers: Vec<u32> = lines[0]
        .split(" ")
        .map(|s| s.parse().map_err(|_| Error::new_str("Failed to parse")))
        .collect::<ACResult<_>>()?;
    let node = parse_node(&mut numbers.iter());
    let count = count_node_2(&node);

    Ok(count)
}
