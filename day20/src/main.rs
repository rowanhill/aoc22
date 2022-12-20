fn main() {
    let input = include_str!("../input.txt");
    let numbers = input.lines().map(|line| line.parse::<isize>().unwrap()).collect::<Vec<_>>();

    let mut tracked_numbers = numbers.iter().map(|n| (*n, false)).collect::<Vec<(isize, bool)>>();

    let mut i = 0;
    for _ in 0..tracked_numbers.len() {
        while &tracked_numbers[i].1 == &true {
            i += 1;
        }
        let tmp = tracked_numbers.remove(i);
        let new_pos = (i as isize + tmp.0).rem_euclid(tracked_numbers.len() as isize) as usize;
        tracked_numbers.insert(new_pos, (tmp.0, true));
    }

    let zero_pos = tracked_numbers.iter().position(|(n,_)| n == &0).unwrap();
    let one = (zero_pos + 1000) % tracked_numbers.len();
    let two = (zero_pos + 2000) % tracked_numbers.len();
    let three = (zero_pos + 3000) % tracked_numbers.len();
    let coord_sum = &tracked_numbers[one].0 + &tracked_numbers[two].0 + &tracked_numbers[three].0;
    println!("Part 1: {}", coord_sum);

    let mut tracked_numbers = numbers.iter().enumerate().map(|(i, n)| (*n * 811589153, i)).collect::<Vec<(isize, usize)>>();

    for _rounds in 0..10 {
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

    let zero_pos = tracked_numbers.iter().position(|(n,_)| n == &0).unwrap();
    let one = (zero_pos + 1000) % tracked_numbers.len();
    let two = (zero_pos + 2000) % tracked_numbers.len();
    let three = (zero_pos + 3000) % tracked_numbers.len();
    let coord_sum = &tracked_numbers[one].0 + &tracked_numbers[two].0 + &tracked_numbers[three].0;
    println!("Part 2: {}", coord_sum);
}
