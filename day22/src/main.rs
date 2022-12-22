use std::collections::HashMap;
use std::iter::repeat;

type Coord = (usize, usize);
type WrapMap = HashMap<Coord, MapCell>;

#[derive(Clone)]
struct MapCell {
    north: Option<(Coord, Rotation)>,
    east: Option<(Coord, Rotation)>,
    south: Option<(Coord, Rotation)>,
    west: Option<(Coord, Rotation)>,
}

impl MapCell {
    fn step(&self, direction: &Direction) -> &Option<(Coord, Rotation)> {
        use Direction::*;
        match direction {
            North => &self.north,
            East => &self.east,
            South => &self.south,
            West => &self.west,
        }
    }

    fn update_dir(&self, direction: &Direction, next: Option<(Coord, Rotation)>) -> MapCell {
        use Direction::*;
        match direction {
            North => MapCell{ north: next, ..self.clone() },
            East => MapCell{ east: next, ..self.clone() },
            South => MapCell{ south: next, ..self.clone() },
            West => MapCell{ west: next, ..self.clone() },
        }
    }
}

#[derive(Clone, Debug)]
enum Direction { North, East, South, West }

impl Direction {
    fn turn_right(&self) -> Direction {
        use Direction::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    fn turn_left(&self) -> Direction {
        use Direction::*;
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    fn password_score(&self) -> usize {
        use Direction::*;
        match self {
            East => 0,
            South => 1,
            West => 2,
            North => 3,
        }
    }
}

#[derive(Clone)]
struct Agent {
    position: Coord,
    direction: Direction,
}

impl Agent {
    fn advance(&mut self, steps: usize, map: &WrapMap) {
        for _ in 0..steps {
            let cur_cell = map.get(&self.position).unwrap();
            let next_pos = cur_cell.step(&self.direction);
            if let Some((pos, rot)) = next_pos {
                self.position = pos.clone();
                self.turn(rot);
            } else {
                break;
            }
        }
    }

    fn turn(&mut self, rotation: &Rotation) {
        match rotation {
            Rotation::Right => {
                self.direction = self.direction.turn_right();
            }
            Rotation::Left => {
                self.direction = self.direction.turn_left();
            }
            Rotation::Half => {
                self.direction = self.direction.turn_left().turn_left();
            }
            Rotation::Nothing => {}
        }
    }

    fn password_score(&self) -> usize {
        1000 * (self.position.1 + 1) +
            4 * (self.position.0 + 1) +
            self.direction.password_score()
    }
}

#[derive(Clone, Debug)]
enum Rotation { Right, Left, Half, Nothing }
#[derive(Debug)]
enum Instruction {
    Advance(usize),
    Turn(Rotation),
}

fn parse_instructions(line: &str) -> Vec<Instruction> {
    let mut start = 0;
    let mut end = 0;
    let bytes = line.as_bytes();
    let mut instrs = vec![];
    for i in 0..bytes.len() {
        let c = bytes[i];
        match c {
            b'L' | b'R' => {
                if end > start {
                    let n = (&line[start..end]).parse::<usize>().unwrap();
                    instrs.push(Instruction::Advance(n));
                }
                match c {
                    b'L' => {
                        instrs.push(Instruction::Turn(Rotation::Left));
                    },
                    b'R' => {
                        instrs.push(Instruction::Turn(Rotation::Right));
                    },
                    _ => unreachable!(),
                }
                start = i + 1;
                end = i + 1;
            },
            _ => {
                end += 1;
            },
        }
    }
    if end > start {
        let n = (&line[start..end]).parse::<usize>().unwrap();
        instrs.push(Instruction::Advance(n));
    }
    instrs
}

fn parse_input(input: &str) -> (Agent, WrapMap, Vec<Instruction>) {
    let mut map = HashMap::new();
    let mut agent = Agent { position: (0, 0), direction: Direction::East };

    let mut map_chars = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (y, line) in input.lines().take_while(|l| !l.is_empty()).enumerate() {
        for (x, c) in line.as_bytes().iter().enumerate() {
            map_chars.insert((x, y), *c);
            width = width.max(x + 1);
            height = height.max(y + 1);
        }
    }

    let mut has_found_agent_start = false;
    for y in 0..height {
        for x in 0..width {
            let c = map_chars.get(&(x, y));
            if let Some(b'.') = c {
                if y == 0 && !has_found_agent_start {
                    agent.position = (x, y);
                    has_found_agent_start = true;
                }
                let west: Option<(Coord, Rotation)> = {
                    let mut next_x = (x + width - 1) % width;
                    let mut result = None;
                    loop {
                        let next_coord = (next_x, y);
                        let c = map_chars.get(&next_coord);
                        if let Some(b'.') = c {
                            result = Some((next_coord, Rotation::Nothing));
                            break;
                        } else if let Some(b'#') = c {
                            break;
                        }
                        next_x = (next_x + width - 1) % width;
                    }
                    result
                };
                let east: Option<(Coord, Rotation)> = {
                    let mut next_x = (x + width + 1) % width;
                    let mut result = None;
                    loop {
                        let next_coord = (next_x, y);
                        let c = map_chars.get(&next_coord);
                        if let Some(b'.') = c {
                            result = Some((next_coord, Rotation::Nothing));
                            break;
                        } else if let Some(b'#') = c {
                            break;
                        }
                        next_x = (next_x + width + 1) % width;
                    }
                    result
                };
                let north: Option<(Coord, Rotation)> = {
                    let mut next_y = (y + height - 1) % height;
                    let mut result = None;
                    loop {
                        let next_coord = (x, next_y);
                        let c = map_chars.get(&next_coord);
                        if let Some(b'.') = c {
                            result = Some((next_coord, Rotation::Nothing));
                            break;
                        } else if let Some(b'#') = c {
                            break;
                        }
                        next_y = (next_y + height - 1) % height;
                    }
                    result
                };
                let south: Option<(Coord, Rotation)> = {
                    let mut next_y = (y + height + 1) % height;
                    let mut result = None;
                    loop {
                        let next_coord = (x, next_y);
                        let c = map_chars.get(&next_coord);
                        if let Some(b'.') = c {
                            result = Some((next_coord, Rotation::Nothing));
                            break;
                        } else if let Some(b'#') = c {
                            break;
                        }
                        next_y = (next_y + height + 1) % height;
                    }
                    result
                };
                let cell = MapCell { north, east, south, west };
                map.insert((x, y), cell);
            }
        }
    }
    let instructions = parse_instructions(input.lines().last().unwrap());
    (agent, map, instructions)
}

/*
  1122
  33
4455
66

1N -> 6R  2N -> 6N  3N -> 1N  4N -> 3R  5N -> 3N  6N -> 4N
1E -> 2N  2E -> 5H  3E -> 2L  4E -> 5N  5E -> 2F  6E -> 5L
1S -> 3N  2S -> 3R  3S -> 5N  4S -> 6N  5S -> 6R  6S -> 2N
1W -> 4H  2W -> 1N  3W -> 4L  4W -> 1H  5W -> 4N  6W -> 1L
 */
fn convert_to_cube(map: &mut WrapMap) {
    use Direction::*;
    use Rotation::*;

    // Top of 1 <-> left of 6

    // Top of 1 goes to left of 6, rotating right (l->r : t->b) (50,0)..(100,0) -> (0,150)..(0,200)
    update_line(
        map,
        &mut (50..100).into_iter(),
        &mut repeat(0).take(50),
        &mut repeat(0).take(50),
        &mut (150..200).into_iter(),
        North,
        Right,
    );
    // Left of 6 goes to top of 1 left turn
    update_line(
        map,
        &mut repeat(0).take(50),
        &mut (150..200).into_iter(),
        &mut (50..100).into_iter(),
        &mut repeat(0).take(50),
        West,
        Left,
    );


    // Top of 2 <-> bottom of 6

    // Top of 2 goes to bottom of 6, no rotation (100,0)..(150,0) -> (0,199)..(50,199)
    update_line(
        map,
        &mut (100..150).into_iter(),
        &mut repeat(0).take(50),
        &mut (0..50).into_iter(),
        &mut repeat(199).take(50),
        North,
        Nothing,
    );
    // Bottom of 6 goes to top of 2, no rotation
    update_line(
        map,
        &mut (0..50).into_iter(),
        &mut repeat(199).take(50),
        &mut (100..150).into_iter(),
        &mut repeat(0).take(50),
        South,
        Nothing,
    );


    // Right of 2 <-> right of 5

    // Right of 2 goes to right of 5, 180 rotation (t->b : b->t)  (149,0)..(149,50) -> (99,150)..(99,100)
    update_line(
        map,
        &mut repeat(149).take(50),
        &mut (0..50).into_iter(),
        &mut repeat(99).take(50),
        &mut (100..150).rev().into_iter(),
        East,
        Half,
    );
    // Right of 5 goes to right of 2, 180 turn (t->b : b->t) (99,100)..(99,150) to (149,49)..=(149,0)
    update_line(
        map,
        &mut repeat(99).take(50),
        &mut (100..150).into_iter(),
        &mut repeat(149).take(50),
        &mut (0..50).rev().into_iter(),
        East,
        Half,
    );


    // Bottom of 2 <-> right of 3

    // Bottom of 2 goes to right of 3, rotating right (l->r : t->b)  (100,49)..(150,49) -> (99,50)..(99,100)
    update_line(
        map,
        &mut (100..150).into_iter(),
        &mut repeat(49).take(50),
        &mut repeat(99).take(50),
        &mut (50..100).into_iter(),
        South,
        Right,
    );
    // Right of 3 goes to bottom of 2, rotating left (t->b : l->r)  (99,50)..(99,100) -> (100,49)..(150,49)
    update_line(
        map,
        &mut repeat(99).take(50),
        &mut (50..100).into_iter(),
        &mut (100..150).into_iter(),
        &mut repeat(49).take(50),
        East,
        Left,
    );


    // Bottom of 5 <-> right of 6

    // Bottom of 5 goes to right of 6, right turn (l->r : t->b)  (50,149)..(100,149) to (49,150)..(49,200)
    update_line(
        map,
        &mut (50..100).into_iter(),
        &mut repeat(149).take(50),
        &mut repeat(49).take(50),
        &mut (150..200).into_iter(),
        South,
        Right,
    );
    // Right of 6 goes to bottom of 5, left turn  (49,150)..(49,200) to (50,149)..(100,149)
    update_line(
        map,
        &mut repeat(49).take(50),
        &mut (150..200).into_iter(),
        &mut (50..100).into_iter(),
        &mut repeat(149).take(50),
        East,
        Left,
    );


    // Left of 4 <-> left of 1

    // Left of 4 goes to left of 1, 180 turn, (0,100)..(0,150) to (50,50)..(50,0)
    update_line(
        map,
        &mut repeat(0).take(50),
        &mut (100..150).into_iter(),
        &mut repeat(50).take(50),
        &mut (0..50).rev().into_iter(),
        West,
        Half,
    );
    // Left of 1 goes to left of 4
    update_line(
        map,
        &mut repeat(50).take(50),
        &mut (0..50).into_iter(),
        &mut repeat(0).take(50),
        &mut (100..150).rev().into_iter(),
        West,
        Half,
    );


    // Top of 4 <-> left of 3

    // Top of 4 goes to left of 3, right turn, (0,100)..(50,100) to (50,50)..(50,100)
    update_line(
        map,
        &mut (0..50).into_iter(),
        &mut repeat(100).take(50),
        &mut repeat(50).take(50),
        &mut (50..100).into_iter(),
        North,
        Right,
    );
    // Left of 3 goes to top of 4
    update_line(
        map,
        &mut repeat(50).take(50),
        &mut (50..100).into_iter(),
        &mut (0..50).into_iter(),
        &mut repeat(100).take(50),
        West,
        Left,
    );
}

fn update_line(
    map: &mut WrapMap,
    from_xs: &mut impl Iterator<Item = usize>,
    from_ys: &mut impl Iterator<Item = usize>,
    to_xs: &mut impl Iterator<Item = usize>,
    to_ys: &mut impl Iterator<Item = usize>,
    from_direction: Direction,
    rotation: Rotation,
) {
    for _ in 0..50 {
        let from_x = from_xs.next().unwrap(); let from_y = from_ys.next().unwrap();
        let to_x = to_xs.next().unwrap(); let to_y = to_ys.next().unwrap();

        let from_coord = (from_x, from_y);
        if let Some(old) = map.get(&from_coord) {
            let to_coord = (to_x, to_y);
            let next = match map.get(&to_coord) {
                Some(_) => Some((to_coord, rotation.clone())),
                None => None
            };
            map.insert(from_coord, old.update_dir(&from_direction, next));
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let (mut agent, map, instructions) = parse_input(input);
    let mut agent2 = agent.clone();
    
    for instr in &instructions {
        match instr {
            Instruction::Advance(n) => agent.advance(*n, &map),
            Instruction::Turn(rot) => agent.turn(rot),
        }
    }
    println!("Part 1: {}", agent.password_score());

    let mut cube_map = map.clone();
    convert_to_cube(&mut cube_map);
    for instr in &instructions {
        match instr {
            Instruction::Advance(n) => agent2.advance(*n, &cube_map),
            Instruction::Turn(rot) => agent2.turn(rot),
        }
    }
    println!("Part 2: {}", agent2.password_score());
}
