use std::collections::VecDeque;
use regex::Regex;

struct Blueprint {
    id: usize,
    ore_ore: usize,
    clay_ore: usize,
    obsidian_ore: usize,
    obsidian_clay: usize,
    geode_ore: usize,
    geode_obsidian: usize,
    max_ore: usize, // the max ore required to build any of the robots
}

struct State {
    ore: usize, clay: usize, obsidian: usize, geode: usize,
    ore_robots: usize, clay_robots: usize, obsidian_robots: usize, geode_robots: usize,
    time_remaining: usize,
}

impl State {
    fn default(time_remaining: usize) -> State {
        State {
            ore: 0, clay: 0, obsidian: 0, geode: 0,
            ore_robots: 1, clay_robots: 0, obsidian_robots: 0, geode_robots: 0,
            time_remaining
        }
    }

    fn idle_for(&self, mins: usize) -> State {
        State {
            ore: self.ore + (self.ore_robots * mins),
            clay: self.clay + (self.clay_robots * mins),
            obsidian: self.obsidian + (self.obsidian_robots * mins),
            geode: self.geode + (self.geode_robots * mins),
            time_remaining: self.time_remaining - mins,
            ..*self
        }
    }

    fn build_ore_robot(&self, ore_cost: usize) -> State {
        State {
            ore: self.ore + self.ore_robots - ore_cost,
            clay: self.clay + self.clay_robots,
            obsidian: self.obsidian + self.obsidian_robots,
            geode: self.geode + self.geode_robots,
            time_remaining: self.time_remaining - 1,
            ore_robots: self.ore_robots + 1,
            ..*self
        }
    }

    fn build_clay_robot(&self, ore_cost: usize) -> State {
        State {
            ore: self.ore + self.ore_robots - ore_cost,
            clay: self.clay + self.clay_robots,
            obsidian: self.obsidian + self.obsidian_robots,
            geode: self.geode + self.geode_robots,
            time_remaining: self.time_remaining - 1,
            clay_robots: self.clay_robots + 1,
            ..*self
        }
    }

    fn build_obsidian_robot(&self, ore_cost: usize, clay_cost: usize) -> State {
        State {
            ore: self.ore + self.ore_robots - ore_cost,
            clay: self.clay + self.clay_robots - clay_cost,
            obsidian: self.obsidian + self.obsidian_robots,
            geode: self.geode + self.geode_robots,
            time_remaining: self.time_remaining - 1,
            obsidian_robots: self.obsidian_robots + 1,
            ..*self
        }
    }

    fn build_geode_robot(&self, ore_cost: usize, obsidian_cost: usize) -> State {
        State {
            ore: self.ore + self.ore_robots - ore_cost,
            clay: self.clay + self.clay_robots,
            obsidian: self.obsidian + self.obsidian_robots - obsidian_cost,
            geode: self.geode + self.geode_robots,
            time_remaining: self.time_remaining - 1,
            geode_robots: self.geode_robots + 1,
            ..*self
        }
    }
}

impl Blueprint {
    fn maximal_geodes(&self, time_limit: usize) -> usize {
        let mut queue = VecDeque::new();
        queue.push_back(State::default(time_limit));

        let mut max_geodes = 0;
        while let Some(state) = queue.pop_front() {
            if state.geode > max_geodes {
                max_geodes = state.geode;
            }
            if state.time_remaining == 0 {
                continue;
            }

            let mut can_build = false;

            // Build an ore robot
            if state.ore_robots > 0 {
                let ore_required = self.ore_ore.saturating_sub(state.ore);
                // Ceil int division
                let mins_required = (ore_required + state.ore_robots - 1) / state.ore_robots;
                if mins_required < state.time_remaining {
                    let ore_deficit_per_min = self.max_ore as isize - state.ore_robots as isize;
                    let ore_stockpile_required = ore_deficit_per_min * state.time_remaining as isize;
                    if ore_stockpile_required > state.ore as isize {
                        let next_state = state.idle_for(mins_required)
                            .build_ore_robot(self.ore_ore);
                        queue.push_back(next_state);
                        can_build = true;
                    }
                }
            }

            // Build a clay robot
            if state.ore_robots > 0 {
                let ore_required = self.clay_ore.saturating_sub(state.ore);
                // Ceil int division
                let mins_required = (ore_required + state.ore_robots - 1) / state.ore_robots;
                if mins_required < state.time_remaining {
                    let clay_deficit_per_min = state.clay_robots as isize - self.obsidian_clay as isize;
                    let clay_stockpile_required = clay_deficit_per_min * state.time_remaining as isize;
                    if clay_stockpile_required < state.clay as isize {
                        let next_state = state.idle_for(mins_required)
                            .build_clay_robot(self.clay_ore);
                        queue.push_back(next_state);
                        can_build = true;
                    }
                }
            }

            // Build an obsidian robot
            if state.ore_robots > 0 && state.clay_robots > 0 {
                let ore_required = self.obsidian_ore.saturating_sub(state.ore);
                // Ceil int division
                let ore_mins_required = (ore_required + state.ore_robots - 1) / state.ore_robots;
                let clay_required = self.obsidian_clay.saturating_sub(state.clay);
                // Ceil int division
                let clay_mins_required = (clay_required + state.clay_robots - 1) / state.clay_robots;
                let mins_required = ore_mins_required.max(clay_mins_required);

                if mins_required < state.time_remaining {
                    let obsidian_deficit_per_min = state.obsidian_robots as isize - self.geode_obsidian as isize;
                    let obsidian_stockpile_required = obsidian_deficit_per_min * state.time_remaining as isize;
                    if obsidian_stockpile_required < state.obsidian as isize {
                        let next_state = state.idle_for(mins_required)
                            .build_obsidian_robot(self.obsidian_ore, self.obsidian_clay);
                        queue.push_back(next_state);
                        can_build = true;
                    }
                }
            }

            // Build a geode robot
            if state.ore_robots > 0 && state.obsidian_robots > 0 {
                let ore_required = self.geode_ore.saturating_sub(state.ore);
                // Ceil int division
                let ore_mins_required = (ore_required + state.ore_robots - 1) / state.ore_robots;
                let obsidian_required = self.geode_obsidian.saturating_sub(state.obsidian);
                // Ceil int division
                let obsidian_mins_required = (obsidian_required + state.obsidian_robots - 1) / state.obsidian_robots;
                let mins_required = ore_mins_required.max(obsidian_mins_required);

                if mins_required < state.time_remaining {
                    let next_state = state.idle_for(mins_required)
                        .build_geode_robot(self.geode_ore, self.geode_obsidian);
                    queue.push_back(next_state);
                    can_build = true;
                }
            }

            if !can_build {
                let next_state = State {
                    ore: state.ore + (state.ore_robots * state.time_remaining),
                    clay: state.clay + (state.clay_robots * state.time_remaining),
                    obsidian: state.obsidian + (state.obsidian_robots * state.time_remaining),
                    geode: state.geode + (state.geode_robots * state.time_remaining),
                    time_remaining: 0,
                    ..state
                };
                queue.push_back(next_state);
            }
        }
        max_geodes
    }

