use std::collections::HashMap;
use std::time::Instant;

struct Valve {
    rate: usize,
    neighbours: Vec<String>,
}

fn is_open(open_valves: u64, valve_index: usize) -> bool {
    open_valves & (1 << valve_index) > 0
}

fn open(open_valves: u64, valve_index: usize) -> u64 {
    open_valves | (1 << valve_index)
}

fn max_released_pressure_solo(
    valve_index: usize,
    mins_remaining: u16,
    pressure_relieved: usize,
    open_valves: u64,
    all_open: u64,
    valves: &Vec<Valve>,
    usable_valve_indexes: &Vec<usize>,
    dists: &Vec<Vec<usize>>,
) -> usize {
    if mins_remaining <= 0 {
        return pressure_relieved;
    }
    if open_valves == all_open {
        return pressure_relieved;
    }

    let max = usable_valve_indexes.iter()
        .filter(|&&i|
            i != valve_index &&
                dists[valve_index][i] + 1 <= mins_remaining as usize &&
                !is_open(open_valves, i)
        )
        .map(|&new_valve_index| {
            let time_to_move_and_open = dists[valve_index][new_valve_index] + 1;
            let new_time_remaining = mins_remaining - time_to_move_and_open as u16;
            max_released_pressure_solo(
                new_valve_index,
                new_time_remaining,
                pressure_relieved + valves[new_valve_index].rate * new_time_remaining as usize,
                open(open_valves, new_valve_index),
                all_open,
                valves,
                usable_valve_indexes,
                dists
            )
        })
        .max();
    if let Some(max) = max {
        if max == 2541 {
            println!();
        }
        max
    } else {
        pressure_relieved
    }
}

fn max_released_pressure_duo(
    my_valve_index: usize,
    el_valve_index: usize,
    my_mins_remaining: u16,
    el_mins_remaining: u16,
    pressure_relieved: usize,
    open_valves: u64,
    all_open: u64,
    valves: &Vec<Valve>,
    usable_valve_indexes: &Vec<usize>,
    dists: &Vec<Vec<usize>>,
    cache: &mut HashMap<(usize, usize, u16, u16, u64), usize>,
) -> usize {
    if my_mins_remaining <= 0 && el_mins_remaining <= 0 {
        return pressure_relieved;
    }
    if open_valves == all_open {
        return pressure_relieved;
    }

    let cache_key = (my_valve_index, el_valve_index, my_mins_remaining, el_mins_remaining, open_valves);
    if let Some(&prev_pressure_relieved) = cache.get(&cache_key) {
        if prev_pressure_relieved >= pressure_relieved {
            return 0;
        }
    }
    cache.insert(cache_key, pressure_relieved);

    let mut max = pressure_relieved;

    // Only I move
    let my_move_max = usable_valve_indexes.iter()
        .filter(|&&i|
            i != my_valve_index &&
                dists[my_valve_index][i] + 1 <= my_mins_remaining as usize &&
                !is_open(open_valves, i)
        )
        .map(|&my_new_valve_index| {
            let time_to_move_and_open = dists[my_valve_index][my_new_valve_index] + 1;
            let my_new_time_remaining = my_mins_remaining - time_to_move_and_open as u16;
            max_released_pressure_duo(
                my_new_valve_index,
                el_valve_index,
                my_new_time_remaining,
                el_mins_remaining,
                pressure_relieved + valves[my_new_valve_index].rate * my_new_time_remaining as usize,
                open(open_valves, my_new_valve_index),
                all_open,
                valves,
                usable_valve_indexes,
                dists,
                cache
            )
        })
        .max();
    if let Some(m) = my_move_max {
        max = max.max(m);
    }

    // Only elephant moves
    let el_move_max = usable_valve_indexes.iter()
        .filter(|&&i|
            i != el_valve_index &&
                dists[el_valve_index][i] + 1 <= el_mins_remaining as usize &&
                !is_open(open_valves, i)
        )
        .map(|&el_new_valve_index| {
            let time_to_move_and_open = dists[el_valve_index][el_new_valve_index] + 1;
            let el_new_time_remaining = el_mins_remaining - time_to_move_and_open as u16;
            max_released_pressure_duo(
                my_valve_index,
                el_new_valve_index,
                my_mins_remaining,
                el_new_time_remaining,
                pressure_relieved + valves[el_new_valve_index].rate * el_new_time_remaining as usize,
                open(open_valves, el_new_valve_index),
                all_open,
                valves,
                usable_valve_indexes,
                dists,
                cache
            )
        })
        .max();
    if let Some(m) = el_move_max {
        max = max.max(m);
    }

    // Both move
    let both_move_max = usable_valve_indexes.iter()
        .filter(|&&i|
            i != my_valve_index &&
                dists[my_valve_index][i] + 1 <= my_mins_remaining as usize &&
                !is_open(open_valves, i)
        )
        .flat_map(|&my_new_valve_index| {
            let my_time_to_move_and_open = dists[my_valve_index][my_new_valve_index] + 1;
            let my_new_time_remaining = my_mins_remaining - my_time_to_move_and_open as u16;

            usable_valve_indexes.iter()
                .filter(|&&i|
                    i != el_valve_index &&
                        i != my_new_valve_index &&
                        dists[el_valve_index][i] + 1 < el_mins_remaining as usize &&
                        !is_open(open_valves, i)
                )
                .map(|&el_new_valve_index| {
                    let el_time_to_move_and_open = dists[el_valve_index][el_new_valve_index] + 1;
                    let el_new_time_remaining = el_mins_remaining - el_time_to_move_and_open as u16;
                    max_released_pressure_duo(
                        my_new_valve_index,
                        el_new_valve_index,
                        my_new_time_remaining,
                        el_new_time_remaining,
                        pressure_relieved + valves[my_new_valve_index].rate * my_new_time_remaining as usize + valves[el_new_valve_index].rate * el_new_time_remaining as usize,
                        open(open(open_valves, my_new_valve_index), el_new_valve_index),
                        all_open,
                        valves,
                        usable_valve_indexes,
                        dists,
                        cache
                    )
                })
                .max()
        })
        .max();
    if let Some(m) = both_move_max {
        max = max.max(m);
    }
    max
}

