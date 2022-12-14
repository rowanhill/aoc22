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
        let mut coord_iter = line.split(" -> ")
            .map(|s| s.split_once(",").unwrap())
            .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()));
        let mut prev = coord_iter.next().unwrap();
        max_y = max(max_y, prev.1);
        while let Some(cur) = coord_iter.next() {
            for x in min(prev.0, cur.0)..=max(prev.0, cur.0) {
                for y in min(prev.1, cur.1)..=max(prev.1, cur.1) {
                    map.insert((x, y), Rock);
                }
            }
            max_y = max(max_y, cur.1);
            prev = cur;
        }
    }

    let mut pt1_printed = false;
    let mut sand_count = 0;
    let mut path = vec![];
    path.push((500, 0));
    loop {
        let sand_pos = path.last().unwrap();
        let new_pos = match (
            map.contains_key(&(sand_pos.0 - 1, sand_pos.1 + 1)) || sand_pos.1 >= max_y + 1,
            map.contains_key(&(sand_pos.0, sand_pos.1 + 1)) || sand_pos.1 >= max_y + 1,
            map.contains_key(&(sand_pos.0 + 1, sand_pos.1 + 1)) || sand_pos.1 >= max_y + 1
        ) {
            (_, false, _) => Some((sand_pos.0, sand_pos.1 + 1)),
            (false, _, _) => Some((sand_pos.0 - 1, sand_pos.1 + 1)),
            (_, _, false) => Some((sand_pos.0 + 1, sand_pos.1 + 1)),
            (true, true, true) => { map.insert(sand_pos.clone(), Sand); sand_count += 1; None }
        };
        if let Some(p) = new_pos {
            path.push(p);
            if !pt1_printed && p.1 > max_y {
                println!("Part 1: {sand_count}");
                pt1_printed = true;
            }
        } else {
            path.pop();
        }
        if path.is_empty() {
            break;
        }
    }
    println!("Part 2: {sand_count}");
}
