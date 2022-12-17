use std::collections::HashMap;

type Coord = (usize, usize);

const BLOCKS_HORIZONTAL: [[bool; 4]; 1] = [
    [true , true , true , true ]
];
const BLOCKS_PLUS: [[bool; 4]; 3] = [
    [false, true , false, false],
    [true , true , true , false],
    [false, true , false, false],
];
const BLOCKS_L: [[bool; 4]; 3] = [
    [false, false, true , false],
    [false, false, true , false],
    [true , true , true , false],
];
const BLOCKS_VERTICAL: [[bool; 4]; 4] = [
    [true , false, false, false],
    [true , false, false, false],
    [true , false, false, false],
    [true , false, false, false],
];
const BLOCKS_SQUARE: [[bool; 4]; 2] = [
    [true , true , false, false],
    [true , true , false, false],
];

#[derive(Debug)]
enum RockShape {
    Horizontal,
    Plus,
    L,
    Vertical,
    Square
}

const ROCK_ORDER: [RockShape; 5] = [RockShape::Horizontal, RockShape::Plus, RockShape::L, RockShape::Vertical, RockShape::Square];

struct Chamber {
    rows: Vec<[bool; 7]>,
    jet_index: usize,
    heights: [usize; 7],
    num_elided_rows: usize,
}

impl Chamber {
    fn new() -> Chamber {
        Chamber { rows: vec![], jet_index: 0, heights: [0; 7], num_elided_rows: 0 }
    }

    fn calc_height_after_rounds(&mut self, jet_dirs: &[u8], rounds: usize) -> usize {
        for rock_index in 0..rounds {
            self.simulate_single_rock(rock_index, jet_dirs);
        }
        self.height()
    }

    fn calc_height_via_periodicity_after_rounds(&mut self, jet_dirs: &[u8], rounds: usize) -> usize {
        let mut rock_index = 0;
        let mut state_lookup = HashMap::new();
        let mut height_lookup = vec![];
        loop {
            // println!("Dropping rock {} onto tower of height {}: {:?}", rock_index, self.height(), self.heights);
            let state = (rock_index % ROCK_ORDER.len(), self.jet_index % jet_dirs.len(), self.heights.clone());
            if let Some(period_start) = state_lookup.insert(state, rock_index) {
                // Find how much height is added within every repeating period
                let height_at_first_period_start = height_lookup[period_start];
                let height_at_first_period_end = self.height();
                let height_per_period = height_at_first_period_end - height_at_first_period_start;

                // Find the number of fully complete repeating periods are needed, plus the number
                // of rounds needed within the final (incomplete) period
                let rounds_after_start_of_periods = rounds - period_start;
                let period_duration = rock_index - period_start;
                let num_full_periods = rounds_after_start_of_periods / period_duration;
                let rounds_in_partial_period = rounds_after_start_of_periods % period_duration;

                let height_added_by_full_periods = num_full_periods * height_per_period;
                // Find the combined height added before the repeating period is encountered plus
                // in the final partial period
                let height_added_outside_periods = height_lookup[period_start + rounds_in_partial_period];

                return height_added_by_full_periods + height_added_outside_periods;
            }

            height_lookup.push(self.height());
            let complete_row = self.simulate_single_rock(rock_index, jet_dirs);
            rock_index += 1;

            // If a row was completed, we can save memory by discarding that row and all beneath it
            if let Some(complete_row) = complete_row {
                self.rows.drain(0..=complete_row);
                self.heights = self.heights.map(|h| h - complete_row);
                self.num_elided_rows += complete_row + 1;
            }
        }
    }

    // Returns the highest row completed as a result of dropping this rock
    fn simulate_single_rock(&mut self, rock_index: usize, jet_dirs: &[u8]) -> Option<usize> {
        let mut coord = self.new_rock_coord();
        let shape = &ROCK_ORDER[rock_index % ROCK_ORDER.len()];

        loop {
            // Move horizontally
            let jet = &jet_dirs[self.jet_index % jet_dirs.len()];
            self.jet_index += 1;
            let maybe_new_coord = match jet {
                b'<' => self.try_left(shape, &coord),
                b'>' => self.try_right(shape, &coord),
                _ => panic!("Unexpected jet_dir byte {}", jet),
            };
            coord = maybe_new_coord.unwrap_or(coord);

            // Try and move down
            let maybe_new_coord = self.try_down(shape, &coord);
            if let Some(new_coord) = maybe_new_coord {
                coord = new_coord;
            } else {
                return self.settle(shape, &coord);
            }
        }
    }

