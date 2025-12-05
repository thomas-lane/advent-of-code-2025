use advent_of_code_2025::input_to_string;

fn part1() {
    let input = input_to_string("day05.txt");
    
    let mut ranges = Vec::new();
    let mut ingredients = Vec::new();

    for line in input.lines() {
        if line.contains("-") {
            let mut split = line.split("-");
            let low = split.next().unwrap().parse::<u64>().unwrap();
            let high = split.next().unwrap().parse::<u64>().unwrap();

            ranges.push((low, high));
        } else if !line.is_empty() {
            ingredients.push(line.parse::<u64>().unwrap());
        }
    }

    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    ingredients.sort();

    let mut fresh_count = 0;

    let mut ingredients_index: usize = 0;
    for range in ranges {
        while ingredients[ingredients_index] <= range.1 {
            if ingredients[ingredients_index] >= range.0 {
                fresh_count += 1;
            }

            ingredients_index += 1;
        }
    }

    println!("Fresh ingredients: {}", fresh_count);
}

fn part2() {
    let input = input_to_string("day05.txt");
    
    let mut ranges = Vec::new();

    for line in input.lines() {
        if line.contains("-") {
            let mut split = line.split("-");
            let low = split.next().unwrap().parse::<u64>().unwrap();
            let high = split.next().unwrap().parse::<u64>().unwrap();

            ranges.push((low, high));
        }
    }

    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut fresh_count = ranges[0].1 - ranges[0].0 + 1;
    let mut overlapping_range_boundary = ranges[0].1;

    for range in ranges {
        if range.0 > overlapping_range_boundary {
            overlapping_range_boundary = range.1;
            fresh_count += range.1 - range.0 + 1;
        } else if range.1 > overlapping_range_boundary {
            fresh_count += range.1 - overlapping_range_boundary;
            overlapping_range_boundary = range.1;
        }
    }

    println!("Fresh ingredients: {}", fresh_count);
}

fn main() {
    part1();
    part2();
}
