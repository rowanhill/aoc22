use std::cmp::{max, min};
use std::collections::HashMap;

enum Substance {
    Rock,
    Sand
}

fn main() {
    use Substance::*;

    let input = include_str!("../input.txt");
    let mut map = HashMap::new();
    let mut max_y = 0;
    for line in input.lines() {
        // println!("{line}");
        let mut coord_iter = line.split(" -> ")
            .map(|s| s.split_once(",").unwrap())
            .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()));
        let mut prev = coord_iter.next().unwrap();
        max_y = max(max_y, prev.1);
        while let Some(cur) = coord_iter.next() {
            // println!("({printl:?}) to ({:?})", prev, cur);
            for x in min(prev.0, cur.0)..=max(prev.0, cur.0) {
                for y in min(prev.1, cur.1)..=max(prev.1, cur.1) {
                    // println!("({x},{y})");
                    map.insert((x, y), Rock);
                }
            }
            max_y = max(max_y, cur.1);
            prev = cur;
        }
    }

    let mut sand_pos = (500, 0);
    loop {
        match (
            map.contains_key(&(sand_pos.0 - 1, sand_pos.1 + 1)),
            map.contains_key(&(sand_pos.0, sand_pos.1 + 1)),
            map.contains_key(&(sand_pos.0 + 1, sand_pos.1 + 1))
        ) {
            (_, false, _) => { sand_pos = (sand_pos.0, sand_pos.1 + 1); },
            (false, _, _) => { sand_pos = (sand_pos.0 - 1, sand_pos.1 + 1); },
            (_, _, false) => { sand_pos = (sand_pos.0 + 1, sand_pos.1 + 1); },
            (true, true, true) => { map.insert(sand_pos, Sand); sand_pos = (500, 0); }
        }
        if sand_pos.1 > max_y {
            break;
        }
    }
    let num_sands = map.values().filter(|s| match s { Sand => true, Rock => false }).count();
    println!("Part 1: {num_sands}");

    sand_pos = (500, 0);
    loop {
        match (
            map.contains_key(&(sand_pos.0 - 1, sand_pos.1 + 1)),
            map.contains_key(&(sand_pos.0, sand_pos.1 + 1)),
            map.contains_key(&(sand_pos.0 + 1, sand_pos.1 + 1))
        ) {
            (_, false, _) => { sand_pos = (sand_pos.0, sand_pos.1 + 1); },
            (false, _, _) => { sand_pos = (sand_pos.0 - 1, sand_pos.1 + 1); },
            (_, _, false) => { sand_pos = (sand_pos.0 + 1, sand_pos.1 + 1); },
            (true, true, true) => {
                map.insert(sand_pos, Sand);
                if sand_pos == (500, 0) {
                    break;
                }
                sand_pos = (500, 0); }
        }
        if sand_pos.1 == max_y + 1 {
            map.insert(sand_pos, Sand);
            sand_pos = (500, 0);
        }
    }
    let num_sands = map.values().filter(|s| match s { Sand => true, Rock => false }).count();
    println!("Part 2: {num_sands}");
}
