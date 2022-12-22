use std::collections::HashMap;

type Coord = (usize, usize);
type WrapMap = HashMap<Coord, MapCell>;

struct MapCell {
    north: Option<Coord>,
    east: Option<Coord>,
    south: Option<Coord>,
    west: Option<Coord>,
}

impl MapCell {
    fn step(&self, direction: &Direction) -> &Option<Coord> {
        use Direction::*;
        match direction {
            North => &self.north,
            East => &self.east,
            South => &self.south,
            West => &self.west,
        }
    }
}

#[derive(Debug)]
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

struct Agent {
    position: Coord,
    direction: Direction,
}

impl Agent {
    fn advance(&mut self, steps: usize, map: &WrapMap) {
        for _ in 0..steps {
            let cur_cell = map.get(&self.position).unwrap();
            let next_pos = cur_cell.step(&self.direction);
            if let Some(pos) = next_pos {
                self.position = pos.clone();
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
        }
    }

    fn password_score(&self) -> usize {
        1000 * (self.position.1 + 1) +
            4 * (self.position.0 + 1) +
            self.direction.password_score()
    }
}

#[derive(Debug)]
enum Rotation { Right, Left }
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
                let west: Option<Coord> = {
                    let mut next_x = (x + width - 1) % width;
                    let mut result = None;
                    loop {
                        let next_coord = (next_x, y);
                        let c = map_chars.get(&next_coord);
                        if let Some(b'.') = c {
                            result = Some(next_coord);
                            break;
                        } else if let Some(b'#') = c {
                            break;
                        }
                        next_x = (next_x + width - 1) % width;
                    }
                    result
                };
                let east: Option<Coord> = {
                    let mut next_x = (x + width + 1) % width;
                    let mut result = None;
                    loop {
                        let next_coord = (next_x, y);
                        let c = map_chars.get(&next_coord);
                        if let Some(b'.') = c {
                            result = Some(next_coord);
                            break;
                        } else if let Some(b'#') = c {
                            break;
                        }
                        next_x = (next_x + width + 1) % width;
                    }
                    result
                };
                let north: Option<Coord> = {
                    let mut next_y = (y + height - 1) % height;
                    let mut result = None;
                    loop {
                        let next_coord = (x, next_y);
                        let c = map_chars.get(&next_coord);
                        if let Some(b'.') = c {
                            result = Some(next_coord);
                            break;
                        } else if let Some(b'#') = c {
                            break;
                        }
                        next_y = (next_y + height - 1) % height;
                    }
                    result
                };
                let south: Option<Coord> = {
                    let mut next_y = (y + height + 1) % height;
                    let mut result = None;
                    loop {
                        let next_coord = (x, next_y);
                        let c = map_chars.get(&next_coord);
                        if let Some(b'.') = c {
                            result = Some(next_coord);
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

fn main() {
    let input = include_str!("../input.txt");
    let (mut agent, map, instructions) = parse_input(input);

    for instr in &instructions {
        match instr {
            Instruction::Advance(n) => agent.advance(*n, &map),
            Instruction::Turn(rot) => agent.turn(rot),
        }
    }
    println!("Part 1: {}", agent.password_score());
}
