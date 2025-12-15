use advent_of_code_2025::{input_to_string, BitArray};

fn parse_line(line: &str) -> (BitArray, Vec<BitArray>) {
    let left_square_index = line.find("[").unwrap();
    let right_square_index = line.find("]").unwrap();

    let light_len = right_square_index - left_square_index - 1;
    let mut light_bits = BitArray::new(light_len);

    let start_index = left_square_index + 1;
    for i in start_index..right_square_index {
        let c = line.as_bytes()[i] as char;

        match c {
            '.' => {}
            '#' => light_bits.set(i - start_index, true),
            _ => panic!("Unexpected indicator light value: {}", c)
        }
    }

    let left_curly_index = line.find("{").unwrap();

    let mut buttons = Vec::new();
    let mut current_number = 0;
    for i in right_square_index + 1..left_curly_index {
        let c = line.as_bytes()[i] as char;

        match c {
            '(' => {
                buttons.push(BitArray::new(light_bits.len()));
            }
            ')' | ',' => {
                buttons.last_mut().unwrap().set(current_number, true);
                current_number = 0;
            }
            '0'..='9' => {
                current_number *= 10;
                current_number += c.to_digit(10).unwrap() as usize;
            }
            ' ' => (),
            _ => panic!("Unexpected character: {}", c)
        }
    }

    (light_bits, buttons)
}

fn button_presses(line: &str) -> i32 {
    let (light_bits, buttons) = parse_line(line);

    let button_combination_count = 2_u32.pow(buttons.len() as u32);
    let mut combinations = vec![BitArray::new(light_bits.len()); button_combination_count as usize + 1];

    if light_bits == BitArray::new(light_bits.len())
    {
        return 0;
    }

    let mut lowest_presses = i32::MAX;

    for i in 1..button_combination_count as usize {
        // Check if a single bit is set
        if i & (i - 1) == 0 {
            combinations[i] = buttons[i.trailing_zeros() as usize].clone();
            if combinations[i] == light_bits {
                return 1;
            }
            continue;
        }

        let most_significant_bit = (buttons.len() as u32 - 1) - (i.leading_zeros() - (std::mem::size_of_val(&i) as u32 * 8 - buttons.len() as u32));
        let uncombined_index = 2usize.pow(most_significant_bit);
        let prev_combined_index = i - uncombined_index;

        combinations[i] = combinations[uncombined_index].xor(&combinations[prev_combined_index]);

        if combinations[i] == light_bits {
            lowest_presses = std::cmp::min(i.count_ones() as i32, lowest_presses);
        }
    }

    assert_ne!(lowest_presses, i32::MAX);

    lowest_presses
}

fn part1() {
    let input = input_to_string("day10.txt");

    let result = input.lines().map(|line| button_presses(line)).sum::<i32>();
    println!("Result: {}", result);
}

use good_lp::{Constraint, Expression, ProblemVariables, Solution, SolverModel, Variable, constraint, default_solver, variable, variables};

fn parse_line_joltage(line: &str) -> (ProblemVariables, Vec<Variable>, Vec<Constraint>) {
    let left_square_index = line.find("[").unwrap();
    let right_square_index = line.find("]").unwrap();

    let light_len = right_square_index - left_square_index - 1;

    let left_curly_index = line.find("{").unwrap();
    let right_curly_index = line.find("}").unwrap();

    let mut buttons = Vec::new();
    let mut current_number = 0;
    for i in right_square_index + 1..left_curly_index {
        let c = line.as_bytes()[i] as char;

        match c {
            '(' => {
                buttons.push(BitArray::new(light_len));
            }
            ')' | ',' => {
                buttons.last_mut().unwrap().set(current_number, true);
                current_number = 0;
            }
            '0'..='9' => {
                current_number *= 10;
                current_number += c.to_digit(10).unwrap() as usize;
            }
            ' ' => (),
            _ => panic!("Unexpected character: '{}'", c)
        }
    }

    let mut problem_variables = ProblemVariables::new();
    let variables: Vec<Variable> = (0..buttons.len()).map(|_| problem_variables.add(variable().min(0).integer())).collect();

    let mut targets = Vec::new();
    let mut current_number = 0;
    for i in left_curly_index + 1..right_curly_index {
        let c = line.as_bytes()[i] as char;

        match c {
            ',' => {
                targets.push(current_number);
                current_number = 0;
            }
            '0'..='9' => {
                current_number *= 10;
                current_number += c.to_digit(10).unwrap() as u32;
            }
            _ => panic!("Unexpected character: '{}'", c)
        }
    }
    targets.push(current_number);

    let mut constraints = Vec::new();
    for i in 0..light_len {
        let expression: Expression = variables.iter().enumerate().map(|(j, v)| (buttons[j].get(i) as i32) * *v).sum();
        constraints.push(expression.eq(targets[i]));
    }

    (problem_variables, variables, constraints)
}

fn part2() {
    let input = input_to_string("day10.txt");

    let result: f64 = input.lines()
        .map(parse_line_joltage)
        .map(|(p, v, c)| {
            let solution = p.minimise(v.iter().sum::<Expression>())
                .using(default_solver)
                .with_all(c)
                .solve()
                .unwrap();
            v.iter().map(|v| solution.value(*v)).sum::<f64>()
        })
        .sum();

    println!("Result: {}", result);
}

fn main() {
    part1();
    part2();
}
