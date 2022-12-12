use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, Clone)]
struct GridCell {
    coord: (usize, usize),
    height: u8,
    dist: i32,
}

impl PartialEq<Self> for GridCell {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}
impl Ord for GridCell {
    fn cmp(&self, other: &Self) -> Ordering {
        // Order GridCell by greatest dist first
        other.dist.cmp(&self.dist)
    }
}
impl PartialOrd<Self> for GridCell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn calculate_shortest_paths(grid: &mut Vec<Vec<GridCell>>, start: &(usize, usize)) {
    let mut queue = BinaryHeap::new();

    grid[start.1][start.0].dist = 0;
    queue.push(grid[start.1][start.0].clone());

    while let Some(cell) = queue.pop() {
        for (dx, dy) in [(-1i32, 0i32), (0, -1), (1, 0), (0, 1)] {
            let newx = cell.coord.0 as i32 + dx;
            let newy = cell.coord.1 as i32 + dy;

            if newx >= 0 && (newx as usize) < grid[0].len() && newy >= 0 && (newy as usize) < grid.len() {
                let neighbour = &mut grid[newy as usize][newx as usize];
                if neighbour.height + 1 >= cell.height {
                    if neighbour.dist == -1 || cell.dist + 1 < neighbour.dist {
                        neighbour.dist = cell.dist + 1;
                        queue.push(neighbour.clone());
                    }
                }
            }
        }
    }
}

fn main() {
    let inst = std::time::Instant::now();
    let input = include_bytes!("../input.txt");
    let mut grid: Vec<Vec<GridCell>> = vec![];
    let mut row = vec![];
    let mut y = 0;
    let mut x = 0;
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut poss_starts = vec![];
    for &b in input {
        if b == b'\n' {
            grid.push(row);
            row = vec![];
            y += 1;
            x = 0;
            continue;
        }
        let height = match b {
            b'S' => 0,
            b'E' => 25,
            _ => b - b'a',
        };
        row.push(GridCell { height, dist: -1, coord: (x, y) });

        if b == b'S' {
            start = (x, y);
        } else if b == b'E' {
            end = (x, y);
        } else if b == b'a' {
            poss_starts.push((x, y));
        }

        x += 1;
    }
    grid.push(row);

    // Find all shortest paths, starting from the end point and running backwards
    calculate_shortest_paths(&mut grid, &end);
    
    let end_cell = &grid[start.1][start.0];
    let part1 = end_cell.dist;
    println!("Part 1: {}", part1);

    let part2 = poss_starts.into_iter()
        .map(|(x, y)| grid[y][x].dist)
        .filter(|d| d > &0)
        .min().unwrap();
    println!("Part 2: {}", part2);

    println!("{:?}", inst.elapsed());
}
