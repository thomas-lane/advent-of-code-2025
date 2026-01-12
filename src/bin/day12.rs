use advent_of_code_2025::{input_to_string, BitArray};

#[derive(Clone)]
struct Grid {
    grid: BitArray,

    width: i64,
    height: i64
}

impl Grid {
    fn new(width: i64, height: i64) -> Self {
        Self { grid: BitArray::new((width * height).try_into().unwrap()), width, height }
    }

    fn get(&self, x: i64, y: i64) -> bool {
        if x < 0 || y < 0 {
            return true;
        }

        if x >= self.width || y >= self.height {
            return true;
        }

        let index = (y * self.width + x) as usize;

        if index >= self.grid.len() {
            return true;
        }

        self.grid.get(index)
    }

    fn put(&mut self, x: i64, y: i64) {
        self.grid.set((y * self.width + x) as usize, true);
    }

    fn can_put(&self, other: &Grid, x_pos: i64, y_pos: i64) -> bool {
        let center_x = other.width / 2;
        let center_y = other.height / 2;

        // Check if it fits
        for x in 0..other.width {
            for y in 0..other.height {
                if self.get(x_pos - center_x + x, y_pos - center_y + y) && other.get(x, y) {
                    return false;
                }
            }
        }

        true
    }

    fn put_grid(&self, other: &Grid, x_pos: i64, y_pos: i64) -> Grid {
        let mut after_put = self.clone();

        let center_x = other.width / 2;
        let center_y = other.height / 2;

        // Place the grid
        for x in 0..other.width {
            for y in 0..other.height {
                if other.get(x, y) {
                    after_put.put(x_pos - center_x + x, y_pos - center_y + y)
                }
            }
        }

        after_put
    }

    fn get_rotated(&self, rotation_count: i32) -> Grid {
        assert_eq!(self.width, self.height);
        assert!(rotation_count > 0);
        let mut rotated_grid = Grid::new(self.width, self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) {
                    rotated_grid.put(self.width - y - 1, x);
                }
            }
        }

        if rotation_count > 1 {
            return rotated_grid.get_rotated(rotation_count - 1);
        }

        rotated_grid
    }

    #[allow(dead_code)] // Never used
    fn get_flipped_x(&self) -> Grid {
        assert_eq!(self.width, self.height);
        let mut flipped_grid = Grid::new(self.width, self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) {
                    flipped_grid.put(self.width - x - 1, y);
                }
            }
        }

        flipped_grid
    }

    #[allow(dead_code)] // Never used
    fn get_flipped_y(&self) -> Grid {
        assert_eq!(self.width, self.height);
        let mut flipped_grid = Grid::new(self.width, self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) {
                    flipped_grid.put(x, self.height - y - 1);
                }
            }
        }

        flipped_grid
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                output.push_str(&format!("{}", if self.get(x, y) { '#' } else { '.' }));
            }
            output.push('\n');
        }

        write!(f, "{}", output)
    }
}

fn parse_pieces(input: &str) -> Vec<Grid> {
    let mut pieces = Vec::new();

    for piece in input.split("\n\n") {
        if piece.lines().next().unwrap().contains("x") {
            break;
        }

        let mut grid = Grid::new(3, 3);
        let mut y = 0;
        for line in piece.lines() {
            if line.contains(":") {
                continue;
            }

            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    grid.put(x as i64, y);
                }
            }

            y += 1;
        }

        pieces.push(grid);
    }

    pieces
}

struct Problem {
    grid_width: i64,
    grid_height: i64,
    piece_counts: Vec<i64>,
}

impl Problem {
    fn parse(input: &str) -> Vec<Self> {
        let mut problems = Vec::new();

        for line in input.lines() {
            let Some(x_position) = line.find('x') else {
                continue;
            };

            let width_str = &line[0..x_position];
            let height_str = &line[x_position + 1..line.find(':').unwrap()];

            let width = width_str.parse::<i64>().unwrap();
            let height = height_str.parse::<i64>().unwrap();

            let pieces_str = &line[line.find(' ').unwrap() + 1..line.len()];
            let mut piece_counts = Vec::new();
            for piece_count_str in pieces_str.split(' ') {
                piece_counts.push(piece_count_str.parse::<i64>().unwrap());
            }

            problems.push(Problem { grid_width: width, grid_height: height, piece_counts })
        }

        problems
    }
}

fn recursive_solve(grid: Grid, mut piece_counts: Vec<i64>, pieces: &Vec<Grid>, memoization: &mut std::collections::HashSet<BitArray>) -> bool {
    if memoization.len() > 10 {
        return false;
    }

    if memoization.contains(&grid.grid) {
        return false;
    }

    let mut attempted_placements = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            for (i, piece) in pieces.iter().enumerate() {
                if piece_counts[i] <= 0 {
                    continue;
                }

                for rotation_count in 0..=3 {
                    let rotated_piece = if rotation_count > 0 { &piece.get_rotated(rotation_count) } else { piece };

                    if grid.can_put(rotated_piece, x, y) {
                        attempted_placements += 1;
                        piece_counts[i] -= 1;

                        if piece_counts.iter().sum::<i64>() <= 0 {
                            return true;
                        }

                        if recursive_solve(grid.put_grid(rotated_piece, x, y), piece_counts.clone(), pieces, memoization) {
                            return true;
                        } else {
                            piece_counts[i] += 1;
                        }

                        if attempted_placements > 1 {
                            return false;
                        }
                    }
                }
            }
        }
    }

    memoization.insert(grid.grid);

    false
}

fn part1() {
    let input = input_to_string("day12.txt");

    let pieces = parse_pieces(&input);
    let problems = Problem::parse(&input);

    let solvable = problems.iter().enumerate().filter(|(i, problem)| {
        if recursive_solve(Grid::new(problem.grid_width, problem.grid_height), problem.piece_counts.clone(), &pieces, &mut std::collections::HashSet::<BitArray>::new()) {
            println!("{}/{} | Solvable!", i + 1, problems.len());
            return true;
        }

        println!("{}/{} | Not solvable...", i + 1, problems.len());
        false
    }).count();

    println!("Solvable: {}", solvable);
}

fn main() {
    part1();
}
