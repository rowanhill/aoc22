use std::collections::{HashSet, VecDeque};

#[repr(u8)]
#[derive(Eq, PartialEq)]
enum Cell {
    Open,
    Wall,
    North,
    East,
    South,
    West,
}

struct Map {
    width: usize,
    height: usize,
    rows: Vec<Vec<Cell>>,
}

impl Map {
    fn parse(input: &str) -> Map {
        use Cell::*;
        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();
        let rows = input.lines().map(|line| {
            line.as_bytes().iter().map(|c| {
                match c {
                    b'.' => Open,
                    b'#' => Wall,
                    b'^' => North,
                    b'>' => East,
                    b'v' => South,
                    b'<' => West,
                    _ => panic!("Unexpected character {c}")
                }
            }).collect()
        }).collect::<Vec<_>>();
        Map { height, width, rows }
    }

    fn is_open(&self, x: usize, y: usize, t: usize) -> bool {
        use Cell::*;
        
        // Walls
        if self.rows[y][x] == Wall {
            return false;
        }

        // South: Look up for blizzards travelling south
        let y_at_zero = (y as isize - 1 - t as isize).rem_euclid(self.height as isize - 2) as usize + 1;
        if self.rows[y_at_zero][x] == South {
            return false;
        }

        // North
        let y_at_zero = (y as isize - 1 + t as isize).rem_euclid(self.height as isize - 2) as usize + 1;
        if self.rows[y_at_zero][x] == North {
            return false;
        }

        // East
        let x_at_zero = (x as isize - 1 - t as isize).rem_euclid(self.width as isize - 2) as usize + 1;
        if self.rows[y][x_at_zero] == East {
            return false;
        }

        // West
        let x_at_zero = (x as isize - 1 + t as isize).rem_euclid(self.width as isize - 2) as usize + 1;
        if self.rows[y][x_at_zero] == West {
            return false;
        }

        true
    }

    fn open_neighbours(&self, x: usize, y: usize, t: usize) -> Vec<(usize, usize)> {
        let mut result = vec![];

        // Up
        if y > 0 && self.is_open(x, y - 1, t) {
            result.push((x, y - 1));
        }

        // Down
        if y < self.height - 1 && self.is_open(x, y + 1, t) {
            result.push((x, y + 1));
        }

        // Left
        if x > 0 && self.is_open(x - 1, y, t) {
            result.push((x - 1, y));
        }

        // Right
        if x < self.width - 1 && self.is_open(x + 1, y, t) {
            result.push((x + 1, y));
        }

        // Wait
        if self.is_open(x, y, t) {
            result.push((x, y));
        }

        result
    }
}

fn find_time_at_reaching(start: (usize, usize), end: (usize, usize), start_time: usize, map: &Map) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((start.0, start.1, start_time));
    let mut visited = HashSet::new();
    visited.insert((start.0, start.1, start_time));

    while let Some((x, y, t)) = queue.pop_front() {
        for (nx, ny) in map.open_neighbours(x, y, t + 1) {
            if (nx, ny) == end {
                return t + 1;
            }
            if visited.contains(&(nx, ny, t + 1)) {
                continue;
            }
            queue.push_back((nx, ny, t + 1));
            visited.insert((nx, ny, t + 1));
        }
    }

    panic!("Could not find route between {:?} @ {} and {:?}", start, start_time, end)
}

fn main() {
    let input = include_str!("../../input.txt");
    let map = Map::parse(input);

    let part1 = find_time_at_reaching((1, 0), (map.width - 2, map.height - 2), 0, &map) + 1;
    println!("Part 1: {}", part1);

    let after_returning = find_time_at_reaching((map.width - 2, map.height - 1), (1, 1), part1, &map) + 1;
    let after_back_again = find_time_at_reaching((1, 0), (map.width - 2, map.height - 2), after_returning, &map) + 1;
    println!("Part 2: {}", after_back_again);
}

#[cfg(test)]
mod tests {
    use crate::{find_time_at_reaching, Map};

    #[test]
    fn east_blizzard() {
        let map = Map::parse("#.###\n#>..#\n#...#\n#...#\n###.#");

        assert_not_open(&map, &[(1, 1)], 0);
        assert_not_open(&map, &[(2, 1)], 1);
        assert_not_open(&map, &[(3, 1)], 2);
        assert_not_open(&map, &[(1, 1)], 3);
    }

