use std::cmp;

pub struct Blueprint {
    ore_robot_ore_cost: u32,
    clay_robot_ore_cost: u32,
    obsidian_robot_ore_cost: u32,
    obsidian_robot_clay_cost: u32,
    geode_robot_ore_cost: u32,
    geode_robot_obsidian_cost: u32,
}

pub fn generator(s: &str) -> Vec<Blueprint> {
    let mut result = Vec::with_capacity(1024);
    for line in s.lines() {
        // Skip until colon, like `Blueprint 1: `
        let (_blueprint_num, line) = line.split_once(": ").unwrap();
        let (ore_robot_ore_cost, line) = line
            .strip_prefix("Each ore robot costs ")
            .unwrap()
            .split_once(" ")
            .unwrap();
        let (clay_robot_ore_cost, line) = line
            .strip_prefix("ore. Each clay robot costs ")
            .unwrap()
            .split_once(" ")
            .unwrap();
        let (obsidian_robot_ore_cost, line) = line
            .strip_prefix("ore. Each obsidian robot costs ")
            .unwrap()
            .split_once(" ")
            .unwrap();
        let (obsidian_robot_clay_cost, line) = line
            .strip_prefix("ore and ")
            .unwrap()
            .split_once(" ")
            .unwrap();
        let (geode_robot_ore_cost, line) = line
            .strip_prefix("clay. Each geode robot costs ")
            .unwrap()
            .split_once(" ")
            .unwrap();
        let (geode_robot_obsidian_cost, line) = line
            .strip_prefix("ore and ")
            .unwrap()
            .split_once(" ")
            .unwrap();
        assert_eq!(line, "obsidian.");

        result.push(Blueprint {
            ore_robot_ore_cost: ore_robot_ore_cost.parse().unwrap(),
            clay_robot_ore_cost: clay_robot_ore_cost.parse().unwrap(),
            obsidian_robot_ore_cost: obsidian_robot_ore_cost.parse().unwrap(),
            obsidian_robot_clay_cost: obsidian_robot_clay_cost.parse().unwrap(),
            geode_robot_ore_cost: geode_robot_ore_cost.parse().unwrap(),
            geode_robot_obsidian_cost: geode_robot_obsidian_cost.parse().unwrap(),
        })
    }
    result
}

pub fn part_1(blueprints: &[Blueprint]) -> u32 {
    const MINUTES: u32 = 24;

    let mut result = 0;
    for (i, blueprint) in blueprints.iter().enumerate() {
        let id_num = u32::try_from(i + 1).unwrap();
        let best_score = best_blueprint_score(blueprint, MINUTES);
        result += id_num * u32::from(best_score);
    }

    result
}