    fn new_rock_coord(&self) -> Coord {
        (2, self.rows.len() + 3)
    }

    fn try_left(&self, shape: &RockShape, bottom_left: &Coord) -> Option<Coord> {
        if bottom_left.0 == 0 {
            return None;
        }

        let new_coord = (bottom_left.0 - 1, bottom_left.1);
        if self.would_collide(shape, &new_coord) {
            None
        } else {
            Some(new_coord)
        }
    }

    fn try_right(&self, shape: &RockShape, bottom_left: &Coord) -> Option<Coord> {
        let cur_right = bottom_left.0 + shape.width() - 1;
        if cur_right >= 6 {
            return None;
        }

        let new_coord = (bottom_left.0 + 1, bottom_left.1);
        if self.would_collide(shape, &new_coord) {
            None
        } else {
            Some(new_coord)
        }
    }

    fn try_down(&self, shape: &RockShape, bottom_left: &Coord) -> Option<Coord> {
        if bottom_left.1 <= 0 {
            return None;
        }

        let new_coord = (bottom_left.0, bottom_left.1 - 1);
        if self.would_collide(shape, &new_coord) {
            None
        } else {
            Some(new_coord)
        }
    }

    fn would_collide(&self, shape: &RockShape, bottom_left: &Coord) -> bool {
        let blocks = shape.blocks();
        let width = shape.width();

        blocks.iter().rev().enumerate().any(|(dy, row)| {
            let y = bottom_left.1 + dy;
            let within_tower = y < self.rows.len();
            within_tower && row.iter().enumerate().any(|(dx, block)| {
                let x = bottom_left.0 + dx;
                dx < width && *block && self.rows[y][x]
            })
        })
    }

    // Add the shape to the tower, and return the highest row completed as a result
    fn settle(&mut self, shape: &RockShape, bottom_left: &Coord) -> Option<usize> {
        let blocks = shape.blocks();
        let width = shape.width();

        let mut complete_row = None;
        for (dy, row) in blocks.iter().rev().enumerate() {
            let y = bottom_left.1 + dy;
            let within_tower = y < self.rows.len();
            if !within_tower {
                self.rows.push([false; 7]);
            }
            for (dx, block) in row.iter().enumerate() {
                let x = bottom_left.0 + dx;
                if dx < width && *block {
                    self.rows[y][x] = true;
                    if self.heights[x] < y {
                        self.heights[x] = y;
                    }
                }
            }
            if self.rows[y] == [true; 7] {
                complete_row = Some(y);
            }
        }
        complete_row
    }

    fn height(&self) -> usize {
        self.num_elided_rows + self.rows.len()
    }
}

impl RockShape {
    fn width(&self) -> usize {
        match self {
            RockShape::Horizontal => 4,
            RockShape::Plus => 3,
            RockShape::L => 3,
            RockShape::Vertical => 1,
            RockShape::Square => 2,
        }
    }

    fn blocks(&self) -> &[[bool; 4]] {
        use RockShape::*;
        match self {
            Horizontal => &BLOCKS_HORIZONTAL[..],
            Plus => &BLOCKS_PLUS[..],
            L => &BLOCKS_L[..],
            Vertical => &BLOCKS_VERTICAL[..],
            Square => &BLOCKS_SQUARE[..],
        }
    }
}

fn main() {
    let jet_dirs = include_bytes!("../input.txt");

    let mut chamber = Chamber::new();
    let height = chamber.calc_height_after_rounds(jet_dirs, 2022);
    println!("Part 1: {}", height);

    let mut chamber = Chamber::new();
    let height = chamber.calc_height_via_periodicity_after_rounds(jet_dirs, 1000000000000);
    println!("Part 2: {height}");
}