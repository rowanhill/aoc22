use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("Could not read input file");

    let mut total_misplaced_priority: u32 = 0;
    let mut total_badge_priority = 0;

    let mut lines = input.lines();
    while let (Some(line1), Some(line2), Some(line3)) = (lines.next(), lines.next(), lines.next()) {
        let mut badge_counts = [0u8; 52];

        for line in [line1, line2, line3] {
            let mid = line.len() / 2;
            let rhs = &line[mid..];

            let mut line_includes_item = [false; 52];
            let mut found_misplaced = false;
            for (index, c) in line.chars().enumerate() {
                let priority = if c.is_lowercase() {
                    c as u32 - b'a' as u32 + 1
                } else {
                    c as u32 - b'A' as u32 + 1 + 26
                };
                line_includes_item[(priority - 1) as usize] = true;

                if !found_misplaced && index < mid {
                    if rhs.contains(c) {
                        total_misplaced_priority += priority;
                        found_misplaced = true;
                    }
                }
            }

            for (index, included) in line_includes_item.into_iter().enumerate() {
                badge_counts[index] += included as u8;
            }
        }

        let badge_index = badge_counts.into_iter()
            .position(|count| count == 3u8)
            .expect("No item in all 3 lines");
        total_badge_priority += badge_index + 1;
    }

    println!("Part 1: {}", total_misplaced_priority);
    println!("Part 2: {}", total_badge_priority);
}
