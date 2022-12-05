use std::fs::read_to_string;

struct ProcStep {
    num: usize,
    from: usize,
    to: usize,
}

impl ProcStep {
    fn parse(s: &str) -> ProcStep {
        let mut words = s.splitn(6, " ");
        let num = words.nth(1).unwrap().parse().unwrap();
        let from = words.nth(1).unwrap().parse().unwrap();
        let to = words.nth(1).unwrap().parse().unwrap();
        ProcStep { num, from, to }
    }
}

/*
            [C]         [N] [R]
[J] [T]     [H]         [P] [L]
[F] [S] [T] [B]         [M] [D]
[C] [L] [J] [Z] [S]     [L] [B]
[N] [Q] [G] [J] [J]     [F] [F] [R]
[D] [V] [B] [L] [B] [Q] [D] [M] [T]
[B] [Z] [Z] [T] [V] [S] [V] [S] [D]
[W] [P] [P] [D] [G] [P] [B] [P] [V]
 1   2   3   4   5   6   7   8   9
 */
fn main() {
    let input = read_to_string("input.txt").expect("Could not read input file");
    let proc_steps: Vec<ProcStep> = input.lines().map(|line| ProcStep::parse(line)).collect();

    let mut stacks = [
        vec!['W', 'B', 'D', 'N', 'C', 'F', 'J'],
        vec!['P', 'Z', 'V', 'Q', 'L', 'S', 'T'],
        vec!['P', 'Z', 'B', 'G', 'J', 'T'],
        vec!['D', 'T', 'L', 'J', 'Z', 'B', 'H', 'C'],
        vec!['G', 'V', 'B', 'J', 'S'],
        vec!['P', 'S', 'Q'],
        vec!['B', 'V', 'D', 'F', 'L', 'M', 'P', 'N'],
        vec!['P', 'S', 'M', 'F', 'B', 'D', 'L', 'R'],
        vec!['V', 'D', 'T', 'R']
    ];
    for step in &proc_steps {
        for _ in 0..step.num {
            let crate_char = stacks[step.from - 1].pop().unwrap();
            stacks[step.to - 1].push(crate_char);
        }
    }
    let tops_of_stacks: String = stacks.iter().map(|stack| stack.last().unwrap()).collect();
    println!("Part 1: {}", tops_of_stacks);

    let mut stacks = [
        vec!['W', 'B', 'D', 'N', 'C', 'F', 'J'],
        vec!['P', 'Z', 'V', 'Q', 'L', 'S', 'T'],
        vec!['P', 'Z', 'B', 'G', 'J', 'T'],
        vec!['D', 'T', 'L', 'J', 'Z', 'B', 'H', 'C'],
        vec!['G', 'V', 'B', 'J', 'S'],
        vec!['P', 'S', 'Q'],
        vec!['B', 'V', 'D', 'F', 'L', 'M', 'P', 'N'],
        vec!['P', 'S', 'M', 'F', 'B', 'D', 'L', 'R'],
        vec!['V', 'D', 'T', 'R']
    ];
    for step in &proc_steps {
        let mut tmp = vec![];
        for _ in 0..step.num {
            tmp.push(stacks[step.from - 1].pop().unwrap());
        }
        for _ in 0..step.num {
            stacks[step.to - 1].push(tmp.pop().unwrap());
        }
    }
    let tops_of_stacks: String = stacks.iter().map(|stack| stack.last().unwrap()).collect();
    println!("Part 2: {}", tops_of_stacks);
}
