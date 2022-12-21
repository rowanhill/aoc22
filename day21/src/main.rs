use std::collections::{HashMap, HashSet};

enum Node {
    Number(isize),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

impl Node {
    fn has_dependent(&self, name: &String) -> bool {
        match self {
            Node::Number(_) => false,
            Node::Add(l, r) | Node::Sub(l, r) | Node::Mul(l, r) | Node::Div(l, r) => l == name || r == name,
        }
    }
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

fn calculate(name: &String, lookup: &HashMap<String, Node>) -> isize {
    use Node::*;
    match lookup.get(name) {
        Some(Number(n)) => *n,
        Some(Add(l, r)) => calculate(l, lookup) + calculate(r, lookup),
        Some(Sub(l, r)) => calculate(l, lookup) - calculate(r, lookup),
        Some(Mul(l, r)) => calculate(l, lookup) * calculate(r, lookup),
        Some(Div(l, r)) => calculate(l, lookup) / calculate(r, lookup),
        _ => unreachable!(),
    }
}

fn find_humn_dependents(lookup: &HashMap<String, Node>) -> HashSet<String> {
    let mut dependents = HashSet::new();
    let mut cur = "humn".to_string();
    while cur != "root".to_string() {
        dependents.insert(cur.clone());
        let mut nexts = lookup.iter()
            .filter(|(_, node)| node.has_dependent(&cur))
            .map(|(name, _)| name)
            .collect::<Vec<_>>();
        if nexts.len() != 1 {
            panic!("Expected exactly 1 dependent, found {}", nexts.len());
        }
        cur = nexts.pop().unwrap().clone();
    }
    dependents
}

fn invert_root(lookup: &HashMap<String, Node>, humn_dependents: &HashSet<String>) -> isize {
    use Node::*;
    let (l, r) = match &lookup[&"root".to_string()] {
        Add(l, r) | Sub(l, r) | Mul(l, r) | Div(l, r) => (l, r),
        Number(_) => panic!("Root is a number"),
    };
    match (humn_dependents.contains(l), humn_dependents.contains(r)) {
        (true, false) => {
            invert_to_humn(calculate(r, lookup), &l, lookup, humn_dependents)
        },
        (false, true) => {
            invert_to_humn(calculate(l, lookup), &r, lookup, humn_dependents)
        },
        (true, true) => panic!("Both operands of root depend on humn"),
        (false, false) => panic!("Neither operand of root depends on humn"),
    }
}

fn invert_to_humn(result: isize, name: &String, lookup: &HashMap<String, Node>, humn_dependents: &HashSet<String>) -> isize {
    use Node::*;
    if name == &"humn".to_string() {
        return result;
    }
    match &lookup[name] {
        Number(n) => *n,
        Add(l, r) => {
            match (humn_dependents.contains(l), humn_dependents.contains(r)) {
                (true, false) => invert_to_humn(result - calculate(r, lookup), l, lookup, humn_dependents),
                (false, true) => invert_to_humn(result - calculate(l, lookup), r, lookup, humn_dependents),
                (true, true) => panic!("Both operands depend on humn"),
                (false, false) => panic!("Neither operand depends on humn"),
            }
        }
        Sub(l, r) => {
            match (humn_dependents.contains(l), humn_dependents.contains(r)) {
                (true, false) => invert_to_humn(result + calculate(r, lookup), l, lookup, humn_dependents),
                (false, true) => invert_to_humn(calculate(l, lookup) - result, r, lookup, humn_dependents),
                (true, true) => panic!("Both operands depend on humn"),
                (false, false) => panic!("Neither operand depends on humn"),
            }
        }
        Div(l, r) => {
            match (humn_dependents.contains(l), humn_dependents.contains(r)) {
                (true, false) => invert_to_humn(result * calculate(r, lookup), l, lookup, humn_dependents),
                (false, true) => invert_to_humn(calculate(l, lookup) / result, r, lookup, humn_dependents),
                (true, true) => panic!("Both operands depend on humn"),
                (false, false) => panic!("Neither operand depends on humn"),
            }
        }
        Mul(l, r) => {
            match (humn_dependents.contains(l), humn_dependents.contains(r)) {
                (true, false) => invert_to_humn(result / calculate(r, lookup), l, lookup, humn_dependents),
                (false, true) => invert_to_humn(result / calculate(l, lookup), r, lookup, humn_dependents),
                (true, true) => panic!("Both operands depend on humn"),
                (false, false) => panic!("Neither operand depends on humn"),
            }
        }
    }
}

fn get_root_operands(lookup: &HashMap<String, Node>) -> (isize, isize) {
    use Node::*;
    match &lookup[&"root".to_string()] {
        Add(l, r) | Sub(l, r) | Mul(l, r) | Div(l, r) => (calculate(l, lookup), calculate(r, lookup)),
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
    let part1 = calculate(&"root".to_string(), &lookup);
    println!("Part 1: {}", part1);

    let dependents = find_humn_dependents(&lookup);
    let part2 = invert_root(&lookup, &dependents);
    println!("Part 2 (via solving the equation for humn): {}", part2);

    let part2 = binary_search_for_humn_value(lookup);
    println!("Part 2 (via binary search): {}", part2);
}