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

// Not actually a useful check but was an interesting exercise
fn _is_convex(points: &Vec<Point>) -> bool {
    let directions = [(0, 1), (-1, 0), (0, -1), (1, 0)];
    let mut min_direction = 3;

    for i in 1..=points.len() {
        let last_point = &points[i - 1];
        let current_point = &points[i % points.len()];

        let x_diff = current_point.x - last_point.x;
        let y_diff = current_point.y - last_point.y;
        let direction = (x_diff.signum(), y_diff.signum());

        if direction != directions[min_direction] && direction != directions[(min_direction + 1) % directions.len()] {
            min_direction += 1;
            min_direction = min_direction.rem_euclid(directions.len());

            if direction != directions[(min_direction + 1) % directions.len()] {
                return false;
            }
        }
    }

    true
}

// Also pointless
fn _calculate_normals(points: &Vec<Point>) -> Vec<(i8, i8)> {
    let mut normals = Vec::<(i8, i8)>::new();
    normals.reserve_exact(points.len());
    normals.push((0, 0));

    for i in 1..=points.len() {
        let prev_point = &points[i - 1];
        let curr_point = &points[i % points.len()];
        let next_point = &points[(i + 1) % points.len()];

        let vec0= ((prev_point.x - curr_point.x).signum(), (prev_point.y - curr_point.y).signum());
        let vec1 = ((next_point.x - curr_point.x).signum(), (next_point.y - curr_point.y).signum());
        
        fn turn_left(vec: &(i64, i64)) -> (i8, i8) {
            (-vec.1 as i8, vec.0 as i8)
        }

        let norm0 = turn_left(&vec0);
        let norm1 = turn_left(&vec1);

        let normal = (norm0.0 + norm1.0, norm0.1 + norm1.1);

        if i == points.len() {
            normals[0] = normal;
        } else {
            normals.push(normal);
        }
    }

    return normals;
}

struct Line<> {
    min: Point,
    max: Point
}

impl Line {
    fn intersects(&self, rect_min: &(i64, i64), rect_max: &(i64, i64)) -> bool {
        let diff = (self.max.x - self.min.x, self.max.y - self.min.y);

        if diff.0 != 0 {
            if self.min.y <= rect_min.1 || self.min.y >= rect_max.1 {
                return false;
            }

            if self.min.x <= rect_min.0 && self.max.x > rect_min.0 {
                return true;
            } else if self.max.x >= rect_max.0 && self.min.x < rect_max.0 {
                return true;
            } else if self.min.x > rect_min.0 && self.max.x < rect_max.0 {
                return true;
            }
        } else {
            if self.min.x <= rect_min.0 || self.min.x >= rect_max.0 {
                return false;
            }

            if self.min.y <= rect_min.1 && self.max.y > rect_min.1 {
                return true;
            } else if self.max.y >= rect_max.1 && self.min.y < rect_max.1 {
                return true;
            } else if self.min.y > rect_min.1 && self.max.y < rect_max.1 {
                return true;
            }
        }

        false
    }
}

fn part2() {
    let input = input_to_string("day09.txt");

    let mut points = Vec::new();
    let mut lines = Vec::new();

    for line in input.lines() {
        let (x_text, y_text) = line.split_at(line.find(",").unwrap());
        let y_text = &y_text[1..];
        points.push(Point { x: x_text.parse().unwrap(), y: y_text.parse().unwrap() });
    }

    for i in 0..points.len() {
        let p0 = &points[i];
        let p1 = &points[(i + 1) % points.len()];
        lines.push(Line {
            min: Point { x: std::cmp::min(p0.x, p1.x), y: std::cmp::min(p0.y, p1.y) },
            max: Point { x: std::cmp::max(p0.x, p1.x), y: std::cmp::max(p0.y, p1.y) }
        });
    }

    let mut areas = Vec::new();

    for i in 0..points.len() {
        for j in 0..i {
            let rect_min = (std::cmp::min(points[i].x, points[j].x), std::cmp::min(points[i].y, points[j].y));
            let rect_max = (std::cmp::max(points[i].x, points[j].x), std::cmp::max(points[i].y, points[j].y));

            let mut intersection = false;
            for line in &lines {
                if line.intersects(&rect_min, &rect_max) {
                    intersection = true;
                    break;
                }
            }

            if intersection {
                continue;
            }

            areas.push(points[i].area(&points[j]));
        }
    }

    areas.sort();
    println!("Greatest area: {}", areas.last().unwrap());
}

fn main() {
    part1();
    part2();
}
