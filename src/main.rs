use nalgebra::DMatrix;
use std::{fs::File, io::{BufReader, BufRead}};

const TRENCH: char = '#';
const TERRAIN: char = '.';

type Coord = (usize, usize);

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

    fn get_path_from_steps(self: &Self, start: Coord, steps: usize) -> Vec<Coord> {
        match self {
            Direction::Up => (start.0 - steps..start.0).rev().map(|x| (x, start.1)).collect(),
            Direction::Down => (start.0 + 1..start.0 + steps + 1).map(|x| (x, start.1)).collect(),
            Direction::Left => (start.1 - steps..start.1).rev().map(|y| (start.0, y)).collect(),
            Direction::Right => (start.1 + 1..start.1 + steps + 1).map(|y| (start.0, y)).collect(),
        }
    }
}

fn follow_path(file: &str) -> Vec<Coord> {
    let mut path: Vec<(usize, usize)> = Vec::new();
    let file = File::open(file).expect("Error opening file");
    let lines = BufReader::new(file).lines();
    let mut start = (0, 0);
    for line in lines {
        let line = line.expect("Error reading line");
        let mut split = line.split_whitespace();
        let direction = Direction::from_char(split.next().unwrap().chars().next().unwrap());
        let steps = split.next().unwrap().parse::<usize>().unwrap();
        // let hex = split.next().unwrap();
        path.append(&mut direction.get_path_from_steps(start, steps));
        start = path.last().unwrap().clone();
    }
    path
}

fn get_min_matrix_dimens(steps: &Vec<Coord>) -> Coord {
    let mut min_x = 0;
    let mut min_y = 0;
    for step in steps {
        min_x = min_x.max(step.0);
        min_y = min_y.max(step.1);
    }
    (min_x + 1, min_y + 1)
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
                while j < matrix.ncols() && matrix[(i, j)] == TERRAIN {
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

fn solution(file: &str) -> usize {
    let steps = follow_path(file);
    let dimens = get_min_matrix_dimens(&steps);
    let mut matrix = DMatrix::from_element(dimens.0, dimens.1, TERRAIN);
    for step in steps {
        matrix[step] = TRENCH;
    }
    fill_in_matrix(&mut matrix);
    println!("{}", matrix);
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
    assert_eq!(solution("example.txt"), 62);
    assert_eq!(solution("input.txt"), 0);
}
