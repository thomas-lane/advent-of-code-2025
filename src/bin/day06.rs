use advent_of_code_2025::input_to_string;

fn part1() {
    let input = input_to_string("day06.txt");

    let mut columns = Vec::new();
    let mut operators = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let mut column_index = 0; // Handle manually in case of empty strings from line.split(" ")
        for value in line.split(" ") {
            if value.is_empty() {
                continue;
            }

            let parsed = value.parse::<i64>();
            match parsed {
                Ok(parsed) => {
                    if i == 0 {
                        columns.push(vec![parsed]);
                    } else {
                        columns[column_index].push(parsed);
                    }
                }
                Err(_) => {
                    assert_eq!(value.len(), 1);
                    operators.push(value.as_bytes()[0]);
                }
            };

            column_index += 1;
        }
    }

    let mut answers = Vec::new();
    for (i, column) in columns.iter().enumerate() {
        let operator = operators[i];
        let mut total = 0;
        let func = match operator {
            b'+' => {
                |x, y| x + y
            }
            b'*' => {
                total = 1;
                |x, y| x * y
            }
            _ => {
                panic!("Unknown operator: {}", operator as char);
            }
        };

        for value in column {
            total = func(total, value);
        }
        answers.push(total);
    }

    let mut result = 0;
    for answer in answers {
        result += answer;
    }

    println!("Result: {}", result);
}

fn get_char_at(input: &String, widths: &Vec<usize>, x: usize, y: usize) -> char {
    let mut index = 0;

    if x >= widths[y] {
        return ' ';
    }

    for i in 0..y {
        index += widths[i];
    }

    index += x;

    assert!(index < input.len());

    input.as_bytes()[index] as char
}

fn part2() {
    let input = input_to_string("day06.txt");

    let mut widths = Vec::new();
    let mut height = 0;
    let mut last_line = &input[0..0];
    for line in input.lines() {
        assert!(!line.is_empty());
        widths.push(line.len() + 1); // Add 1 for newline
        height += 1;
        last_line = line;
    }

    struct Problem {
        min_x: usize,
        max_x: usize,
        operator: char,
    }

    let mut problems = Vec::new();

    let mut starting_x = 0;
    let mut starting_operator = ' ';
    for (i, c) in last_line.chars().enumerate() {
        if c != ' ' {
            if i != 0 {
                problems.push(Problem { min_x: starting_x, max_x: i - 2, operator: starting_operator });
            }

            starting_x = i;
            starting_operator = c;
        }
    }

    let max_width = widths.iter().max().unwrap();
    problems.push(Problem { min_x: starting_x, max_x: max_width - 2, operator: starting_operator});

    let mut answers = Vec::new();

    for problem in problems {
        let mut values = Vec::new();

        for x in (problem.min_x..=problem.max_x).rev() {
            let mut value = 0;
            for y in 0..height - 1 {
                let c = get_char_at(&input, &widths, x, y);
                if let Some(digit) = c.to_digit(10) {
                    value *= 10;
                    value += digit as u64;
                }
            }

            values.push(value);
        }

        let mut total = 0;
        let func = match problem.operator {
            '+' => {
                |x, y| x + y
            }
            '*' => {
                total = 1;
                |x, y| x * y
            }
            _ => {
                panic!("Unknown operator: \"{}\"", problem.operator);
            }
        };

        for value in values {
            total = func(total, value);
        }

        answers.push(total);
    }

    let result: u64 = answers.iter().sum();

    println!("Result: {}", result);
}

fn main() {
    part1();
    part2();
}
