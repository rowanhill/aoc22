use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;

fn is_marker(buffer: &VecDeque<char>) -> bool {
    let hashset: HashSet<_> = buffer.iter().collect();
    hashset.len() == buffer.len()
}

fn find_position_of_marker(input: &str, len: usize) -> usize {
    let mut buffer = VecDeque::new();
    let mut input_chars = input.chars();
    while buffer.len() < len {
        buffer.push_back(input_chars.next().unwrap());
    }
    let mut marker_count = len;
    while !is_marker(&buffer) {
        buffer.pop_front();
        buffer.push_back(input_chars.next().unwrap());
        marker_count += 1;
    }
    marker_count
}

fn main() {
    let input = read_to_string("input.txt").expect("Could not read input file");

    println!("Part 1: {}", find_position_of_marker(&input, 4));
    println!("Part 2: {}", find_position_of_marker(&input, 14));

    // Second implementation, just for fun:
    let p1 = input.as_bytes()
        .windows(4)
        .position(|buffer| buffer.iter().collect::<HashSet<_>>().len() == buffer.len())
        .unwrap() + 4;
    println!("{}", p1);
    let p2 = input.as_bytes()
        .windows(14)
        .position(|buffer| buffer.iter().collect::<HashSet<_>>().len() == buffer.len())
        .unwrap() + 14;
    println!("{}", p2);
}