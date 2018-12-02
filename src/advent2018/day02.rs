use errors::{ACResult, Error};
use std::collections::HashMap;

pub fn a02_1(lines: Vec<String>) -> ACResult<i32> {
    let mut c2 = 0;
    let mut c3 = 0;
    for line in lines.iter() {
        let line_chars = line.chars();
        let mut char_counts = HashMap::new();
        for c in line_chars {
            let count = char_counts.entry(c).or_insert(0);
            *count += 1;
        }
        let mut istwo = false;
        let mut isthree = false;
        for (_, char_count) in &char_counts {
            if *char_count == 2 {
                istwo = true;
            }
            if *char_count == 3 {
                isthree = true;
            }
        }
        if istwo {
            c2 += 1;
        }
        if isthree {
            c3 += 1;
        }
    }
    Ok(c2 * c3)
}

pub fn a02_2(lines: Vec<String>) -> ACResult<String> {
    for line in lines.iter() {
        for line2 in lines.iter() {
            if line == line2 {
                continue;
            }
            let mut common = "".to_string();
            for (a, b) in line.chars().zip(line2.chars()) {
                if a == b {
                    common += &a.to_string();
                }
            }
            if common.len() == line.len() - 1 {
                return Ok(common);
            }
        }
    }
    Err(Error::new_str("No correct box ID found :("))
}
