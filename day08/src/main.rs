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

// NOTE: Original solution didn't include this function, but instead basically copied the body 4 times,
// once for each direction
fn scan_grid<const N: usize>(
    grid: &mut Grid<N>,
    outer: &(impl Iterator<Item = usize> + Clone),
    inner: &(impl Iterator<Item = usize> + Clone),
    is_row: bool,
    dir_index: usize,
) {
    for i in outer.clone() {
        let mut tallest = -1i8;
        let mut dists_to_not_smaller = [0; 10];
        for j in inner.clone() {
            // Are we iterating over a row or a column?
            let (x, y) = if is_row { (j, i) } else { (i, j) };

            // Check if the tree is visible from the edge in this direction
            if grid[y][x].0 > tallest {
                grid[y][x].1 = true;
                tallest = grid[y][x].0;
            }

            // Record the distance to the nearest tree that is >= the height of this tree in this direction
            grid[y][x].2[dir_index] = dists_to_not_smaller[(grid[y][x].0 as usize)];

            // Distance for trees this height or smaller reset to 1 (this is the closest tree >= their height)
            for h in 0..=grid[y][x].0 {
                dists_to_not_smaller[h as usize] = 1;
            }
            // Distance for taller trees increments by one (this is another tree they can see over)
            for h in (grid[y][x].0+1)..=9 {
                dists_to_not_smaller[h as usize] += 1;
            }
        }
    }
}

fn main() {
    // const GRID_SIZE: usize = 5;
    // let mut grid = parse_grid::<GRID_SIZE>("example.txt");
    const GRID_SIZE: usize = 99;
    let mut grid = parse_grid::<GRID_SIZE>("input.txt");

    let range = 0..GRID_SIZE;
    let revrs = range.clone().rev();
    scan_grid::<GRID_SIZE>(&mut grid, &range, &range, true, 0);
    scan_grid::<GRID_SIZE>(&mut grid, &range, &revrs, true, 1);
    scan_grid::<GRID_SIZE>(&mut grid, &revrs, &range, false, 2);
    scan_grid::<GRID_SIZE>(&mut grid, &revrs, &revrs, false, 3);

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
