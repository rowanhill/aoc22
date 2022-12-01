use std::cmp::Reverse;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read input file");

    let mut max = 0u32;
    let mut cur = 0;
    let mut vals = vec![];
    for line in contents.lines() {
        if line.is_empty() {
            if cur > max {
                max = cur;
            }
            vals.push(cur);
            cur = 0;
        } else {
            let val: u32 = line.parse().expect("Could not parse line");
            cur += val;
        }
    }

    println!("Part 1: {}", max);

    vals.sort_by_key(|v| Reverse(*v));
    let top_three: u32 = vals[0..=2].iter().sum();

    println!("Part 2: {}", top_three);
}
