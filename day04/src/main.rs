use std::fs::read_to_string;
use std::ops::{RangeBounds, RangeInclusive};
use std::str::FromStr;

trait ElfWorkAssignment<T> where T: FromStr, Self: RangeBounds<T> {
    fn parse(s: &str) -> Self;
    fn overlaps(&self, other: &Self) -> bool;
    fn contains_completely(&self, other: &Self) -> bool;
}

impl <T> ElfWorkAssignment<T> for RangeInclusive<T> where T: FromStr + PartialOrd {
    fn parse(s: &str) -> Self {
        let (left, right) = s.split_once("-")
            .unwrap_or_else(|| panic!("Unexpected range format: {}", s));
        let start = left.parse::<T>().unwrap_or_else(|_| panic!("Unexpected range start: {}", left));
        let end = right.parse::<T>().unwrap_or_else(|_| panic!("Unexpected range end: {}", right));
        start..=end
    }

    fn overlaps(&self, other: &Self) -> bool {
        other.start() <= self.end() && other.end() >= self.start()
    }

    fn contains_completely(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }
}

fn main() {
    let input = read_to_string("input.txt").expect("Could not read input file");

    let range_pairs = input.lines().map(|line: &str| {
        let (lhs, rhs) = line.split_once(",").expect("No comma");
        (RangeInclusive::<u64>::parse(lhs), RangeInclusive::<u64>::parse(rhs))
    }).collect::<Vec<_>>();

    let part1 = range_pairs.iter().filter(|(range1, range2)| {
        range1.contains_completely(range2) || range2.contains_completely(range1)
    }).count();
    println!("Part 1: {}", part1);

    let part2 = range_pairs.iter().filter(|(range1, range2)| {
        range1.overlaps(range2) || range2.overlaps(range1)
    }).count();
    println!("Part 2: {}", part2);
}
