use std::fs::read_to_string;

// (height, is_visible_from_edge, [left, right, down, up])
type GridCell = (i8, bool, [u32; 4]);
type Grid<const N: usize> = [[GridCell; N]; N];

fn parse_grid<const N: usize>(path: &str) -> Grid<N> {
    let input = read_to_string(path).expect("Could not read input file");
    let mut grid = [[(0, false, [0; 4]); N]; N];
    for (row_index, line) in input.lines().enumerate() {
        for (col_index, c) in line.chars().enumerate() {
            grid[row_index][col_index].0 = c.to_digit(10).unwrap() as i8;
        }
    }
    grid
}

fn main() {
    // const GRID_SIZE: usize = 5;
    // let mut grid = parse_grid::<GRID_SIZE>("example.txt");
    const GRID_SIZE: usize = 99;
    let mut grid = parse_grid::<GRID_SIZE>("input.txt");

    for y in 0..GRID_SIZE {
        // Left to right
        let mut tallest = -1i8;
        let mut height_counts = [0; 10];
        for x in 0..GRID_SIZE {
            if grid[y][x].0 > tallest {
                grid[y][x].1 = true;
                tallest = grid[y][x].0;
            }
            grid[y][x].2[0] = height_counts[(grid[y][x].0 as usize)];
            for h in 0..=grid[y][x].0 {
                height_counts[h as usize] = 1;
            }
            for h in (grid[y][x].0+1)..=9 {
                height_counts[h as usize] += 1;
            }
        }

        // Right to left
        tallest = -1;
        height_counts = [0; 10];
        for x in (0..GRID_SIZE).rev() {
            if grid[y][x].0 > tallest {
                grid[y][x].1 = true;
                tallest = grid[y][x].0;
            }
            grid[y][x].2[1] = height_counts[(grid[y][x].0 as usize)];
            for h in 0..=grid[y][x].0 {
                height_counts[h as usize] = 1;
            }
            for h in (grid[y][x].0+1)..=9 {
                height_counts[h as usize] += 1;
            }
        }
    }
    for x in 0..GRID_SIZE {
        // Top to bottom
        let mut tallest = -1;
        let mut height_counts = [0; 10];
        for y in 0..GRID_SIZE {
            if grid[y][x].0 > tallest {
                grid[y][x].1 = true;
                tallest = grid[y][x].0;
            }
            grid[y][x].2[2] = height_counts[(grid[y][x].0 as usize)];
            for h in 0..=grid[y][x].0 {
                height_counts[h as usize] = 1;
            }
            for h in (grid[y][x].0+1)..=9 {
                height_counts[h as usize] += 1;
            }
        }

        // Bottom to top
        tallest = -1;
        height_counts = [0; 10];
        for y in (0..GRID_SIZE).rev() {
            if grid[y][x].0 > tallest {
                grid[y][x].1 = true;
                tallest = grid[y][x].0;
            }
            grid[y][x].2[3] = height_counts[(grid[y][x].0 as usize)];
            for h in 0..=grid[y][x].0 {
                height_counts[h as usize] = 1;
            }
            for h in (grid[y][x].0+1)..=9 {
                height_counts[h as usize] += 1;
            }
        }
    }

    let num_visible = grid.iter()
        .flat_map(|row| row.iter().map(|(_, v, _)| v))
        .filter(|&&v| v)
        .count();
    println!("Part 1: {}", num_visible);

    let max_scenic_score = grid.iter()
        .flat_map(|row| row.iter().map(|(_, _, hs)| hs[0] * hs[1] * hs[2] * hs[3]))
        .max()
        .unwrap();
    println!("Part 2: {}", max_scenic_score);
}
