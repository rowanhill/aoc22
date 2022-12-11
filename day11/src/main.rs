use std::collections::BinaryHeap;

#[derive(Clone)]
enum WorryOp {
    Add(u64),
    Times(u64),
    Square,
}

#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    op: WorryOp,
    test: (u64, usize, usize),
    inspections: u64,
}

fn calc_monkey_business(mut monkeys: Vec<Monkey>, rounds: usize, worry_reducer: impl Fn(u64) -> u64) -> u64 {
    let mut passed_items: Vec<(usize, u64)> = vec![];
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            for mut worry in monkey.items.iter().copied() {
                worry = match monkey.op {
                    WorryOp::Add(n) => worry + n,
                    WorryOp::Times(n) => worry *n,
                    WorryOp::Square => worry * worry
                };
                worry = worry_reducer(worry);
                let recipient = if worry % monkey.test.0 == 0 {
                    monkey.test.1
                } else {
                    monkey.test.2
                };
                passed_items.push((recipient, worry));
            }
            monkey.inspections += monkey.items.len() as u64;
            monkey.items.clear();

            for (j, worry) in &passed_items {
                monkeys[*j].items.push(*worry);
            }
            passed_items.clear();
        }
    }

    let inspections: BinaryHeap<u64> = monkeys.iter().map(|m| m.inspections).collect();
    inspections.iter().take(2).product()
}

fn main() {
    let input = include_str!("../input.txt");

    let mut lines = input.lines();
    let mut monkeys = vec![];
    while lines.next().is_some() {
        let items = lines.next().unwrap()[18..].split(", ").map(|n| n.parse().unwrap()).collect();
        let op_line = lines.next().unwrap();
        let operand = &op_line[25..];
        let op = if operand == "old" {
            WorryOp::Square
        } else {
            let op_num = operand.parse().unwrap();
            match &op_line[23..=23] {
                "+" => WorryOp::Add(op_num),
                "*" => WorryOp::Times(op_num),
                _ => unreachable!(),
            }
        };
        let test_div = lines.next().unwrap()[21..].parse().unwrap();
        let if_true = lines.next().unwrap()[29..].parse().unwrap();
        let if_false = lines.next().unwrap()[30..].parse().unwrap();
        let test = (test_div, if_true, if_false);
        monkeys.push(Monkey { items, op, test, inspections: 0 });
        lines.next();
    }

    let monkey_business_1 = calc_monkey_business(monkeys.clone(), 20, |w| w / 3);
    println!("Part 1: {}", monkey_business_1);

    let test_product: u64 = monkeys.iter().map(|m| m.test.0).product();
    let monkey_business_2 = calc_monkey_business(monkeys, 10_000, |w| w % test_product);
    println!("Part 2: {}", monkey_business_2);
}