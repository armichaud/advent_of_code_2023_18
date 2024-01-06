mod flood_fill;
use std::{fs::File, io::{BufReader, BufRead}};

use flood_fill::{flood_fill, Direction};

fn get_vertices(file: &str) -> Vec<(i128, i128)> {
    let mut current: (i128, i128) = (0, 0);
    let mut vertices: Vec<(i128, i128)> = Vec::from([current]);
    let file = File::open(file).expect("Error opening file");
    let lines = BufReader::new(file).lines();
    for line in lines {
        let line = line.expect("Error reading line");
        let mut split = line.split_whitespace();
        split.next();
        split.next();
        let mut hex = split.next().unwrap().split("").filter(|c| !c.is_empty() && char::is_alphanumeric(c.chars().next().unwrap())).collect::<Vec<&str>>();
        let direction = Direction::from_int(hex.pop().unwrap().parse::<i32>().unwrap());
        let steps = i128::from_str_radix(&hex.concat(), 16).unwrap();
        let vertex = direction.get_next_vertex(current, steps);
        vertices.push(vertex);
        current = vertex;
    }
    vertices
}

fn shoelace(file: &str) -> i128 {
    let steps = get_vertices(file);
    let mut sum = 0;
    for i in 0..steps.len() - 1 {
        let (x1, y1) = steps[i];
        let (x2, y2) = steps[i + 1];
        println!("x1: {} y1: {} x2: {} y2: {}", x1, y1, x2, y2);
        sum += (x1 * y2) - (x2 * y1);
    }
    sum.abs() / 2
}

fn main() {
    assert_eq!(flood_fill("example.txt", false), 62);
    assert_eq!(flood_fill("input.txt", false), 38188);
    assert_eq!(shoelace("example.txt"), 952408144115);
    //assert_eq!(shoelace("input.txt", true), 0);
}
