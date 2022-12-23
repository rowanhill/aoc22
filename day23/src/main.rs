use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

type Coord = (isize, isize);

fn main() {
    let inst = std::time::Instant::now();
    let input = include_str!("../input.txt");

    let mut directions = [
        ((-1, -1), (0, -1), (1, -1)), // N
        ((-1,  1), (0,  1), (1,  1)), // S
        ((-1, -1), (-1, 0), (-1, 1)), // W
        (( 1, -1), (1,  0), (1,  1)), // E
    ].into_iter().cycle();

    let mut elves: HashSet<Coord> = input.lines().enumerate()
        .flat_map(|(y, row)| {
            row.as_bytes().into_iter().enumerate()
                .filter(|(_, c)| c == &&b'#')
                .map(move |(x, _)| (x as isize, y as isize))
        })
        .collect();

    let mut round = 1;
    loop {
        // Determine the order of directions to check this round
        let round_dirs = directions.clone().take(4).collect::<Vec<_>>();

        // Find where each elf proposes to move (if anywhere)
        let mut proposals: HashMap<Coord, Vec<Coord>> = HashMap::new();
        for elf in &elves {
            // Check if this elf has any neighbours - if so, it should consider moving
            let mut should_move = false;
            'outer: for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    if elves.contains(&(elf.0+dx, elf.1+dy)) {
                        should_move = true;
                        break 'outer;
                    }
                }
            }
            if !should_move {
                continue;
            }

            // Find the first direction (if any) this elf would like to propose to move
            for (delta_a, delta_b, delta_c) in &round_dirs {
                let a = (elf.0 + delta_a.0, elf.1 + delta_a.1);
                let b = (elf.0 + delta_b.0, elf.1 + delta_b.1);
                let c = (elf.0 + delta_c.0, elf.1 + delta_c.1);
                if !elves.contains(&a) && !elves.contains(&b) && !elves.contains(&c) {
                    proposals.entry(b.clone()).or_insert_with(|| vec![]).push(elf.clone());
                    break;
                }
            }
        }

        // If no elf proposed a move, we're done
        if proposals.is_empty() {
            println!("Part 2: {}", round);
            break;
        }

        // Find only the places proposed by a single elf and move them
        for (proposed_coord, elf_coords) in proposals.into_iter().filter(|(_, elf_coords)| elf_coords.len() == 1) {
            elves.remove(&elf_coords[0]);
            elves.insert(proposed_coord);
        }

        // Adjust the directions checked
        directions.next();
        
        if round == 10 {
            let empty_spaces = num_free_spaces(&elves);
            println!("Part 1: {}", empty_spaces);
        }

        round += 1;

        // draw(&elves);
        // println!();
    }
    println!("{:?}", inst.elapsed());
}

fn bounds(elves: &HashSet<Coord>) -> (RangeInclusive<isize>, RangeInclusive<isize>) {
    let mut min_x = 100_000; let mut max_x = -100_000;
    let mut min_y = 100_000; let mut max_y = -100_000;
    for elf in elves {
        min_x = elf.0.min(min_x);
        min_y = elf.1.min(min_y);
        max_x = elf.0.max(max_x);
        max_y = elf.1.max(max_y);
    }
    (min_x..=max_x, min_y..=max_y)
}

fn num_free_spaces(elves: &HashSet<Coord>) -> usize {
    let (xrange, yrange) = bounds(elves);
    let width = (xrange.end() - xrange.start() + 1) as usize;
    let height = (yrange.end() - yrange.start() + 1) as usize;
    width * height - elves.len()
}

#[allow(dead_code)]
fn draw(elves: &HashSet<Coord>) {
    let (xrange, yrange) = bounds(elves);
    for y in yrange {
        for x in xrange.clone() {
            if elves.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}