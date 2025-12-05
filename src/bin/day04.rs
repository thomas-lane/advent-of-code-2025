use advent_of_code_2025::input_to_string;

fn check_cell(map: &String, width: isize, index: usize) -> bool {
    let mut count = 0;
    let offset_width = width + 1; // Account for newlines
    let offsets = vec![-offset_width - 1, -offset_width, -offset_width + 1, -1, 1, offset_width - 1, offset_width, offset_width + 1];

    for offset in offsets {
        let offset_index = (index as isize + offset) as usize;
        if offset_index >= map.len() {
            continue;
        }

        let cell = map.as_bytes()[(index as isize + offset) as usize];
        if cell == b'@' {
            count += 1;
        }
    }
    
    count < 4
}

fn part1() {
    let input = input_to_string("day04.txt");
    
    let mut rolls_of_paper = 0;
    let width = input.lines().nth(0).unwrap().len() as isize;

    for (i, c) in input.chars().enumerate() {
        if c == '@' && check_cell(&input, width, i) {
            rolls_of_paper += 1;
        }
    }

    println!("Rolls of paper: {}", rolls_of_paper);
}

fn part2() {
    let mut input = input_to_string("day04.txt");
    
    let mut rolls_of_paper = 0;
    let width = input.lines().nth(0).unwrap().len() as isize;

    let mut remove_indices = Vec::new();

    loop {
        let mut converged = true;
        for (i, c) in input.chars().enumerate() {
            if c == '@' && check_cell(&input, width, i) {
                rolls_of_paper += 1;
                remove_indices.push(i);
                converged = false;
            }
        }
    
        for i in &remove_indices {
            input.replace_range(*i..*i + 1, ".");
        }

        if converged {
            break;
        }
    }

    println!("Rolls of paper: {}", rolls_of_paper);
}

fn main() {
    part1();
    part2();
}
