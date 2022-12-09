use std::collections::HashSet;

type Coord = (i64, i64);

fn main() {
    let input = include_str!("../index.txt");

    let mut knots: [Coord; 10] = [(0, 0); 10];
    
    let mut visited_1 = HashSet::new();
    let mut visited_9 = HashSet::new();
    visited_1.insert(knots[1]);
    visited_9.insert(knots[9]);

    for line in input.lines() {
        let (dir, num_steps) = line.split_once(" ").unwrap();
        let num_steps: u32 = num_steps.parse().unwrap();
        let delta = match dir {
            "L" => (-1, 0),
            "R" => (1, 0),
            "U" => (0, -1),
            "D" => (0, 1),
            _ => unreachable!()
        };

        for _ in 0..num_steps {
            knots[0] = (knots[0].0 + delta.0, knots[0].1 + delta.1);

            for i in 1..10 {
                let dx = knots[i - 1].0 - knots[i].0;
                let dy = knots[i - 1].1 - knots[i].1;

                if dx.abs() > 1 || dy.abs() > 1 {
                    knots[i].0 += dx.signum();
                    knots[i].1 += dy.signum();
                }
            }

            visited_1.insert(knots[1]);
            visited_9.insert(knots[9]);
        }
    }

    println!("Part 1: {}", visited_1.len());
    println!("Part 2: {}", visited_9.len());
}
