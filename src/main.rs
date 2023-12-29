use nalgebra::DMatrix;
use std::{fs::File, io::{BufReader, BufRead}};

const TRENCH: char = '#';
const TERRAIN: char = '.';

type Coord = (i32, i32);

enum Direction {
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

    fn from_int(i: i32) -> Direction {
        match i {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            _ => panic!("Invalid direction"),
        }
    }

    fn get_path_from_steps(self: &Self, start: Coord, steps: i32) -> Vec<Coord> {
        match self {
            Direction::Up => (start.0 - steps..start.0).rev().map(|x| (x, start.1)).collect(),
            Direction::Down => (start.0 + 1..start.0 + steps + 1).map(|x| (x, start.1)).collect(),
            Direction::Left => (start.1 - steps..start.1).rev().map(|y| (start.0, y)).collect(),
            Direction::Right => (start.1 + 1..start.1 + steps + 1).map(|y| (start.0, y)).collect(),
        }
    }
}

fn follow_path(file: &str, use_hex_instructions: bool) -> Vec<Coord> {
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

fn solution(file: &str, use_hex_instructions: bool) -> usize {
    let steps = follow_path(file, use_hex_instructions);
    let dimens = get_min_matrix_dimens(&steps);
    let mut matrix = DMatrix::from_element(dimens.0, dimens.1, TERRAIN);
    for step in steps {
        matrix[((step.0 + dimens.2) as usize, (step.1 + dimens.3) as usize)] = TRENCH;
    }
    fill_in_matrix(&mut matrix);
    let mut sum = 0;
    for i in 0..matrix.nrows() {
        for j in 0..matrix.ncols() {
            if matrix[(i, j)] == TRENCH {
                sum += 1;
            }
        }
    }
    sum
}

fn main() {
    //assert_eq!(solution("example.txt", false), 62);
    //assert_eq!(solution("input.txt", false), 38188);
    assert_eq!(solution("example.txt", true), 952408144115);
    //assert_eq!(solution("input.txt", true), 0);
}
