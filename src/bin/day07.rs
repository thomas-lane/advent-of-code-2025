use advent_of_code_2025::{input_to_string, BitArray};

fn part1() {
    let input = input_to_string("day07.txt");

    let width = input.lines().nth(0).unwrap().len();
    let mut active_beams = BitArray::new(width);
    let mut activate_next_iteration: Vec<usize> = Vec::new();

    let mut split_count = 0;

    for line in input.lines() {
        for &i in &activate_next_iteration {
            if i != i.clamp(0, width - 1) {
                continue;
            }
            active_beams.set(i, true);
        }
        activate_next_iteration.clear();

        for (i, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    active_beams.set(i, true);
                }
                '^' => {
                    if !active_beams.get(i) {
                        continue;
                    }

                    active_beams.set(i, false);

                    // In case there are sequential '^' characters (didn't come up in the given input)
                    activate_next_iteration.push(i - 1);
                    activate_next_iteration.push(i + 1);

                    split_count += 1;
                }
                '.' => {
                    continue;
                }
                _ => {
                    panic!("Unexpected character: {}", c);
                }
            }
        }
    }

    println!("Split count: {}", split_count);
}

fn part2() {
    let input = input_to_string("day07.txt");

    let width = input.lines().nth(0).unwrap().len();
    let mut active_beams = vec![0u64; width].into_boxed_slice();
    let mut activate_next_iteration: Vec<(usize, u64)> = Vec::new();

    let mut split_count = 1;

    for line in input.lines() {
        for &i in &activate_next_iteration {
            if i.0 != i.0.clamp(0, width - 1) {
                continue;
            }
            active_beams[i.0] += i.1;
        }
        activate_next_iteration.clear();

        for (i, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    active_beams[i] += 1;
                }
                '^' => {
                    if active_beams[i] == 0 {
                        continue;
                    }

                    split_count += active_beams[i];

                    
                    // In case there are sequential '^' characters (didn't come up in the given input)
                    activate_next_iteration.push((i - 1, active_beams[i]));
                    activate_next_iteration.push((i + 1, active_beams[i]));

                    active_beams[i] = 0;
                }
                '.' => {
                    continue;
                }
                _ => {
                    panic!("Unexpected character: {}", c);
                }
            }
        }
    }

    println!("Split count: {}", split_count);
}

fn main() {
    part1();
    part2();
}
