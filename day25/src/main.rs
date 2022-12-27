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
    let mut col_val = 1;
    for c in snafu.as_bytes().iter().rev() {
        result += col_val * snafu_char_to_decimal(c);
        col_val *= 5;
    }
    result

}

/*
Encoding in a standard base is a simple case of div and mod. SNAFU is a *balanced* quinary system,
however, where digits can represent a negative value in a column. This complicates things slightly,
because encoding a single value between 0 and 4 (inclusive) can involve two columns:
  0 => 00
  1 => 01
  2 => 02
  3 => 1=
  4 => 1-
This means that, as we encode each digit, we need to track whether or not we're "carrying" one from
encoding the digit in the column to the right.
 */
fn decimal_to_snafu(decimal: isize) -> String {
    let mut snafu_digits = vec![];

    let mut num = decimal;
    let mut carry = 0;

    while num > 0 || carry > 0 {
        let digit_plus_carry = num % 5 + carry;
        num = num / 5;
        let c = match digit_plus_carry % 5 {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '=',
            4 => '-',
            _ => unreachable!()
        };
        snafu_digits.push(c);
        carry = if digit_plus_carry > 2 { 1 } else { 0 };
    }

    snafu_digits.iter().rev().collect()
}

fn main() {
    let input = include_str!("../input.txt");
    let part1_decimal = input.lines().map(|line| snafu_to_decimal(line)).sum::<isize>();
    assert_eq!(part1_decimal, snafu_to_decimal(&decimal_to_snafu(part1_decimal)));
    println!("Part 1: {}", decimal_to_snafu(part1_decimal));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ints_to_snafu_and_back() {
        for i in 1..100 {
            let snafu = decimal_to_snafu(i);
            let decimal = snafu_to_decimal(&snafu);
            assert_eq!(i, decimal, "{} -> {} -> {}", i, snafu, decimal);
        }
    }
}
