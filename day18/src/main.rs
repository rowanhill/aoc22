use std::collections::{HashSet, VecDeque};

type Coord = (isize, isize, isize);

fn main() {
    let input = include_str!("../input.txt");
    let cubes: HashSet<Coord> = input.lines().map(|line: &str| {
        let parts = line.splitn(3, ',')
            .map(|part| part.parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        (parts[0], parts[1], parts[2])
    }).collect();

    let mut surface_area = 0;
    for cube in &cubes {
        for (dx, dy, dz) in [(-1,0,0),(1,0,0), (0,-1,0),(0,1,0), (0,0,-1),(0,0,1)] {
            let neighbour = (cube.0 + dx, cube.1 + dy, cube.2 + dz);
            if !cubes.contains(&neighbour) {
                surface_area += 1;
            }
        }
    }
    println!("Part 1: {surface_area}");

    let min_x = cubes.iter().map(|c| c.0).min().unwrap() - 1;
    let max_x = cubes.iter().map(|c| c.0).max().unwrap() + 1;
    let min_y = cubes.iter().map(|c| c.1).min().unwrap() - 1;
    let max_y = cubes.iter().map(|c| c.1).max().unwrap() + 1;
    let min_z = cubes.iter().map(|c| c.2).min().unwrap() - 1;
    let max_z = cubes.iter().map(|c| c.2).max().unwrap() + 1;

    let x_range = min_x..=max_x;
    let y_range = min_y..=max_y;
    let z_range = min_z..=max_z;

    let min_coord: Coord = (min_x, min_y, min_z);
    assert!(!cubes.contains(&min_coord));

    let mut queue = VecDeque::from([min_coord.clone()]);
    let mut visited = HashSet::new();
    let mut exterior_surface = 0;
    while let Some(coord) = queue.pop_front() {
        for (dx, dy, dz) in [(-1,0,0),(1,0,0), (0,-1,0),(0,1,0), (0,0,-1),(0,0,1)] {
            let neighbour = (coord.0 + dx, coord.1 + dy, coord.2 + dz);
            if !x_range.contains(&neighbour.0) ||
                !y_range.contains(&neighbour.1) ||
                !z_range.contains(&neighbour.2)
            {
                continue;
            }
            if visited.contains(&neighbour) {
                continue;
            }
            if cubes.contains(&neighbour) {
                exterior_surface += 1;
            } else {
                queue.push_back(neighbour.clone());
                visited.insert(neighbour);
            }
        }
    }
    println!("Part 2: {}", exterior_surface);
}
