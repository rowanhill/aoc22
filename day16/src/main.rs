use std::collections::{HashMap, HashSet};

struct Valve {
    name: String,
    rate: usize,
    neighbours: Vec<String>,
}

fn main() {
    let input = include_str!("../input.txt");
    let map = input.lines()
        .map(|line| {
            let (l, r) = line.split_once("; ").unwrap();
            let name = (&l[6..=7]).to_string();
            let rate = (&l[23..]).parse().unwrap();
            let neighbours = (&r[22..]).split(", ").map(|s| s.trim().to_string()).collect::<Vec<String>>();
            Valve { name, rate, neighbours }
        })
        .fold(HashMap::new(), |mut acc, valve| {
            acc.insert(valve.name.clone(), valve);
            acc
        });

    /*
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II

a - B - C - D - E - f - g - H
|\_________/
i
|
J

D, B, J, H, E, C

*/

    // Calculate a map of valve name -> Vec<(non-0 valve name, cost of movement)>
    // Opportunity cost of opening a valve is (move mins + 1)*(sum of rates of all valves that remain closed)
    // Pick the valve that minimises opportunity cost
    // Repeat until all valves opened

    let num_unstuck_valves = map.values().filter(|v| v.rate > 0).count();
    let opened_valves = HashSet::new();
    let mut cache = HashMap::new();
    let max = max_released_pressure_qq(
        &map["AA"],
        30,
        0,
        0,
        num_unstuck_valves,
        &opened_valves,
        &map,
        &mut cache
    );
    println!("Part 1: {:?}", max);
}

fn max_released_pressure_qq(
    cur_valve: &Valve,
    mins_remaining: usize,
    pressure_released: usize,
    pressure_per_min: usize,
    num_unstuck_valves: usize,
    opened_valves: &HashSet<String>,
    map: &HashMap<String, Valve>,
    cache: &mut HashMap<(String, usize), usize>,
) -> usize {
    if mins_remaining <= 0 {
        return pressure_released;
    }
    let min_final_released = pressure_released + pressure_per_min * mins_remaining;
    if opened_valves.len() >= num_unstuck_valves {
        return min_final_released;
    }

    let cache_key = (cur_valve.name.clone(), mins_remaining);
    if let Some(&prev_best) = cache.get(&cache_key) {
        // If a previous path has reached this valve at the same time with a higher lower bound
        // on the total amount of pressure released, prune this branch
        if prev_best >= min_final_released {
            return 0;
        }
    }
    cache.insert(cache_key, min_final_released);

    let max_by_opening = if cur_valve.rate > 0 && !opened_valves.contains(&cur_valve.name) {
        let mut new_open = opened_valves.clone();
        new_open.insert(cur_valve.name.clone());
        max_released_pressure_qq(
            cur_valve,
            mins_remaining - 1,
            pressure_released + pressure_per_min,
            pressure_per_min + cur_valve.rate,
            num_unstuck_valves,
            &new_open,
            map,
            cache
        )
    } else {
        0
    };
    let max_by_moving = cur_valve.neighbours.iter()
        .map(|n| max_released_pressure_qq(
            &map[n],
            mins_remaining - 1,
            pressure_released + pressure_per_min,
            pressure_per_min,
            num_unstuck_valves,
            opened_valves,
            map,
            cache
        ))
        .max()
        .unwrap();

    max_by_opening.max(max_by_moving)
}
