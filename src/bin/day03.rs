use advent_of_code_2025::input_to_string;

fn part1() {
    let input = input_to_string("day03.txt");

    let mut total_joltage = 0;
    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let mut left_pointer = 0;
        let mut right_pointer = line.len() - 1;

        for i in 1..right_pointer {
            if line.chars().nth(i) > line.chars().nth(left_pointer) {
                left_pointer = i;
            }
        }

        for i in (left_pointer + 1..right_pointer).rev() {
            if line.chars().nth(i) > line.chars().nth(right_pointer) {
                right_pointer = i;
            }
        }

        let combined = format!("{}{}", line.chars().nth(left_pointer).unwrap(), line.chars().nth(right_pointer).unwrap());
        let joltage: i32 = combined.parse().unwrap();
        total_joltage += joltage;
    }

    println!("Joltage: {}", total_joltage);
}

fn part2() {
    let input = input_to_string("day03.txt");

    let mut total_joltage = 0;
    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let mut pointers = Vec::<usize>::new();

        for pointer_index in 0..12 {
            let start_index = if pointer_index == 0 { 0 } else { pointers[pointer_index - 1] + 1 };

            let mut greatest_index = start_index;
            for i in start_index..line.len() + pointer_index - 11 {
                if line.chars().nth(i) > line.chars().nth(greatest_index) {
                    greatest_index = i;
                }
            }

            pointers.push(greatest_index);
        }

        let mut joltage = 0;

        for (i, pointer) in pointers.iter().enumerate() {
            let digit_value = 10_u64.pow(11 - i as u32) * line.chars().nth(*pointer).unwrap().to_digit(10).unwrap() as u64;
            joltage += digit_value;
        }

        total_joltage += joltage;
    }

    println!("Joltage: {}", total_joltage);
}

fn main() {
    part1();
    part2();
}
