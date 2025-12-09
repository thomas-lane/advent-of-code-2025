use std::collections::HashSet;

use advent_of_code_2025::input_to_string;

struct Point {
    x: i64,
    y: i64,
    z: i64,
    chain: i32,
}

impl Point {
    fn distance(&self, other: &Point) -> f32 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f32).sqrt()
    }    
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

struct Distance {
    distance: f32,
    p0: usize,
    p1: usize
}

// There's an excessive amount of manual array indexing because
// the borrow checker did not like my references and I didn't
// feel like finding a better way when writing this.
fn part1() {
    let input = input_to_string("day08.txt");

    let mut points = Vec::new();
    for line in input.lines() {
        let mut coords_iter = line.split(",");
        points.push(Point {
            x: coords_iter.next().unwrap().parse().unwrap(),
            y: coords_iter.next().unwrap().parse().unwrap(),
            z: coords_iter.next().unwrap().parse().unwrap(),
            chain: -1
        });
    }

    let mut distances = Vec::new();
    for i in 0..points.len() {
        let (left, right) = points.split_at_mut(i);
        for (j, other_point) in left.iter().enumerate() {
            distances.push(Distance { distance: right[0].distance(other_point), p0: j, p1: i });
        }
    }

    distances.sort_by(|a, b| a.distance.total_cmp(&b.distance));

    let mut chains = Vec::new();
    let mut count = 0;
    for distance in distances {
        if count >= 1000 {
            break;
        }
        count += 1;
        
        let p0 = distance.p0;
        let p1 = distance.p1;

        if points[p0].chain != -1 && points[p0].chain == points[p1].chain {
            continue;
        }

        let (high_chain_point_index, low_chain_point_index) = if points[p0].chain > points[p1].chain { (p0, p1) } else { (p1, p0) };

        if points[high_chain_point_index].chain == -1 {
            let index = chains.len() as i32;
            points[high_chain_point_index].chain = index;
            points[low_chain_point_index].chain = index;

            chains.push(HashSet::from([distance.p0, distance.p1]));

            continue;
        }

        if points[low_chain_point_index].chain == -1 {
            chains[points[high_chain_point_index].chain as usize].insert(low_chain_point_index);
            points[low_chain_point_index].chain = points[high_chain_point_index].chain;
        } else {
            let dead_chain_contents = chains[points[low_chain_point_index].chain as usize].clone();
            chains[points[high_chain_point_index].chain as usize].extend(dead_chain_contents);

            let low_chain = points[low_chain_point_index].chain as usize;
            for &i in &chains[points[low_chain_point_index].chain as usize] {
                points[i].chain = points[high_chain_point_index].chain;
            }
            chains[low_chain].clear();
        }
    }

    chains.sort_by(|a, b| a.len().cmp(&b.len()));

    let mut result = 1;

    for value in chains.iter().rev().take(3) {
        let mult_value = std::cmp::max(value.len(), 1);
        result *= mult_value;
    }

    println!("Result: {}", result);
}

fn part2() {
    let input = input_to_string("day08.txt");

    let mut points = Vec::new();
    for line in input.lines() {
        let mut coords_iter = line.split(",");
        points.push(Point {
            x: coords_iter.next().unwrap().parse().unwrap(),
            y: coords_iter.next().unwrap().parse().unwrap(),
            z: coords_iter.next().unwrap().parse().unwrap(),
            chain: -1
        });
    }

    let mut distances = Vec::new();
    for i in 0..points.len() {
        let (left, right) = points.split_at_mut(i);
        for (j, other_point) in left.iter().enumerate() {
            distances.push(Distance { distance: right[0].distance(other_point), p0: j, p1: i });
        }
    }

    distances.sort_by(|a, b| a.distance.total_cmp(&b.distance));

    let mut chains = Vec::new();

    for distance in distances {
        let p0 = distance.p0;
        let p1 = distance.p1;

        if points[p0].chain != -1 && points[p0].chain == points[p1].chain {
            continue;
        }

        let (high_chain_point_index, low_chain_point_index) = if points[p0].chain > points[p1].chain { (p0, p1) } else { (p1, p0) };

        if points[high_chain_point_index].chain == -1 {
            let index = chains.len() as i32;
            points[high_chain_point_index].chain = index;
            points[low_chain_point_index].chain = index;

            chains.push(HashSet::from([distance.p0, distance.p1]));

            continue;
        }

        if points[low_chain_point_index].chain == -1 {
            chains[points[high_chain_point_index].chain as usize].insert(low_chain_point_index);
            points[low_chain_point_index].chain = points[high_chain_point_index].chain;
        } else {
            let dead_chain_contents = chains[points[low_chain_point_index].chain as usize].clone();
            chains[points[high_chain_point_index].chain as usize].extend(dead_chain_contents);

            let low_chain = points[low_chain_point_index].chain as usize;
            for &i in &chains[points[low_chain_point_index].chain as usize] {
                points[i].chain = points[high_chain_point_index].chain;
            }
            chains[low_chain].clear();
        }

        if points[0].chain != -1 && chains[points[0].chain as usize].len() == points.len() {
            let x0 = points[distance.p0].x;
            let x1 = points[distance.p1].x;
            println!("Result: {}", x0 * x1);
            return;
        }
    }

    panic!("Unable to connect all points");
}

fn main() {
    part1();
    part2();
}
