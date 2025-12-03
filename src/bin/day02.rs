use advent_of_code_2025::input_to_string;

fn part1() {
    let input = input_to_string("day02.txt").trim().to_owned();
    let ranges = input.split(",");
    let mut result = 0;

    for range in ranges {
        let hyphen_index = range.find("-").expect("No hyphen in range");
        let start: u64 = range[..hyphen_index].parse().expect("Couldn't parse start");
        let end: u64 = range[hyphen_index + 1..].parse().expect("Couldn't parse end");

        for i in start..=end {
            let str_i = i.to_string();

            if str_i.len() % 2 != 0 {
                continue;
            }

            if str_i[..str_i.len() / 2] == str_i[str_i.len() / 2..] {
                result += i;
            }
        }
    }

    println!("Result: {}", result);
}

fn find_divisors(number: u64) -> Vec<u64> {
    let mut divisors = Vec::new();

    for i in 1..=((number as f32).sqrt() as u64) {
        if number % i == 0 {
            divisors.push(i);
        }
    }

    for i in (0..divisors.len()).rev() {
        divisors.push(number / divisors[i]);
    }

    divisors
}

fn part2() {
    let input = input_to_string("day02.txt").trim().to_owned();
    let ranges = input.split(",");
    let mut result = 0;

    for range in ranges {
        let hyphen_index = range.find("-").expect("No hyphen in range");
        let start: u64 = range[..hyphen_index].parse().expect("Couldn't parse start");
        let end: u64 = range[hyphen_index + 1..].parse().expect("Couldn't parse end");

        for i in start..=end {
            let str_i = i.to_string();

            let divisors = find_divisors(str_i.len() as u64);

            for divisor in divisors.iter().copied().rev() {
                if divisor == str_i.len() as u64 {
                    continue;
                }

                let slice0 = &str_i[0..divisor as usize];
                let mut no_match = false;

                for slice in 1..(str_i.len() / divisor as usize) {
                    let offset = (divisor * slice as u64) as usize;
                    if slice0 != &str_i[offset..offset + divisor as usize] {
                        no_match = true;
                        break;
                    }
                }

                if !no_match {
                    result += i;
                    break;
                }
            }
        }
    }

    println!("Result: {}", result);
}

fn main() {
    part1();
    part2();
}
