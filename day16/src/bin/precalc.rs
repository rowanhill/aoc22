use std::collections::{HashMap, VecDeque};

struct Valve {
    rate: usize,
    neighbours: Vec<String>,
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct BitValves(u64);

impl BitValves {
    fn new() -> BitValves {
        BitValves(0)
    }

    fn is_open(&self, valve_index: usize) -> bool {
        self.0 & (1 << valve_index) > 0
    }

    fn open(&self, valve_index: usize) -> BitValves {
        BitValves(self.0 | (1 << valve_index))
    }

    fn disjoint_with(&self, other: &Self) -> bool {
        self.0 & other.0 == 0
    }
}

fn parse_valves(input: &str) -> (Vec<Valve>, HashMap<String, usize>) {
    let mut valve_index_by_name = HashMap::new();
    let valves = input.lines()
        .enumerate()
        .map(|(i, line): (usize, &str)| {
            let (l, r) = line.split_once("; ").unwrap();
            let name = (&l[6..=7]).to_string();
            let rate = (&l[23..]).parse().unwrap();
            let neighbours = (&r[22..]).split(", ").map(|s| s.trim().to_string()).collect::<Vec<String>>();
            valve_index_by_name.insert(name.clone(), i);
            Valve { rate, neighbours }
        })
        .collect::<Vec<Valve>>();
    (valves, valve_index_by_name)
}

fn calculate_all_shortest_distances(valves: &Vec<Valve>, valve_index_by_name: &HashMap<String, usize>) -> Vec<Vec<usize>> {
    // Floyd-Warshall to calculate min dists between all valves
    let mut dists = vec![vec![usize::MAX; valves.len()]; valves.len()];
    for (i, valve) in valves.iter().enumerate() {
        for n in &valve.neighbours {
            let j = &valve_index_by_name[n];
            dists[i][*j] = 1;
        }
        dists[i][i] = 0;
    }
    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                if dists[i][j] > dists[i][k].saturating_add(dists[k][j]) {
                    dists[i][j] = dists[i][k].saturating_add(dists[k][j]);
                }
            }
        }
    }
    dists
}

fn usable_valve_indexes(valves: &Vec<Valve>) -> Vec<usize> {
    valves.iter()
        .enumerate()
        .filter(|(_, v)| v.rate > 0)
        .map(|(i, _)| i)
        .collect()
}

fn max_pressure_release_for_all_combinations(
    valves: &Vec<Valve>,
    usable_valve_indexes: &Vec<usize>,
    start_valve_index: usize,
    min_dists: &Vec<Vec<usize>>,
    mins_allowed: usize
) -> HashMap<BitValves, usize> {
    let mut result = HashMap::new();

    struct SearchState { valve: usize, mins_remaining: usize, open_valves: BitValves, pressure: usize }

    let mut queue = VecDeque::new();
    queue.push_back(SearchState {
        valve: start_valve_index,
        mins_remaining: mins_allowed,
        open_valves: BitValves::new(),
        pressure: 0
    });

    while let Some(SearchState { valve, mins_remaining, open_valves, pressure }) = queue.pop_back() {
        // If this is the best pressure released for this set of open valves we've seen so far, store it in result
        let cur_best = *result.get(&open_valves).unwrap_or(&0);
        let new_best = cur_best.max(pressure);
        result.insert(open_valves.clone(), new_best);

        // Find all the closed valves we still have time to go to and open
        let available_valves_and_dists = usable_valve_indexes.iter()
            .filter(|i| !open_valves.is_open(**i))
            .map(|i| (i, min_dists[valve][*i]))
            .filter(|(_, d)| mins_remaining > *d);

        for (&next_valve, dist) in available_valves_and_dists {
            let new_mins_remaining = mins_remaining - dist - 1;
            let state = SearchState {
                valve: next_valve,
                mins_remaining: new_mins_remaining,
                open_valves: open_valves.open(next_valve),
                pressure: pressure + valves[next_valve].rate * new_mins_remaining,
            };
            queue.push_back(state);
        }
    }

    result
}

fn max_pressure_release(
    valves: &Vec<Valve>,
    usable_valve_indexes: &Vec<usize>,
    start_valve_index: usize,
    min_dists: &Vec<Vec<usize>>,
    mins_allowed: usize
) -> usize {
    let max_releases_by_valves = max_pressure_release_for_all_combinations(
        valves,
        usable_valve_indexes,
        start_valve_index,
        min_dists,
        mins_allowed,
    );
    *max_releases_by_valves.values().max().unwrap()
}

fn max_pressure_release_by_two_agents(
    valves: &Vec<Valve>,
    usable_valve_indexes: &Vec<usize>,
    start_valve_index: usize,
    min_dists: &Vec<Vec<usize>>,
    mins_allowed: usize
) -> usize {
    let max_released_by_valves = max_pressure_release_for_all_combinations(
        valves,
        usable_valve_indexes,
        start_valve_index,
        min_dists,
        mins_allowed,
    );
    let mut max_combined = 0;
    for (subset_one_valves, subset_one_pressure) in &max_released_by_valves {
        for (subset_two_valves, subset_two_pressure) in &max_released_by_valves {
            if subset_one_valves.disjoint_with(subset_two_valves) {
                max_combined = max_combined.max(subset_one_pressure + subset_two_pressure);
            }
        }
    }
    max_combined
}

fn main() {
    let input = include_str!("../../input.txt");
    let (valves, valve_index_by_name) = parse_valves(input);
    let min_dists = calculate_all_shortest_distances(&valves, &valve_index_by_name);
    let usable_valve_indexes = usable_valve_indexes(&valves);

    let part1 = max_pressure_release(&valves, &usable_valve_indexes, valve_index_by_name["AA"], &min_dists, 30);
    println!("Part 1: {}", part1);

    let part2 = max_pressure_release_by_two_agents(&valves, &usable_valve_indexes, valve_index_by_name["AA"], &min_dists, 26);
    println!("Part 2: {}", part2);
}