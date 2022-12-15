use std::collections::HashSet;
use std::iter::from_fn;
use std::ops::RangeInclusive;
use regex::Regex;
use range_union_find::IntRangeUnionFind;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn parse(x: &str, y: &str) -> Coord {
        Coord::new(x.parse().unwrap(), y.parse().unwrap())
    }
    fn new(x: isize, y: isize) -> Coord {
        Coord { x, y }
    }

    fn dist_to(&self, other: &Coord) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

struct Sensor {
    coord: Coord,
    nearest_beacon: Coord,
    dist: usize,
}

impl Sensor {
    fn parse(sx: &str, sy: &str, bx: &str, by: &str) -> Sensor {
        let coord = Coord::parse(sx, sy);
        let nearest_beacon = Coord::parse(bx, by);
        let dist = coord.dist_to(&nearest_beacon);
        Sensor { coord, nearest_beacon, dist }
    }

    fn covered_range_at(&self, y: isize) -> Option<RangeInclusive<isize>> {
        let dy = self.coord.y.abs_diff(y);
        let dx = self.dist as isize - dy as isize;
        if dx >= 0 {
            Some((self.coord.x-dx)..=(self.coord.x+dx))
        } else {
            None
        }
    }

    fn contains(&self, point: &Coord) -> bool {
        self.coord.dist_to(point) <= self.dist
    }

    fn border_iter(&self) -> impl Iterator<Item = Coord> + '_ {
        let border_dist = (self.dist + 1) as isize;
        let left = Coord::new(self.coord.x - border_dist, self.coord.y);
        let right = Coord::new(self.coord.x + border_dist, self.coord.y);
        let top = Coord::new(self.coord.x, self.coord.y - border_dist);
        let bottom = Coord::new(self.coord.x, self.coord.y + border_dist);

        let left_to_top = coord_iterator(left.clone(), top.clone(), 1, -1);
        let top_to_right = coord_iterator(top, right.clone(), 1, 1);
        let right_to_bottom = coord_iterator(right, bottom.clone(), -1, 1);
        let bottom_to_left = coord_iterator(bottom, left, -1, -1);

        left_to_top.chain(top_to_right).chain(right_to_bottom).chain(bottom_to_left)
    }
}

fn coord_iterator(start: Coord, end: Coord, dx: isize, dy: isize) -> impl Iterator<Item = Coord> {
    let mut next = start;
    from_fn(move || {
        if next == end {
            None
        } else {
            let result = Some(next.clone());
            next = Coord::new(next.x + dx, next.y + dy);
            result
        }
    })
}

fn main() {
    let re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
    let input = include_str!("../input.txt");
    let mut sensors = vec![];
    let mut beacons = HashSet::new();
    for line in input.lines() {
        let cap = re.captures_iter(line).next().expect(line);
        let sensor = Sensor::parse(&cap[1], &cap[2], &cap[3], &cap[4]);
        beacons.insert(sensor.nearest_beacon.clone());
        sensors.push(sensor);
    }

    let target_row = 2_000_000;
    // let target_row = 10;
    let unioned_ranges = sensors.iter()
        .filter_map(|s| s.covered_range_at(target_row))
        .fold(IntRangeUnionFind::new(), |mut acc, r| { acc.insert_range(&r).unwrap(); acc });
    let total_coverage = unioned_ranges
        .into_collection::<Vec<_>>()
        .into_iter()
        .map(|r| r.end() - r.start() + 1)
        .sum::<isize>();
    let num_beacons = beacons.iter().filter(|b| b.y == target_row).count() as isize;
    println!("Part 1: {}", total_coverage - num_beacons);

    let search_max = 4_000_000;
    // let search_max = 20;
    'sensors: for sensor in &sensors {
        for border_coord in sensor.border_iter() {
            let in_search_bounds = border_coord.x >= 0 && border_coord.x <= search_max &&
                border_coord.y >= 0 && border_coord.y <= search_max;
            if !in_search_bounds {
                continue;
            }
            let is_covered = sensors.iter().any(|s| s.contains(&border_coord));
            if is_covered {
                continue;
            }
            println!("Part 2: {}", border_coord.x * 4000000 + border_coord.y);
            break 'sensors;
        }
    }
}