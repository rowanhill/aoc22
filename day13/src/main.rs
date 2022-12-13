use std::cmp::Ordering;

#[derive(Clone, Eq)]
enum PacketNode {
    Int(u8),
    List(Vec<PacketNode>)
}

fn parse_line(line: &str) -> PacketNode {
    let mut stack = vec![];
    let mut iter = line.as_bytes().iter().peekable();
    while let Some(b) = iter.next() {
        match b {
            b'[' => stack.push(vec![]),
            b']' => {
                let node = PacketNode::List(stack.pop().unwrap());
                if !stack.is_empty() {
                    stack.last_mut().unwrap().push(node);
                } else if iter.peek().is_none() {
                    return node;
                } else {
                    panic!("Tried to pop empty stack");
                }
            },
            b'0'..=b'9' => {
                let mut num = b - b'0';
                loop {
                    match iter.peek() {
                        None => break,
                        Some(&peeked) => match peeked {
                            b'0'..=b'9' => {
                                num *= 10;
                                let next = iter.next().unwrap();
                                num += next;
                            },
                            _ => break
                        }
                    }
                }
                stack.last_mut().unwrap().push(PacketNode::Int(num));
            },
            b' ' | b',' => continue,
            _ => unreachable!(),
        }
    }
    for b in line.as_bytes() {
        match b {
            b'[' => stack.push(vec![]),
            b']' => {
                let node = PacketNode::List(stack.pop().unwrap());
                stack.last_mut().unwrap().push(node);
            },
            b'0'..=b'9' => stack.last_mut().unwrap().push(PacketNode::Int(b - b'0')),
            b' ' | b',' => continue,
            _ => unreachable!(),
        }
    }
    PacketNode::List(stack.pop().unwrap())
}

impl PartialEq<Self> for PacketNode {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl PartialOrd<Self> for PacketNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketNode {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (PacketNode::Int(l), PacketNode::Int(r)) => l.cmp(r),
            (PacketNode::List(ls), PacketNode::List(rs)) => {
                for (child_l, child_r) in ls.iter().zip(rs.iter()) {
                    let child_ordering = child_l.cmp(child_r);
                    if child_ordering != Ordering::Equal {
                        return child_ordering;
                    }
                }
                ls.len().cmp(&rs.len())
            }
            (PacketNode::List(_), PacketNode::Int(_)) => self.cmp(&PacketNode::List(vec![other.clone()])),
            (PacketNode::Int(_), PacketNode::List(_)) => PacketNode::List(vec![self.clone()]).cmp(other),
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let pairs: Vec<_> = input.split("\n\n")
        .map(|s| {
            let (left, right) = s.split_once("\n").unwrap();
            (parse_line(left), parse_line(right))
        })
        .collect();

    let part1 = pairs.iter().enumerate()
        .filter_map(|(i, (left, right))| {
            if left < right {
                Some(i+1)
            } else {
                None
            }
        })
        .sum::<usize>();
    println!("Part 1: {}", part1);

    let mut packets: Vec<_> = pairs.iter().flat_map(|(left, right)| vec![left.clone(), right.clone()]).collect();

    let div1 = PacketNode::List(vec![PacketNode::List(vec![PacketNode::Int(2)])]);
    let div2 = PacketNode::List(vec![PacketNode::List(vec![PacketNode::Int(6)])]);
    packets.push(div1.clone());
    packets.push(div2.clone());
    packets.sort();

    let pos1 = packets.iter().position(|p| p.cmp(&div1) == Ordering::Equal).unwrap() + 1;
    let pos2 = packets.iter().position(|p| p.cmp(&div2) == Ordering::Equal).unwrap() + 1;
    println!("Part 2: {}", pos1 * pos2);
}
