const WIDTH: usize = 40;
const HEIGHT: usize = 6;

struct CPU {
    x: i32,
    cycle: i32,
    signal_strength: i32,
    screen: [[bool; WIDTH]; HEIGHT],
}

impl CPU {
    fn new() -> CPU {
        CPU {
            x: 1,
            cycle: 0,
            signal_strength: 0,
            screen: [[false; WIDTH]; HEIGHT],
        }
    }

    fn draw_pixel(&mut self) {
        let y = (self.cycle / WIDTH as i32) as usize;
        let x = (self.cycle % WIDTH as i32) as usize;
        self.screen[y][x] = self.x.abs_diff(x as i32) <= 1;
    }

    fn print_screen(&self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                print!("{}", if self.screen[y][x] { "#" } else { "." });
            }
            println!();
        }
    }

    fn tick(&mut self) {
        self.draw_pixel();
        self.cycle += 1;

        if self.cycle % 40 == 20 {
            self.signal_strength += self.x * self.cycle;
        }
    }

    fn noop(&mut self) {
        self.tick();
    }

    fn addx(&mut self, v: i32) {
        self.tick();
        self.tick();
        self.x += v;
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let mut cpu = CPU::new();
    for line in input.lines() {
        if line == "noop" {
            cpu.noop();
        } else {
            cpu.addx(*&line[5..].parse().unwrap())
        }
    }

    println!("Part 1: {}", cpu.signal_strength);

    println!("Part 2:");
    cpu.print_screen();
}
