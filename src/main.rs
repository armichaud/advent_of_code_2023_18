use nalgebra::DMatrix;
use std::{fs::File, io::{BufReader, BufRead}};


type coord = (usize, usize);

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

    fn get_path_from_steps(self: &Self, start: coord, steps: usize) -> Vec<coord> {
        match self {
            Direction::Up => (start.0..start.0 + steps).map(|x| (x, start.1)).collect(),
            Direction::Down => (start.0..start.0 - steps).map(|x| (x, start.1)).collect(),
            Direction::Left => (start.1..start.1 - steps).map(|y| (start.0, y)).collect(),
            Direction::Right => (start.1..start.1 + steps).map(|y| (start.0, y)).collect(),
        }
    }
}

fn follow_path(file: &str) -> Vec<coord> {
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

fn get_min_matrix_dimens(steps: &Vec<coord>) -> coord {
    let mut min_x = 0;
    let mut min_y = 0;
    for step in steps {
        min_x = min_x.max(step.0);
        min_y = min_y.max(step.1);
    }
    (min_x, min_y)
}

fn fill_in_trench(matrix: &mut DMatrix<char>) {}

fn solution(file: &str) -> usize {
    let steps = follow_path(file);
    let dimens = get_min_matrix_dimens(&steps);
    let mut trench = DMatrix::from_element(dimens.0, dimens.1, '.');
    for step in steps {
        trench[step] = '#';
    }
    fill_in_trench(&mut trench);
    let mut sum = 0;
    for i in 0..trench.nrows() {
        for j in 0..trench.ncols() {
            if trench[(i, j)] == '#' {
                sum += 1;
            }
        }
    }
    sum
}

fn main() {
    assert_eq!(solution("example.txt"), 62);
}
