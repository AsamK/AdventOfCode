use crate::errors::{ACResult, Error};
use nom::{call, complete, do_parse, error_position, flat_map, named, parse_to, tag, take_while};
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&crate::utils::read_lines(data)?).map(|r| r.to_string()),
        2 => level_2(&crate::utils::read_lines(data)?).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

struct FabricPieceInfo {
    i: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

named!(
    number<nom::types::CompleteStr<'_>, usize>,
    flat_map!(
        complete!(take_while!(|c: char| c.is_digit(10))),
        parse_to!(usize)
    )
);

named!(
    info_line<nom::types::CompleteStr<'_>, FabricPieceInfo>,
    do_parse!(
        tag!("#")
            >> i: number
            >> tag!(" @ ")
            >> left: number
            >> tag!(",")
            >> top: number
            >> tag!(": ")
            >> width: number
            >> tag!("x")
            >> height: number
            >> (FabricPieceInfo {
                i,
                left,
                top,
                width,
                height
            })
    )
);

struct FabricPiece {
    inner: Vec<bool>,
}
impl FabricPiece {
    fn new(max_width: usize, max_height: usize, info: &FabricPieceInfo) -> Self {
        let mut fab = vec![false; max_width * max_height];
        for y in info.top..(info.top + info.height) {
            let line_offset = y * max_width;
            for x in info.left..(info.left + info.width) {
                fab[line_offset + x] = true;
            }
        }
        FabricPiece { inner: fab }
    }

    fn get(&self, i: usize) -> bool {
        self.inner[i]
    }
}

fn get_max_width_height((w, h): (usize, usize), info: &FabricPieceInfo) -> (usize, usize) {
    (
        if info.left + info.width > w {
            info.left + info.width
        } else {
            w
        },
        if info.top + info.height > h {
            info.top + info.height
        } else {
            h
        },
    )
}

fn level_1(lines: &[String]) -> ACResult<usize> {
    let infos = lines
        .iter()
        .map(|line| info_line(nom::types::CompleteStr(&line)).map(|x| x.1))
        .collect::<Result<Vec<FabricPieceInfo>, _>>()
        .map_err(|_| Error::new_str("Failed to parse blueprint"))?;

    let (max_width, max_height) = infos.iter().fold((0, 0), get_max_width_height);

    let mut overlap = vec![0; max_width * max_height];

    for a in infos.iter() {
        let fab_a = FabricPiece::new(max_width, max_height, a);

        for b in infos.iter() {
            if a.i >= b.i
                || a.left + a.width - 1 < b.left
                || a.left > b.left + b.width - 1
                || a.top + a.height - 1 < b.top
                || a.top > b.top + b.height - 1
            {
                continue;
            }
            let fab_b = FabricPiece::new(max_width, max_height, b);

            for (i, overlap) in overlap.iter_mut().enumerate() {
                if fab_a.get(i) && fab_b.get(i) {
                    *overlap += 1;
                }
            }
        }
    }
    Ok(overlap.iter().filter(|o| **o >= 1).count())
}

fn level_2(lines: &[String]) -> ACResult<usize> {
    let infos = lines
        .iter()
        .map(|line| info_line(nom::types::CompleteStr(&line)).map(|x| x.1))
        .collect::<Result<Vec<FabricPieceInfo>, _>>()
        .map_err(|_| Error::new_str("Failed to parse blueprint"))?;

    let (max_width, max_height) = infos.iter().fold((0, 0), get_max_width_height);

    let mut ov = vec![false; infos.len()];

    for a in infos.iter() {
        let fab_a = FabricPiece::new(max_width, max_height, a);

        for b in infos.iter() {
            if a.i >= b.i
                || a.left + a.width - 1 < b.left
                || a.left > b.left + b.width - 1
                || a.top + a.height - 1 < b.top
                || a.top > b.top + b.height - 1
            {
                continue;
            }
            let fab_b = FabricPiece::new(max_width, max_height, b);

            for i in 0..(max_height * max_width) {
                if fab_a.get(i) && fab_b.get(i) {
                    ov[a.i - 1] = true;
                    ov[b.i - 1] = true;
                }
            }
        }
    }
    let mut overlap_free_i = None;
    for (i, ov) in ov.iter().enumerate() {
        if !*ov {
            if overlap_free_i.is_some() {
                return Err(Error::new_str("Two overlap free pieces found"));
            }
            overlap_free_i = Some(i + 1);
        }
    }
    if let Some(i) = overlap_free_i {
        Ok(i)
    } else {
        Err(Error::new_str("No overlap free piece found"))
    }
}
