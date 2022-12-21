use std::collections::HashMap;

enum Node {
    Number(isize),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

fn parse(input: &str) -> HashMap<String, Node> {
    use Node::*;
    let mut lookup = HashMap::new();
    for line in input.lines() {
        let name = line[0..4].to_string();
        let expression = &line[6..];
        if let Ok(number) = expression.parse() {
            lookup.insert(name, Number(number));
        } else {
            let lhs = expression[0..4].to_string();
            let op = &expression[5..=5];
            let rhs = expression[7..].to_string();
            let node = match op {
                "+" => Add(lhs, rhs),
                "-" => Sub(lhs, rhs),
                "*" => Mul(lhs, rhs),
                "/" => Div(lhs, rhs),
                _ => unreachable!(),
            };
            lookup.insert(name, node);
        }
    }
    lookup
}

fn calculate(name: String, lookup: &HashMap<String, Node>) -> isize {
    use Node::*;
    match lookup.get(&name) {
        Some(Number(n)) => *n,
        Some(Add(l, r)) => calculate(l.clone(), lookup) + calculate(r.clone(), lookup),
        Some(Sub(l, r)) => calculate(l.clone(), lookup) - calculate(r.clone(), lookup),
        Some(Mul(l, r)) => calculate(l.clone(), lookup) * calculate(r.clone(), lookup),
        Some(Div(l, r)) => calculate(l.clone(), lookup) / calculate(r.clone(), lookup),
        _ => unreachable!(),
    }
}

fn get_root_operands(lookup: &HashMap<String, Node>) -> (isize, isize) {
    use Node::*;
    match &lookup[&"root".to_string()] {
        Add(l, r) | Sub(l, r) | Mul(l, r) | Div(l, r) => (calculate(l.clone(), lookup), calculate(r.clone(), lookup)),
        Number(_) => panic!("Root is a number"),
    }
}

// This function assumes humn only affects root's lhs, and the relationship between humn and lhs is
// monotonic. This is true for my input, but not true for any general input you could devise.
fn binary_search_for_humn_value(mut lookup: HashMap<String, Node>) -> isize {
    let humn = "humn".to_string();
    // First, keep doubling until we find an upper bound;
    let mut lower = if let Node::Number(n) = lookup[&humn] {
        n
    } else {
        panic!("humn is not a number");
    };
    let mut upper = lower * 2;
    loop {
        lookup.insert(humn.clone(), Node::Number(upper));
        let (l, r) = get_root_operands(&lookup);
        if l > r {
            lower = upper;
            upper *= 2;
        } else {
            break;
        }
    }
    // Then, binary search within the bounds until we find a humn number that makes root's inputs ==
    loop {
        let test = (lower + upper) / 2;
        lookup.insert(humn.clone(), Node::Number(test));
        let (l, r) = get_root_operands(&lookup);
        if l == r {
            return test;
        }
        if lower == upper {
            panic!("Binary search failed: bounds have collapsed, but no answer found");
        }
        if l > r {
            lower = test;
        } else if l < r {
            upper = test;
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let lookup = parse(input);
    let part1 = calculate("root".to_string(), &lookup);
    println!("Part 1: {}", part1);

    let part2 = binary_search_for_humn_value(lookup);
    println!("Part 2: {}", part2);
}