use std::collections::{HashSet, VecDeque};
use std::ops::RangeInclusive;

#[derive(Eq, PartialEq, Hash, Clone)]
struct Coord(isize, isize, isize);

impl Coord {
    fn neighbours(&self) -> [Coord; 6] {
        [
            Coord(self.0 - 1, self.1, self.2), Coord(self.0 + 1, self.1, self.2),
            Coord(self.0, self.1 - 1, self.2), Coord(self.0, self.1 + 1, self.2),
            Coord(self.0, self.1, self.2 - 1), Coord(self.0, self.1, self.2 + 1),
        ]
    }
}

struct Cuboid(RangeInclusive<isize>, RangeInclusive<isize>, RangeInclusive<isize>);

impl Cuboid {
    fn min_coord(&self) -> Coord {
        Coord(*self.0.start(), *self.1.start(), *self.2.start())
    }

    fn contains(&self, coord: &Coord) -> bool {
        self.0.contains(&coord.0) && self.1.contains(&coord.1) && self.2.contains(&coord.2)
    }
}

// Find the bounding box of all given Coords, expanded in each direction by 1
fn expanded_bounding_box(coords: &HashSet<Coord>) -> Cuboid {
    Cuboid(
        (coords.iter().map(|c| c.0).min().unwrap() - 1)..=coords.iter().map(|c| c.0).max().unwrap() + 1,
        (coords.iter().map(|c| c.1).min().unwrap() - 1)..=coords.iter().map(|c| c.1).max().unwrap() + 1,
        (coords.iter().map(|c| c.2).min().unwrap() - 1)..=coords.iter().map(|c| c.2).max().unwrap() + 1,
    )
}

fn main() {
    let input = include_str!("../input.txt");
    let cubes: HashSet<Coord> = input.lines().map(|line: &str| {
        let parts = line.splitn(3, ',')
            .map(|part| part.parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        Coord(parts[0], parts[1], parts[2])
    }).collect();

    let surface_area = cubes.iter().map(|cube| {
        cube.neighbours().iter().filter(|n| !cubes.contains(n)).count()
    }).sum::<usize>();
    println!("Part 1: {surface_area}");

    let bounding_box = expanded_bounding_box(&cubes);
    let min_coord = bounding_box.min_coord();
    assert!(!cubes.contains(&min_coord), "The min coord of the cubes' bounding box should not contain a cube, by definition");

    let mut queue = VecDeque::from([min_coord.clone()]);
    let mut visited = HashSet::from([min_coord.clone()]);
    let mut exterior_surface_area = 0;
    while let Some(coord) = queue.pop_front() {
        for neighbour in coord.neighbours() {
            if !bounding_box.contains(&neighbour) || visited.contains(&neighbour) {
                continue;
            }
            if cubes.contains(&neighbour) {
                exterior_surface_area += 1;
            } else {
                queue.push_back(neighbour.clone());
                visited.insert(neighbour);
            }
        }
    }
    println!("Part 2: {}", exterior_surface_area);
}
