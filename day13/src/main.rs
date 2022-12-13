#![feature(array_chunks)]

use std::cmp::Ordering;
use std::iter::Peekable;

#[derive(Clone, Eq, PartialEq)]
enum PacketNode {
    Int(u8),
    List(Vec<PacketNode>)
}

fn parse_packet(iter: &mut Peekable<impl Iterator<Item = u8>>) -> PacketNode {
    match iter.peek() {
        Some(b'[') => {
            iter.next();
            parse_packet_list(iter)
        },
        Some(_) => parse_packet_int(iter),
        None => unreachable!(),
    }
}

fn parse_packet_list(iter: &mut Peekable<impl Iterator<Item = u8>>) -> PacketNode {
    use PacketNode::*;
    let mut items = vec![];
    loop {
        match iter.peek() {
            Some(b',') | Some(b' ') => { iter.next(); },
            Some(b']') => break,
            Some(_) => { items.push(parse_packet(iter)); },
            None => unreachable!(),
        }
    }
    iter.next();
    List(items)
}

fn parse_packet_int(iter: &mut Peekable<impl Iterator<Item = u8>>) -> PacketNode {
    use PacketNode::*;
    let mut num = 0;
    loop {
        let b = iter.next().unwrap();
        num = num * 10 + (b - b'0');
        match iter.peek() {
            Some(b) if b >= &b'0' && b <= &b'9' => continue,
            _ => break,
        }
    }
    Int(num)
}

impl PartialOrd<Self> for PacketNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketNode {
    fn cmp(&self, other: &Self) -> Ordering {
        use PacketNode::*;
        match (self, other) {
            (Int(l), Int(r)) => l.cmp(r),
            (List(ls), List(rs)) => ls.cmp(rs),
            (List(_), Int(_)) => self.cmp(&List(vec![other.clone()])),
            (Int(_), List(_)) => List(vec![self.clone()]).cmp(other),
        }
    }
}

fn parse_packets(bytes: &[u8]) -> Vec<PacketNode> {
    let mut result = vec![];
    let mut iter = bytes.iter().cloned().peekable();
    loop {
        match iter.peek() {
            Some(b'\n') => { iter.next(); },
            Some(_) => { result.push(parse_packet(&mut iter)); },
            None => break,
        }
    }
    result
}

fn main() {
    let inst = std::time::Instant::now();
    let input = include_bytes!("../input.txt");
    let mut packets = parse_packets(input);

    let part1 = packets.array_chunks()
        .enumerate()
        .filter(|(_, [left, right])| left < right)
        .map(|(i, _)| i + 1)
        .sum::<usize>();
    println!("Part 1: {}", part1);

    let div1 = PacketNode::List(vec![PacketNode::List(vec![PacketNode::Int(2)])]);
    let div2 = PacketNode::List(vec![PacketNode::List(vec![PacketNode::Int(6)])]);
    packets.push(div1.clone());
    packets.push(div2.clone());
    packets.sort();

    let pos1 = packets.iter().position(|p| p == &div1).unwrap() + 1;
    let pos2 = packets.iter().position(|p| p == &div2).unwrap() + 1;
    println!("Part 2: {}", pos1 * pos2);

    println!("{:?}", inst.elapsed());
}
