use crate::errors::{ACResult, Error};
use std::f64::consts::PI;
use std::io::BufRead;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&crate::utils::read_lines(data)?).map(|r| r.to_string()),
        2 => level_2(&crate::utils::read_lines(data)?).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

fn parse_asteroids(lines: &[String]) -> Vec<(i32, i32)> {
    let mut asteroids = Vec::new();
    for (y, l) in lines.iter().enumerate() {
        for (x, v) in l.chars().enumerate() {
            if v == '#' {
                asteroids.push((x as i32, y as i32));
            }
        }
    }
    asteroids
}

fn find_best_asteroid(asteroids: &[(i32, i32)]) -> Option<(u32, (i32, i32))> {
    let mut largest = None;
    for a1 in asteroids.iter() {
        let mut c = 0;
        for a2 in asteroids.iter().filter(|a| *a != a1) {
            let (x_diff, y_diff) = (a2.0 as f64 - a1.0 as f64, a2.1 as f64 - a1.1 as f64);
            let radius = (x_diff * x_diff + y_diff * y_diff).sqrt();
            let angle = y_diff.atan2(x_diff);
            let mut blocked = false;
            for a3 in asteroids.iter().filter(|a| *a != a1 && *a != a2) {
                let (x3_diff, y3_diff) = (a3.0 as f64 - a1.0 as f64, a3.1 as f64 - a1.1 as f64);
                let radius3 = (x3_diff * x3_diff + y3_diff * y3_diff).sqrt();
                let angle3 = y3_diff.atan2(x3_diff);

                if (angle3 - angle).abs() < 0.00000001 && radius3 < radius {
                    blocked = true;
                    break;
                }
            }
            if !blocked {
                c += 1;
            }
        }
        if let Some((l, _)) = largest {
            if c > l {
                largest = Some((c, *a1));
            }
        } else {
            largest = Some((c, *a1));
        }
    }
    largest
}

fn level_1(lines: &[String]) -> ACResult<u32> {
    let asteroids = parse_asteroids(lines);
    let (largest, _point) = find_best_asteroid(&asteroids).unwrap();
    Ok(largest)
}

