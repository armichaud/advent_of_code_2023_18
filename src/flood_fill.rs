use nalgebra::DMatrix;
use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet};

const TRENCH: char = '#';
const TERRAIN: char = '.';

pub type Coord = (i32, i32);

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }

    pub fn from_int(i: i32) -> Direction {
        match i {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            _ => panic!("Invalid direction"),
        }
    }

    pub fn get_next_vertex(self: &Self, start: (i128, i128), steps: i128) -> (i128, i128) {
        match self {
            Direction::Up => (start.0, start.1 + steps),
            Direction::Down => (start.0, start.1 - steps),
            Direction::Left => (start.0 - steps, start.1),
            Direction::Right => (start.0 + steps, start.1),
        }
    }

    pub fn get_path_from_steps(self: &Self, start: Coord, steps: i32) -> Vec<Coord> {
        match self {
            Direction::Up => (start.0 - steps..start.0).rev().map(|x| (x, start.1)).collect(),
            Direction::Down => (start.0 + 1..start.0 + steps + 1).map(|x| (x, start.1)).collect(),
            Direction::Left => (start.1 - steps..start.1).rev().map(|y| (start.0, y)).collect(),
            Direction::Right => (start.1 + 1..start.1 + steps + 1).map(|y| (start.0, y)).collect(),
        }
    }
}

pub fn follow_path(file: &str, use_hex_instructions: bool) -> Vec<Coord> {
    let mut path: Vec<(i32, i32)> = Vec::new();
    let file = File::open(file).expect("Error opening file");
    let lines = BufReader::new(file).lines();
    let mut start = (0, 0);
    for line in lines {
        let line = line.expect("Error reading line");
        let mut split = line.split_whitespace();
        let mut direction = Direction::from_char(split.next().unwrap().chars().next().unwrap());
        let mut steps = split.next().unwrap().parse::<i32>().unwrap();
        if use_hex_instructions {
            let mut hex = split.next().unwrap().split("").filter(|c| !c.is_empty() && char::is_alphanumeric(c.chars().next().unwrap())).collect::<Vec<&str>>();
            direction = Direction::from_int(hex.pop().unwrap().parse::<i32>().unwrap());
            steps = i32::from_str_radix(&hex.concat(), 16).unwrap();
        }
        path.append(&mut direction.get_path_from_steps(start, steps));
        start = path.last().unwrap().clone();
    }
    path
}

fn get_min_matrix_dimens(steps: &Vec<Coord>) -> (usize, usize, i32, i32) {
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for step in steps {
        min_x = min_x.min(step.0);
        min_y = min_y.min(step.1);
        max_x = max_x.max(step.0);
        max_y = max_y.max(step.1);
    }
    ((max_x + 1 - min_x).try_into().unwrap(), (max_y + 1 - min_y).try_into().unwrap(), min_x.abs(), min_y.abs())
}

fn fill_in_matrix(matrix: &mut DMatrix<char>) {
    for i in 1..matrix.nrows() - 1 {
        let mut paint = false;
        let mut j = 0;
        while j < matrix.ncols() {
            let current = matrix[(i, j)];
            if current == TRENCH && !paint {
                paint = true;
            } else if current == TERRAIN && paint {
                let mut potential_trench = Vec::new();
                while j < matrix.ncols() && matrix[(i, j)] == TERRAIN && matrix[(i - 1, j)] == TRENCH {
                    potential_trench.push((i, j));
                    j += 1;
                }
                if j == matrix.ncols() {
                    break;
                }
                for coord in potential_trench { 
                    matrix[coord] = TRENCH;
                }
                paint = false;
            }
            j += 1;
        }
    }
}

#[allow(dead_code)]
fn get_cell(terrain: &HashSet<(i32, i32)>, coords: (usize, usize), row_offset: i32, col_offset: i32) -> char {
    if terrain.contains(&(coords.0  as i32 - row_offset, coords.1 as i32- col_offset)) { TRENCH } else { TERRAIN }
}

#[allow(dead_code)]
fn fill_in_hypothetical_matrix(dimens: (usize, usize, i32, i32), terrain: &mut HashSet<(i32, i32)>) -> usize {
    let row_stop = dimens.0 - 1;
    let col_stop = dimens.1 - 1;
    for i in 1..row_stop {
        let mut paint = false;
        let mut j = 0;
        while j < col_stop {
            let current = get_cell(&terrain, (i, j), dimens.2, dimens.3);
            if current == TRENCH && !paint {
                paint = true;
            } else if current == TERRAIN && paint {
                let mut potential_trench = Vec::new();
                while j < dimens.1 && get_cell(&terrain, (i, j), dimens.2, dimens.3) == TERRAIN && get_cell(&terrain, (i - 1, j), dimens.2, dimens.3) == TRENCH {
                    potential_trench.push((i, j));
                    j += 1;
                }
                if j == dimens.1 {
                    break;
                }
                for coord in potential_trench { 
                    let coord = (coord.0 as i32, coord.1 as i32);
                    terrain.insert(coord);
                }
                paint = false;
            }
            j += 1;
        }
    }
    terrain.len()
}

fn space_around(trench: HashSet<Coord>, nrows: usize, ncols: usize) -> usize {
    let last_row = nrows as i32 - 1;
    let last_col = ncols as i32 - 1;
    let mut sum = 0;
    let mut stack = Vec::<Coord>::new();
    for i in 0..nrows {
        if !trench.contains(&(i as i32, 0)) {
            stack.push((i as i32, 0));
        }
        if !trench.contains(&(i as i32, last_col)) {
            stack.push((i as i32, last_col));
        }
    }
    for j in 0..ncols {
        if !trench.contains(&(0, j as i32)) {
            stack.push((0, j as i32));
        }
        if !trench.contains(&(last_row, j as i32)) {
            stack.push((last_row, j as i32));
        }
    }
    let mut visited = HashSet::new();
    while stack.len() > 0 {
        let current = stack.pop().unwrap();
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);
        sum += 1;
        let mut neighbors = Vec::<Coord>::new();
        if current.0 > 0 {
            neighbors.push((current.0 - 1, current.1));
        }
        if current.0 < nrows as i32 - 1 {
            neighbors.push((current.0 + 1, current.1));
        }
        if current.1 > 0 {
            neighbors.push((current.0, current.1 - 1));
        }
        if current.1 < ncols as i32 - 1 {
            neighbors.push((current.0, current.1 + 1));
        }
        for neighbor in neighbors {
            if !visited.contains(&neighbor) && !trench.contains(&neighbor) {
                stack.push(neighbor);
            }
        }
    }
    sum
}

pub fn flood_fill(file: &str, use_hex_instructions: bool) -> usize {
    let steps = follow_path(file, use_hex_instructions);
    let dimens = get_min_matrix_dimens(&steps);
    let mut sum = 0;
    if !use_hex_instructions {
        let mut matrix = DMatrix::from_element(dimens.0, dimens.1, TERRAIN);
        for step in steps {
            matrix[((step.0 + dimens.2) as usize, (step.1 + dimens.3) as usize)] = TRENCH;
        }
        fill_in_matrix(&mut matrix);
        for i in 0..matrix.nrows() {
            for j in 0..matrix.ncols() {
                if matrix[(i, j)] == TRENCH {
                    sum += 1;
                }
            }
        }
    } else {
        // sum = fill_in_hypothetical_matrix(dimens, &mut steps.iter().map(|x| (x.0, x.1)).collect());
        sum = (dimens.0 * dimens.1) - space_around(steps.iter().map(|x| (x.0, x.1)).collect(), dimens.0, dimens.1);
    }
    sum
}
