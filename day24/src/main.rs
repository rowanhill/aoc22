use std::collections::{HashSet, VecDeque};

#[repr(u8)]
#[derive(Clone)]
enum Direction {
    North,
    East,
    South,
    West
}

#[derive(Clone)]
struct Blizzards(Vec<Vec<Vec<Direction>>>);

impl Blizzards {
    fn new(width: usize, height: usize) -> Blizzards {
        Blizzards(vec![vec![vec![]; width]; height])
    }

    fn next(&self) -> Blizzards {
        let height = self.height();
        let width = self.width();
        let mut next_blizzards = Self::new(width, height);

        for x in 0..width {
            for y in 0..height {
                for b in &self.0[y][x] {
                    let (new_x, new_y) = match b {
                        Direction::North => if y == 1 { (x, height - 2) } else { (x, y - 1) },
                        Direction::South => if y == height - 2 { (x, 1) } else { (x, y + 1) },
                        Direction::West => if x == 1 { (width - 2, y) } else { (x - 1, y) },
                        Direction::East => if x == width - 2 { (1, y) } else { (x + 1, y) },
                    };
                    next_blizzards.0[new_y][new_x].push(b.clone());
                }
            }
        }

        next_blizzards
    }

    fn is_open(&self, x: usize, y: usize) -> bool {
        self.0[y][x].is_empty()
    }

    #[allow(dead_code)]
    fn draw(&self, walls: &HashSet<(usize, usize)>) {
        let height = self.height();
        let width = self.width();
        for y in 0..height {
            for x in 0..width {
                if walls.contains(&(x, y)) {
                    print!("#");
                    continue;
                }
                let bs = &self.0[y][x];
                if bs.len() == 0 {
                    print!(".");
                } else if bs.len() == 1 {
                    match &bs[0] {
                        Direction::North => print!("^"),
                        Direction::East => print!(">"),
                        Direction::South => print!("v"),
                        Direction::West => print!("<"),
                    }
                } else {
                    print!("{}", bs.len());
                }
            }
            println!();
        }
    }

    fn height(&self) -> usize {
        self.0.len()
    }
    fn width(&self) -> usize {
        self.0[0].len()
    }
}

fn time_to_move_between(
    start_coord: &(usize, usize),
    end_coord: &(usize, usize),
    all_blizzards: &Vec<Blizzards>,
    walls: &HashSet<(usize, usize)>,
    width: usize,
    height: usize,
    start_min: usize,
) -> usize {
    let move_deltas = [(0, -1), (0, 1), (-1, 0), (1, 0), (0, 0)];

    // Queue stores elf coord at end of given minute
    let mut queue = VecDeque::new();
    queue.push_back((start_coord.clone(), start_min));
    let mut visited = HashSet::new();
    while let Some((coord, minute)) = queue.pop_front() {
        let next_blizzards = & all_blizzards[minute % (width * height)];
        for delta in &move_deltas {
            let new_x = coord.0 as isize + delta.0;
            let new_y = coord.1 as isize + delta.1;
            if new_y < 0 || new_y >= height as isize {
                // Can't walk out of the map
                continue;
            }
            let next_coord = (new_x as usize, new_y as usize);
            if walls.contains(&next_coord) {
                // Can't walk into walls
                continue;
            }
            if visited.contains(&(next_coord, minute + 1)) {
                continue;
            }
            visited.insert((next_coord.clone(), minute + 1));
            if next_blizzards.is_open(next_coord.0, next_coord.1) {
                if &next_coord == end_coord {
                    return minute;
                } else {
                    queue.push_back((next_coord, minute + 1));
                }
            }
        }
    }
    panic!("Could not find route");
}

fn main() {
    use Direction::*;
    let input = include_str!("../input.txt");
    let mut entrance_coord = (0, 0);
    let mut found_start = false;
    let mut exit_coord = (0, 0);
    let mut found_exit = false;
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let mut initial_blizzards = Blizzards::new(width, height);
    let mut walls = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.as_bytes().iter().enumerate() {
            match c {
                b'.' => {
                    if y == 0 && !found_start {
                        found_start = true;
                        entrance_coord = (x, y);
                    } else if y == height - 1 && !found_exit {
                        found_exit = true;
                        exit_coord = (x, y);
                    }
                },
                b'#' => { walls.insert((x, y)); },
                b'^' => initial_blizzards.0[y][x].push(North),
                b'>' => initial_blizzards.0[y][x].push(East),
                b'v' => initial_blizzards.0[y][x].push(South),
                b'<' => initial_blizzards.0[y][x].push(West),
                _ => panic!("Unexpected input character {}", c),
            }
        }
    }

    // Precalculate blizzard states
    // initial_blizzards.draw(&walls);
    let mut all_blizzards = Vec::with_capacity(width * height);
    all_blizzards.push(initial_blizzards);
    for i in 1..(width * height) {
        let new_blizzards = all_blizzards[i-1].next();
        // new_blizzards.draw(&walls);
        all_blizzards.push(new_blizzards);
    }
    println!("Calculated all blizzards");

    let part1 = time_to_move_between(
        &entrance_coord,
        &exit_coord,
        &all_blizzards,
        &walls,
        width,
        height,
        0,
    );
    println!("Part 1: {}", part1);

    let go_back = time_to_move_between(
        &exit_coord,
        &entrance_coord,
        &all_blizzards,
        &walls,
        width,
        height,
        part1,
    );
    let and_exit_again = time_to_move_between(
        &entrance_coord,
        &exit_coord,
        &all_blizzards,
        &walls,
        width,
        height,
        go_back,
    );
    println!("Part 2: {}", and_exit_again);
}
