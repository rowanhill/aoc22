fn snafu_char_to_decimal(snafu: &u8) -> isize {
    match snafu {
        b'2' => 2,
        b'1' => 1,
        b'0' => 0,
        b'-' => -1,
        b'=' => -2,
        _ => panic!("Unexpected snafu digit {}", snafu),
    }
}

fn snafu_to_decimal(snafu: &str) -> isize {
    let mut result = 0isize;
    for (i, c) in snafu.as_bytes().iter().rev().enumerate() {
        result += (5 as usize).pow(i as u32) as isize * snafu_char_to_decimal(c);
    }
    result

}

fn decimal_to_snafu(decimal: isize) -> String {
    let mut result = String::from("2");
    let mut result_decimal = snafu_to_decimal(&result);
    while decimal > result_decimal {
        result.push('2');
        result_decimal = snafu_to_decimal(&result);
    }

    'outer: for n in 0..result.len() {
        for (cur, next) in [("2", "1"), ("1", "0"), ("0","-"), ("-", "=")] {
            if result_decimal > decimal && &result[n..=n] == cur {
                result.replace_range(n..=n, next);
                let new_decimal = snafu_to_decimal(&result);
                if new_decimal < decimal {
                    result.replace_range(n..=n, cur);
                    continue 'outer;
                }
                result_decimal = new_decimal;
            }
        }
    }

    result
}

fn main() {
    let input = include_str!("../input.txt");
    let part1_decimal = input.lines().map(|line| snafu_to_decimal(line)).sum::<isize>();
    assert_eq!(part1_decimal, snafu_to_decimal(&decimal_to_snafu(part1_decimal)));
    println!("Part 1: {}", decimal_to_snafu(part1_decimal));
}
