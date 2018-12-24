use crate::errors::{ACResult, Error};
use nom::{
    alt, call, complete, delimited, do_parse, error_position, many1, map, named, opt, preceded,
    tag, take_while1, terminated, tuple, tuple_parser,
};
use std::cmp::Ordering;
use std::io::BufRead;
use std::io::Read;

pub fn get_result<T: BufRead>(data: T, level: u8) -> ACResult<String> {
    match level {
        1 => level_1(&parse_line(data)?).map(|r| r.to_string()),
        2 => level_2(&parse_line(data)?).map(|r| r.to_string()),
        _ => Err(Error::new(format!("Level {} not implemented", level))),
    }
}

fn parse_line<T: Read>(mut data: T) -> ACResult<Input> {
    let mut contents = String::new();
    data.read_to_string(&mut contents)
        .map_err(|_| Error::new_str("Failed to read data"))?;

    parse_input(&contents)
        .map(|x| x.1)
        .map_err(|e| Error::new(format!("Failed to parse input: {}", e)))
}

#[derive(Debug)]
struct Input {
    immune_groups: Vec<Group>,
    infection_groups: Vec<Group>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Group {
    unit_count: u32,
    hit_point_per_unit: u32,
    attack_damage: u32,
    initiative: u32,
    attack_type: Attack,
    weaknesses: Vec<Attack>,
    immunities: Vec<Attack>,
}

impl Group {
    fn defending_factor_to(&self, attack: &Attack) -> u32 {
        if self.immunities.contains(attack) {
            return 0;
        }
        if self.weaknesses.contains(attack) {
            return 2;
        }
        return 1;
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Attack {
    Slashing,
    Radiation,
    Fire,
    Bludgeoning,
    Cold,
}

named!(parse_number_u32<&str, u32>,
    complete!(map!(take_while1!(|c: char| c.is_numeric()), |c| c.to_string().parse().unwrap()))
);

named!(parse_i_w<&str, (Vec<Attack>, Vec<Attack>)>,
alt!(
    do_parse!(
        tag!("immune to ") >>
        immunes: many1!(terminated!(parse_attack, opt!(tag!(", ")))) >>
        weaks: opt!(
            preceded!(
                tag!("; weak to "),
                many1!(terminated!(parse_attack, opt!(tag!(", "))))
            )
        ) >>
        ((immunes, weaks.unwrap_or_else(|| Vec::new())))
    )
        |
    do_parse!(
        tag!("weak to ") >>
        weaks: many1!(terminated!(parse_attack, opt!(tag!(", ")))) >>
        immunes: opt!(
            preceded!(
                tag!("; immune to "),
                many1!(terminated!(parse_attack, opt!(tag!(", "))))
            )
        ) >>
        (( immunes.unwrap_or_else(|| Vec::new()), weaks))
    )
)
);

named!(parse_attack<&str, Attack>,
    map!(take_while1!(|c| c!=' ' && c!=';' &&c!=')' && c!=','),
    |word| match word {
        "slashing"=> Attack::Slashing,
        "radiation"=> Attack::Radiation,
        "fire"=> Attack::Fire,
        "bludgeoning"=> Attack::Bludgeoning,
        "cold"=> Attack::Cold,
        _ => panic!("Invalid attach: {}", word),
    }
    )
);

named!(parse_group<&str, Group>,
    do_parse!(
        unit_count: parse_number_u32 >>
        tag!(" units each with ") >>
        hit_point_per_unit: parse_number_u32 >>
        tag!(" hit points ") >>
        i_w: map!(opt!(delimited!(
            tag!("("),
            parse_i_w,
            tag!(") ")
        )),
        |o| o.unwrap_or_else(|| (Vec::new(), Vec::new()))
         ) >>
        tag!("with an attack that does ") >>
        attack_damage: parse_number_u32 >>
        tag!(" ") >>
        attack_type: parse_attack >>
        tag!(" damage at initiative ") >>
        initiative: parse_number_u32 >>
        tag!("\n") >>
        (Group {unit_count, hit_point_per_unit, attack_damage, initiative, attack_type,
        immunities: i_w.0,
        weaknesses: i_w.1})
    )
);

named!(parse_input<&str, Input>,
    do_parse!(
        tag!("Immune System:\n") >>
        immune_groups: many1!(parse_group) >>
        tag!("\n") >>
        tag!("Infection:\n") >>
        infection_groups: complete!(many1!(parse_group)) >>
        (Input { immune_groups, infection_groups })
    )
);

#[derive(Debug, Eq, PartialEq, Clone)]
enum Type {
    Immune,
    Infection,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct GroupState<'a> {
    id: u32,
    info: &'a Group,
    units_taken: u32,
    typ: Type,
}
impl<'a> GroupState<'a> {
    fn effective_power(&self) -> u32 {
        self.info.attack_damage * self.remaining_units()
    }

    fn remaining_units(&self) -> u32 {
        if self.info.unit_count <= self.units_taken {
            return 0;
        }
        self.info.unit_count - self.units_taken
        // let remaining_hitpoints = total - self.hitpoints_taken;
        // remaining_hitpoints / self.info.hit_point_per_unit
        //     + if remaining_hitpoints % self.info.hit_point_per_unit != 0 {
        //         1
        //     } else {
        //         0
        //     }
    }
}

fn run_game(mut groups: Vec<GroupState>) -> Vec<GroupState> {
    let initial_size = groups.len();
    loop {
        groups.sort_unstable_by(|a, b| {
            let comp = a.effective_power().cmp(&b.effective_power());
            if comp != Ordering::Equal {
                return comp;
            }
            a.info.initiative.cmp(&b.info.initiative)
        });
        groups.reverse();

        let mut attacks = vec![None; initial_size];
        let mut count = 0;
        for g in groups.iter() {
            let g2 = groups
                .iter()
                .max_by_key(|g2| {
                    if g.typ == g2.typ || attacks.contains(&Some(g2.id)) {
                        return (0, 0, 0);
                    }
                    let damage =
                        g.effective_power() * g2.info.defending_factor_to(&g.info.attack_type);
                    // let lost_units = damage / g2.info.hit_point_per_unit;
                    (damage, g2.effective_power(), g2.info.initiative)
                })
                .unwrap();
            if g2.info.defending_factor_to(&g.info.attack_type) == 0
                || g2.typ == g.typ
                || attacks.contains(&Some(g2.id))
            {
                continue;
            }
            attacks[g.id as usize] = Some(g2.id);
            count += 1;
        }

        if count == 0 {
            // No more enemy armies
            break;
        }

        // Attack
        groups.sort_unstable_by(|a, b| a.info.initiative.cmp(&b.info.initiative));
        groups.reverse();

        // let mut damages = vec![None; input.immune_groups.len() + input.infection_groups.len()];
        let mut taken = 0;
        for id1 in groups.clone().iter().map(|g| g.id) {
            let g1 = groups
                .iter()
                .find(|gg| gg.id == id1 as u32)
                .unwrap()
                .clone();

            if g1.remaining_units() == 0 {
                continue;
            }

            if let Some(id2) = attacks[g1.id as usize] {
                let g2 = groups.iter_mut().find(|gg| gg.id == id2).unwrap();
                let damage =
                    g1.effective_power() * g2.info.defending_factor_to(&g1.info.attack_type);
                let lost_units = damage / g2.info.hit_point_per_unit;
                if lost_units > 0 {
                    taken += 1;
                }
                g2.units_taken += lost_units;
            }
        }
        if taken == 0 {
            break;
        }
        // for g in groups.iter_mut() {
        //     if let Some(damage) = damages[g.id as usize] {
        //         g.units_taken += damage;
        //     }
        // }
        groups = groups
            .into_iter()
            .filter(|g| g.remaining_units() > 0)
            .collect();
    }
    groups
}

fn level_1(input: &Input) -> ACResult<u32> {
    let groups: Vec<_> = input
        .immune_groups
        .iter()
        .enumerate()
        .map(|(i, g)| GroupState {
            id: i as u32,
            info: g,
            units_taken: 0,
            typ: Type::Immune,
        })
        .chain(
            input
                .infection_groups
                .iter()
                .enumerate()
                .map(|(i, g)| GroupState {
                    id: input.immune_groups.len() as u32 + i as u32,
                    info: g,
                    units_taken: 0,
                    typ: Type::Infection,
                }),
        )
        .collect();
    let groups = run_game(groups);
    Ok(groups.iter().map(|g| g.remaining_units()).sum())
}

fn level_2(input: &Input) -> ACResult<u32> {
    let mut boost = 0;
    loop {
        let input = Input {
            immune_groups: input
                .immune_groups
                .iter()
                .map(|g| {
                    let mut g = g.clone();
                    g.attack_damage += boost;
                    g
                })
                .collect(),
            infection_groups: input.infection_groups.clone(),
        };
        let groups: Vec<_> = input
            .immune_groups
            .iter()
            .enumerate()
            .map(|(i, g)| GroupState {
                id: i as u32,
                info: g,
                units_taken: 0,
                typ: Type::Immune,
            })
            .chain(
                input
                    .infection_groups
                    .iter()
                    .enumerate()
                    .map(|(i, g)| GroupState {
                        id: input.immune_groups.len() as u32 + i as u32,
                        info: g,
                        units_taken: 0,
                        typ: Type::Infection,
                    }),
            )
            .collect();
        let groups = run_game(groups);
        if groups.iter().filter(|g| g.typ == Type::Infection).count() > 0 {
            boost += 1;
            continue;
        }
        return Ok(groups.iter().map(|g| g.remaining_units()).sum());
    }
}