fn main() {
    let input = include_str!("../input.txt");
    let mut valve_index_by_name = HashMap::new();
    let valves = input.lines()
        .enumerate()
        .map(|(i, line)| {
            let (l, r) = line.split_once("; ").unwrap();
            let name = (&l[6..=7]).to_string();
            let rate = (&l[23..]).parse().unwrap();
            let neighbours = (&r[22..]).split(", ").map(|s| s.trim().to_string()).collect::<Vec<String>>();
            valve_index_by_name.insert(name.clone(), i);
            Valve { rate, neighbours }
        })
        .collect::<Vec<Valve>>();

    // Floyd-Warshall to calculate min dists between all valves
    let mut dists = vec![vec![1000; valves.len()]; valves.len()];
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
                if dists[i][j] > dists[i][k] + dists[k][j] {
                    dists[i][j] = dists[i][k] + dists[k][j];
                }
            }
        }
    }

    let usable_valve_indexes = valves.iter()
        .enumerate()
        .filter(|(_, v)| v.rate > 0)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let mut all_open = 0;
    for uvi in &usable_valve_indexes {
        all_open = open(all_open, *uvi);
    }

    let max = max_released_pressure_solo(
        valve_index_by_name["AA"],
        30,
        0,
        0,
        all_open,
        &valves,
        &usable_valve_indexes,
        &dists
    );
    println!("Part 1: {}", max);

    let mut cache = HashMap::new();
    let max = max_released_pressure_duo(
        valve_index_by_name["AA"],
        valve_index_by_name["AA"],
        26,
        26,
        0,
        0,
        all_open,
        &valves,
        &usable_valve_indexes,
        &dists,
        &mut cache
    );
    println!("Part 2: {}", max);
}