    fn quality_level(&self, time_limit: usize) -> usize {
        self.maximal_geodes(time_limit) * self.id
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let re = Regex::new(r"Blueprint (?P<id>\d+): Each ore robot costs (?P<ore_ore>\d+) ore. Each clay robot costs (?P<clay_ore>\d+) ore. Each obsidian robot costs (?P<obsidian_ore>\d+) ore and (?P<obsidian_clay>\d+) clay. Each geode robot costs (?P<geode_ore>\d+) ore and (?P<geode_obsidian>\d+) obsidian.").unwrap();
    let blueprints: Vec<_> = input.lines().map(|line| {
        let caps = re.captures(line).unwrap();
        let id = *&caps["id"].parse().unwrap();
        let ore_ore = *&caps["ore_ore"].parse().unwrap();
        let clay_ore = *&caps["clay_ore"].parse().unwrap();
        let obsidian_ore = *&caps["obsidian_ore"].parse().unwrap();
        let obsidian_clay = *&caps["obsidian_clay"].parse().unwrap();
        let geode_ore = *&caps["geode_ore"].parse().unwrap();
        let geode_obsidian = *&caps["geode_obsidian"].parse().unwrap();
        let max_ore = *[ore_ore, clay_ore, obsidian_ore, geode_ore].iter().max().unwrap();
        Blueprint { id, ore_ore, clay_ore, obsidian_ore, obsidian_clay, geode_ore, geode_obsidian, max_ore }
    }).collect();

    let sum_quality: usize = blueprints.iter().map(|b| b.quality_level(24)).sum();
    println!("Part 1: {}", sum_quality);

    let product_max_geodes: usize = blueprints[0..3].iter().map(|b| b.maximal_geodes(32)).product();
    println!("Part 2: {}", product_max_geodes);
}

#[cfg(test)]
mod test {
    use crate::Blueprint;

    /*
Blueprint 1:
  Each ore robot costs 4 ore.
  Each clay robot costs 2 ore.
  Each obsidian robot costs 3 ore and 14 clay.
  Each geode robot costs 2 ore and 7 obsidian.
     */
    #[test]
    fn example_1_quality() {
        let b = Blueprint {
            id: 1,
            ore_ore: 4,
            clay_ore: 2,
            obsidian_ore: 3, obsidian_clay: 14,
            geode_ore: 2, geode_obsidian: 7,
            max_ore: 4,
        };
        let q = b.quality_level(24);
        assert_eq!(q, 9);
    }

    /*
Blueprint 2:
  Each ore robot costs 2 ore.
  Each clay robot costs 3 ore.
  Each obsidian robot costs 3 ore and 8 clay.
  Each geode robot costs 3 ore and 12 obsidian
     */
    #[test]
    fn example_2_quality() {
        let b = Blueprint {
            id: 2,
            ore_ore: 2,
            clay_ore: 3,
            obsidian_ore: 3, obsidian_clay: 8,
            geode_ore: 3, geode_obsidian: 12,
            max_ore: 3,
        };
        let q = b.quality_level(24);
        assert_eq!(q, 24);
    }
}