pub fn part_2(blueprints: &[Blueprint]) -> u32 {
    const MINUTES: u32 = 32;

    let mut result = 1;
    for blueprint in blueprints.iter().take(3) {
        let best_score = best_blueprint_score(blueprint, MINUTES);
        result *= u32::from(best_score);
    }

    result
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
struct RateCount {
    rate: u32,
    count: u32,
}

impl RateCount {
    fn after_minutes(self, minutes: u32) -> Self {
        Self {
            rate: self.rate,
            count: self.count + self.rate * minutes,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct State {
    minutes_left: u32,

    ore: RateCount,
    clay: RateCount,
    obsidian: RateCount,
    geode: RateCount,
}

impl State {
    fn new(minutes_left: u32) -> Self {
        Self {
            minutes_left,

            ore: RateCount { rate: 1, count: 0 },
            clay: RateCount::default(),
            obsidian: RateCount::default(),
            geode: RateCount::default(),
        }
    }

    fn after_minutes(&self, minutes: u32) -> Self {
        Self {
            minutes_left: self.minutes_left - minutes,
            ore: self.ore.after_minutes(minutes),
            clay: self.clay.after_minutes(minutes),
            obsidian: self.obsidian.after_minutes(minutes),
            geode: self.geode.after_minutes(minutes),
        }
    }
}

fn best_blueprint_score(blueprint: &Blueprint, minutes: u32) -> u32 {
    let mut best_score = 0;

    let most_needed_ore = blueprint
        .ore_robot_ore_cost
        .max(blueprint.clay_robot_ore_cost)
        .max(blueprint.obsidian_robot_ore_cost)
        .max(blueprint.geode_robot_ore_cost);
    let most_needed_clay = blueprint.obsidian_robot_clay_cost;
    let most_needed_obsidian = blueprint.geode_robot_obsidian_cost;

    let mut queue = Vec::with_capacity(1024);
    queue.push(State::new(minutes));
    while let Some(state) = queue.pop() {
        best_score = best_score.max(state.geode.count + state.geode.rate * state.minutes_left);

        // Quick exit if building geodes every minute can't even beat best
        if state.geode.count
            + (state.geode.rate + (state.minutes_left + 1) / 2) * state.minutes_left
            < best_score
        {
            continue;
        }

        if state.ore.rate < most_needed_ore {
            // Try building an ore robot
            let next_ore = (blueprint.ore_robot_ore_cost.saturating_sub(state.ore.count)
                + (state.ore.rate - 1))
                / state.ore.rate;
            if next_ore < state.minutes_left {
                let mut new_state = state.after_minutes(next_ore + 1);
                new_state.ore.count -= blueprint.ore_robot_ore_cost;
                new_state.ore.rate += 1;
                queue.push(new_state);
            }
        }

        if state.clay.rate < most_needed_clay {
            // Try building a clay robot
            let next_clay = (blueprint
                .clay_robot_ore_cost
                .saturating_sub(state.ore.count)
                + (state.ore.rate - 1))
                / state.ore.rate;
            if next_clay < state.minutes_left {
                let mut new_state = state.after_minutes(next_clay + 1);
                new_state.ore.count -= blueprint.clay_robot_ore_cost;
                new_state.clay.rate += 1;
                queue.push(new_state);
            }
        }

        if state.obsidian.rate < most_needed_obsidian && state.clay.rate > 0 {
            let next_obsidian = cmp::max(
                (blueprint
                    .obsidian_robot_ore_cost
                    .saturating_sub(state.ore.count)
                    + (state.ore.rate - 1))
                    / state.ore.rate,
                (blueprint
                    .obsidian_robot_clay_cost
                    .saturating_sub(state.clay.count)
                    + (state.clay.rate - 1))
                    / state.clay.rate,
            );
            if next_obsidian < state.minutes_left {
                let mut new_state = state.after_minutes(next_obsidian + 1);
                new_state.ore.count -= blueprint.obsidian_robot_ore_cost;
                new_state.clay.count -= blueprint.obsidian_robot_clay_cost;
                new_state.obsidian.rate += 1;
                queue.push(new_state);
            }
        }
        // Try building a geode robot
        if state.obsidian.rate > 0 {
            let next_geode = cmp::max(
                (blueprint
                    .geode_robot_ore_cost
                    .saturating_sub(state.ore.count)
                    + (state.ore.rate - 1))
                    / state.ore.rate,
                (blueprint
                    .geode_robot_obsidian_cost
                    .saturating_sub(state.obsidian.count)
                    + (state.obsidian.rate - 1))
                    / state.obsidian.rate,
            );
            if next_geode < state.minutes_left {
                let mut new_state = state.after_minutes(next_geode + 1);
                new_state.ore.count -= blueprint.geode_robot_ore_cost;
                new_state.obsidian.count -= blueprint.geode_robot_obsidian_cost;
                new_state.geode.rate += 1;
                queue.push(new_state);
            }
        }
    }

    best_score
}

super::day_test! {demo_1 == 33}
super::day_test! {demo_2 == 3472}
super::day_test! {part_1 == 1150}
super::day_test! {part_2 == 37367}
