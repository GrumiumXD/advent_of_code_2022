use cached::proc_macro::cached;
use rayon::prelude::*;

use nom::{
    bytes::complete::tag,
    character::complete,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult, Parser,
};

enum Bot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Clone)]
struct Resources {
    ore: u16,
    clay: u16,
    obs: u16,
    ore_b: u16,
    clay_b: u16,
    obs_b: u16,
}

impl Resources {
    pub fn tick(&mut self) {
        self.ore += self.ore_b;
        self.clay += self.clay_b;
        self.obs += self.obs_b;
    }
}

impl Default for Resources {
    fn default() -> Self {
        Resources {
            ore: Default::default(),
            clay: Default::default(),
            obs: Default::default(),
            ore_b: 1,
            clay_b: Default::default(),
            obs_b: Default::default(),
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    id: u16,
    ore_bot: u16,
    clay_bot: u16,
    obs_bot: (u16, u16),
    geo_bot: (u16, u16),
    max_ore_cost: u16,
}

impl Blueprint {
    pub fn new(
        id: u16,
        ore_bot: u16,
        clay_bot: u16,
        obs_bot: (u16, u16),
        geo_bot: (u16, u16),
    ) -> Self {
        let max_ore_cost = ore_bot.max(clay_bot).max(obs_bot.0).max(geo_bot.0);

        Blueprint {
            id,
            ore_bot,
            clay_bot,
            obs_bot,
            geo_bot,
            max_ore_cost,
        }
    }

    pub fn should_build_bot(&self, r: &Resources, bot_type: Bot) -> Option<Resources> {
        match bot_type {
            Bot::Ore => {
                if r.ore_b < self.max_ore_cost && r.ore >= self.ore_bot {
                    let mut new_r = r.clone();

                    new_r.tick();
                    new_r.ore -= self.ore_bot;
                    new_r.ore_b += 1;

                    return Some(new_r);
                }
                None
            }
            Bot::Clay => {
                if r.clay_b < self.obs_bot.1 && r.ore >= self.clay_bot {
                    let mut new_r = r.clone();

                    new_r.tick();
                    new_r.ore -= self.clay_bot;
                    new_r.clay_b += 1;

                    return Some(new_r);
                }
                None
            }
            Bot::Obsidian => {
                if r.obs_b < self.geo_bot.1 && r.ore >= self.obs_bot.0 && r.clay >= self.obs_bot.1 {
                    let mut new_r = r.clone();

                    new_r.tick();
                    new_r.ore -= self.obs_bot.0;
                    new_r.clay -= self.obs_bot.1;
                    new_r.obs_b += 1;

                    return Some(new_r);
                }
                None
            }
            Bot::Geode => {
                if r.ore >= self.geo_bot.0 && r.obs >= self.geo_bot.1 {
                    let mut new_r = r.clone();

                    new_r.tick();
                    new_r.ore -= self.geo_bot.0;
                    new_r.obs -= self.geo_bot.1;

                    return Some(new_r);
                }
                None
            }
        }
    }
}

fn parse_blueprints(input: &str) -> IResult<&str, Vec<Blueprint>> {
    separated_list1(
        tag(" obsidian.\n"),
        tuple((
            preceded(tag("Blueprint "), complete::u16),
            preceded(tag(": Each ore robot costs "), complete::u16),
            preceded(tag(" ore. Each clay robot costs "), complete::u16),
            preceded(tag(" ore. Each obsidian robot costs "), complete::u16),
            preceded(tag(" ore and "), complete::u16),
            preceded(tag(" clay. Each geode robot costs "), complete::u16),
            preceded(tag(" ore and "), complete::u16),
        ))
        .map(|t| Blueprint::new(t.0, t.1, t.2, (t.3, t.4), (t.5, t.6))),
    )(input)
}

#[cached(
    key = "String",
    convert = r#"{
        format!("{}-{}-{}-{}-{}-{}-{}-{}-{}", bp.id, resources.ore, resources.clay, resources.obs, resources.ore_b, resources.clay_b, resources.obs_b, minutes, geodes)
    }"#
)]
fn search(bp: &Blueprint, mut resources: Resources, minutes: u16, geodes: u16) -> u16 {
    if minutes == 0 {
        return geodes;
    }

    let mut result = geodes;

    if let Some(new_resources) = bp.should_build_bot(&resources, Bot::Geode) {
        let new_geodes = geodes + (minutes - 1);
        result = result.max(search(bp, new_resources, minutes - 1, new_geodes));
    } else if let Some(new_resources) = bp.should_build_bot(&resources, Bot::Obsidian) {
        result = result.max(search(bp, new_resources, minutes - 1, geodes));
    } else if let Some(new_resources) = bp.should_build_bot(&resources, Bot::Clay) {
        result = result.max(search(bp, new_resources, minutes - 1, geodes));
    }

    if let Some(new_resources) = bp.should_build_bot(&resources, Bot::Ore) {
        result = result.max(search(bp, new_resources, minutes - 1, geodes));
    }

    resources.tick();
    result = result.max(search(bp, resources, minutes - 1, geodes));

    result
}

pub fn puzzle_1(input: &str) -> String {
    let (_, blueprints) = parse_blueprints(input).unwrap();

    let result = blueprints
        .par_iter()
        .map(|bp| search(bp, Resources::default(), 24, 0) * bp.id)
        .sum::<u16>();

    result.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let (_, blueprints) = parse_blueprints(input).unwrap();

    let result = blueprints
        .par_iter()
        .take(3)
        .map(|bp| search(bp, Resources::default(), 32, 0) as u32)
        .product::<u32>();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn p1() {
        let result = puzzle_1(INPUT);
        assert_eq!(result, "33");
    }
}
