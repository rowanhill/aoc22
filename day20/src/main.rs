fn mix(tracked_numbers: &mut Vec<(isize, usize)>) {
    let mut i = 0;
    for to_move in 0..tracked_numbers.len() {
        while &tracked_numbers[i].1 != &to_move {
            i = (i + 1) % tracked_numbers.len();
        }
        let tmp = tracked_numbers.remove(i);
        let new_pos = (i as isize + tmp.0).rem_euclid(tracked_numbers.len() as isize) as usize;
        tracked_numbers.insert(new_pos, tmp);
    }
}

fn sum_coordinates(tracked_numbers: &Vec<(isize, usize)>) -> isize {
    let zero_pos = tracked_numbers.iter().position(|(n,_)| n == &0).unwrap();
    let one = (zero_pos + 1000) % tracked_numbers.len();
    let two = (zero_pos + 2000) % tracked_numbers.len();
    let three = (zero_pos + 3000) % tracked_numbers.len();
     &tracked_numbers[one].0 + &tracked_numbers[two].0 + &tracked_numbers[three].0
}

fn main() {
    let input = include_str!("../input.txt");
    let tracked_numbers = input.lines()
        .enumerate()
        .map(|(i, line)| (line.parse::<isize>().unwrap(), i))
        .collect::<Vec<_>>();

    let mut part1_nums = tracked_numbers.clone();
    mix(&mut part1_nums);
    let part1_sum = sum_coordinates(&part1_nums);
    println!("Part 1: {}", part1_sum);

    let key = 811589153;
    let mut part2_nums = tracked_numbers.into_iter().map(|(n, i)| (n * key, i)).collect::<Vec<_>>();
    for _ in 0..10 {
        mix(&mut part2_nums);
    }
    let part2_sum = sum_coordinates(&part2_nums);
    println!("Part 2: {}", part2_sum);
}
