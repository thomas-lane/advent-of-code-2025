use advent_of_code_2025::input_to_string;

fn parse_rotation(code: &str) -> i32 {
    let sign = if code.chars().nth(0) == Some('L') { -1 } else { 1 }; // Assume L or R
    let digits = &code[1..];
    let value: i32 = digits.parse().expect("Unable to parse rotation value");
    // println!("Movement: {}", sign * value);
    sign * value
}

fn part1() {
    let input = input_to_string("day01.txt");
    let mut position = 50;

    let mut code = 0;

    for line in input.lines() {
        position += parse_rotation(line);
        position %= 100;

        if position == 0 {
            code += 1;
        }
    }

    println!("Code: {}", code);
}

fn part2() {
    // let input = input_to_string("day01.txt");
    // let mut position = 50;

    // let mut code = 0;

    // for line in input.lines() {
    //     let rotation = parse_rotation(line);
    //     let zero_laps = (rotation / 100).abs();
    //     position += rotation % 100;
    //     let zero_crosses = zero_laps + if position < 0 || position >= 100 || (position == 0 && (rotation % 100 != 0)) { 1 } else { 0 };
    //     println!("Position: {} | Post-mod: {} | Crosses: {}", position, ((position % 100) + 100) % 100, zero_crosses);
    //     position = ((position % 100) + 100) % 100;

    //     // if position == 0 {
    //     //     code += zero_crosses;
    //     // }
    //     code += zero_crosses;
    // }

    // println!("Code: {}", code);

    let input = input_to_string("day01.txt");
    let mut position = 50;

    let mut code = 0;

    for line in input.lines() {
        let rotation = parse_rotation(line);

        let zero_laps = (rotation / 100).abs();
        let remaining_rotation = rotation % 100;

        position += remaining_rotation;
        let mod_position = ((position % 100) + 100) % 100;

        // Turned past 0 or 99 and did not start at 0
        if position != mod_position && position != remaining_rotation {
            code += 1;
        }

        // Rotated to land on 0
        if position == 0 && remaining_rotation != 0 {
            code += 1;
        }

        position = mod_position;

        code += zero_laps;
    }

    println!("Code: {}", code);
}

fn main() {
    part1();
    part2();
}