    #[test]
    fn west_blizzard() {
        let map = Map::parse("#.###\n#<..#\n#...#\n#...#\n###.#");

        assert_not_open(&map, &[(1, 1)], 0);
        assert_not_open(&map, &[(3, 1)], 1);
        assert_not_open(&map, &[(2, 1)], 2);
        assert_not_open(&map, &[(1, 1)], 3);
    }

    #[test]
    fn north_blizzard() {
        let map = Map::parse("#.###\n#^..#\n#...#\n#...#\n###.#");

        assert_not_open(&map, &[(1, 1)], 0);
        assert_not_open(&map, &[(1, 3)], 1);
        assert_not_open(&map, &[(1, 2)], 2);
        assert_not_open(&map, &[(1, 1)], 3);
    }


    #[test]
    fn south_blizzard() {
        let map = Map::parse("#.###\n#v..#\n#...#\n#...#\n###.#");

        assert_not_open(&map, &[(1, 1)], 0);
        assert_not_open(&map, &[(1, 2)], 1);
        assert_not_open(&map, &[(1, 3)], 2);
        assert_not_open(&map, &[(1, 1)], 3);
    }

    #[test]
    fn converging_blizzards() {
        let map = Map::parse("#.###\n#.v.#\n#>.<#\n#.^.#\n###.#");

        assert_not_open(&map, &[(2, 1), (1, 2), (3, 2), (2, 3)], 0);
        assert_not_open(&map, &[(2, 2)], 1);
        assert_not_open(&map, &[(2, 1), (1, 2), (3, 2), (2, 3)], 2);
        assert_not_open(&map, &[(2, 1), (1, 2), (3, 2), (2, 3)], 3);
        assert_not_open(&map, &[(2, 2)], 4);
        assert_not_open(&map, &[(2, 1), (1, 2), (3, 2), (2, 3)], 5);
        assert_not_open(&map, &[(2, 1), (1, 2), (3, 2), (2, 3)], 6);
        assert_not_open(&map, &[(2, 2)], 7);
        assert_not_open(&map, &[(2, 1), (1, 2), (3, 2), (2, 3)], 8);
    }

    #[test]
    fn neighbours_corners() {
        let map = Map::parse("#####\n#...#\n#...#\n#...#\n#####");
        assert_eq!(map.open_neighbours(1, 1, 0), vec![(1, 2), (2, 1), (1, 1)]);
        assert_eq!(map.open_neighbours(3, 1, 0), vec![(3, 2), (2, 1), (3, 1)]);
        assert_eq!(map.open_neighbours(1, 3, 0), vec![(1, 2), (2, 3), (1, 3)]);
        assert_eq!(map.open_neighbours(3, 3, 0), vec![(3, 2), (2, 3), (3, 3)]);
    }

    #[test]
    fn neighbours_middle() {
        let map = Map::parse("#.###\n#...#\n#...#\n#...#\n###.#");
        assert_eq!(map.open_neighbours(2, 2, 0), vec![(2, 1), (2, 3), (1, 2), (3, 2), (2, 2)]);
    }

    #[test]
    fn neighbours_by_blizzards() {
        let map = Map::parse("#.###\n#.v.#\n#>.<#\n#.^.#\n###.#");
        assert_eq!(map.open_neighbours(2, 2, 0), vec![(2, 2)]);
        assert_eq!(map.open_neighbours(2, 2, 1), vec![(2, 1), (2, 3), (1, 2), (3, 2)]);
    }

    #[test]
    fn time_for_example_route() {
        let map = Map::parse("#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#");
        let expected = find_time_at_reaching((1, 0), (map.width - 2, map.height - 2), 0, &map) + 1;
        assert_eq!(expected, 18);
    }

    #[test]
    fn time_to_go_back_example() {
        let map = Map::parse("#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#");
        let expected = find_time_at_reaching((map.width - 2, map.height - 1), (1, 1), 18, &map) + 1;
        assert_eq!(expected, 18 + 23);
    }

    fn assert_not_open(map: &Map, not_opens: &[(usize, usize)], time: usize) {
        for x in 1..(map.width-1) {
            for y in 1..(map.height-1) {
                let expect_open = !not_opens.contains(&(x, y));
                assert_eq!(map.is_open(x, y, time), expect_open, "({:?}, {:?}) @ {} is_open should be {:?}", x, y, time, expect_open);
            }
        }
    }
}