fn level_2(lines: &[String]) -> ACResult<i32> {
    let asteroids = parse_asteroids(lines);
    let (_largest, base) = find_best_asteroid(&asteroids).unwrap();
    let mut asteroids = asteroids
        .iter()
        .filter(|a| **a != base)
        .map(|a2| {
            let (x_diff, y_diff) = (a2.0 as f64 - base.0 as f64, base.1 as f64 - a2.1 as f64);
            let radius = (x_diff * x_diff + y_diff * y_diff).sqrt();
            let angle = y_diff.atan2(x_diff);
            let angle = if angle < 0.0 { 2.0 * PI + angle } else { angle };
            let angle = 2.0 * PI - angle;

            (a2, radius, angle)
        })
        .collect::<Vec<_>>();

    let mut start_angle = 1.5 * PI;
    let mut blocked_angle = None;
    let mut shot_down = 0;
    loop {
        let mut next = None;
        let mut index = 0;
        for (i, a) in asteroids.iter().enumerate() {
            let a_angle: f64 = a.2;
            if a_angle < start_angle {
                continue;
            }
            if let Some(blocked_angle) = blocked_angle {
                if (a_angle - blocked_angle as f64).abs() < 0.00001 {
                    continue;
                }
            }
            if let Some((_, radius, angle)) = next {
                if ((a_angle - angle as f64).abs() > 0.00000001 && a.2 < angle)
                    || ((a_angle - angle).abs() < 0.00000001 && a.1 < radius)
                {
                    next = Some(*a);
                    index = i;
                }
            } else {
                next = Some(*a);
                index = i;
            }
        }
        if next.is_none() {
            start_angle = 0.0;
            continue;
        }
        let next = next.unwrap();
        start_angle = next.2;
        blocked_angle = Some(next.2);
        asteroids.remove(index);
        shot_down += 1;
        if shot_down == 200 {
            return Ok((next.0).0 * 100 + (next.0).1);
        }
        if asteroids.is_empty() {
            break;
        }
    }
    Err(Error::new_str("Failed"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_level_1_examples() {
        assert_eq!(
            level_1(&[
                ".#..#".to_owned(),
                ".....".to_owned(),
                "#####".to_owned(),
                "....#".to_owned(),
                "...##".to_owned(),
            ]),
            Ok(8),
        );
        assert_eq!(
            level_1(&[
                "......#.#.".to_owned(),
                "#..#.#....".to_owned(),
                "..#######.".to_owned(),
                ".#.#.###..".to_owned(),
                ".#..#.....".to_owned(),
                "..#....#.#".to_owned(),
                "#..#....#.".to_owned(),
                ".##.#..###".to_owned(),
                "##...#..#.".to_owned(),
                ".#....####".to_owned(),
            ]),
            Ok(33),
        );
        assert_eq!(
            level_1(&[
                "#.#...#.#.".to_owned(),
                ".###....#.".to_owned(),
                ".#....#...".to_owned(),
                "##.#.#.#.#".to_owned(),
                "....#.#.#.".to_owned(),
                ".##..###.#".to_owned(),
                "..#...##..".to_owned(),
                "..##....##".to_owned(),
                "......#...".to_owned(),
                ".####.###.".to_owned(),
            ]),
            Ok(35),
        );
        assert_eq!(
            level_1(&[
                ".#..#..###".to_owned(),
                "####.###.#".to_owned(),
                "....###.#.".to_owned(),
                "..###.##.#".to_owned(),
                "##.##.#.#.".to_owned(),
                "....###..#".to_owned(),
                "..#.#..#.#".to_owned(),
                "#..#.#.###".to_owned(),
                ".##...##.#".to_owned(),
                ".....#.#..".to_owned(),
            ]),
            Ok(41),
        );
        assert_eq!(
            level_1(&[
                ".#..##.###...#######".to_owned(),
                "##.############..##.".to_owned(),
                ".#.######.########.#".to_owned(),
                ".###.#######.####.#.".to_owned(),
                "#####.##.#.##.###.##".to_owned(),
                "..#####..#.#########".to_owned(),
                "####################".to_owned(),
                "#.####....###.#.#.##".to_owned(),
                "##.#################".to_owned(),
                "#####.##.###..####..".to_owned(),
                "..######..##.#######".to_owned(),
                "####.##.####...##..#".to_owned(),
                ".#####..#.######.###".to_owned(),
                "##...#.##########...".to_owned(),
                "#.##########.#######".to_owned(),
                ".####.#.###.###.#.##".to_owned(),
                "....##.##.###..#####".to_owned(),
                ".#.#.###########.###".to_owned(),
                "#.#.#.#####.####.###".to_owned(),
                "###.##.####.##.#..##".to_owned(),
            ]),
            Ok(210),
        );
    }

    #[test]
    fn run_level_2_examples() {
        assert_eq!(
            level_2(&[
                ".#..##.###...#######".to_owned(),
                "##.############..##.".to_owned(),
                ".#.######.########.#".to_owned(),
                ".###.#######.####.#.".to_owned(),
                "#####.##.#.##.###.##".to_owned(),
                "..#####..#.#########".to_owned(),
                "####################".to_owned(),
                "#.####....###.#.#.##".to_owned(),
                "##.#################".to_owned(),
                "#####.##.###..####..".to_owned(),
                "..######..##.#######".to_owned(),
                "####.##.####...##..#".to_owned(),
                ".#####..#.######.###".to_owned(),
                "##...#.##########...".to_owned(),
                "#.##########.#######".to_owned(),
                ".####.#.###.###.#.##".to_owned(),
                "....##.##.###..#####".to_owned(),
                ".#.#.###########.###".to_owned(),
                "#.#.#.#####.####.###".to_owned(),
                "###.##.####.##.#..##".to_owned(),
            ]),
            Ok(802),
        );
    }
}
