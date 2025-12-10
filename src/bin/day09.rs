use advent_of_code_2025::input_to_string;

struct Point {
    x: i64,
    y: i64
}

impl Point {
    fn area(&self, other: &Self) -> i64 {
        ((self.x - other.x).abs() + 1) * ((self.y - other.y).abs() + 1)
    }
}

fn part1() {
    let input = input_to_string("day09.txt");

    let mut points = Vec::new();

    for line in input.lines() {
        let (x_text, y_text) = line.split_at(line.find(",").unwrap());
        let y_text = &y_text[1..];
        points.push(Point { x: x_text.parse().unwrap(), y: y_text.parse().unwrap() });
    }

    let mut areas = Vec::new();

    for i in 0..points.len() {
        for j in 0..i {
            areas.push(points[i].area(&points[j]));
        }
    }

    areas.sort();
    println!("Greatest area: {}", areas.last().unwrap());
}

fn part2() {
}

fn main() {
    part1();
    part2();
}
