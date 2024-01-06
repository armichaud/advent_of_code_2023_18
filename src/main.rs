mod flood_fill;
use std::{fs::File, io::{BufReader, BufRead}};

use flood_fill::{flood_fill, Direction};

struct Polygon {
    vertices: Vec<(i128, i128)>,
    boundary_points: i128,
}

fn get_vertices(file: &str) -> Polygon {
    let mut current: (i128, i128) = (0, 0);
    let mut vertices: Vec<(i128, i128)> = Vec::from([current]);
    let file = File::open(file).expect("Error opening file");
    let lines = BufReader::new(file).lines();
    let mut boundary_points = 0;
    for line in lines {
        let line = line.expect("Error reading line");
        let mut split = line.split_whitespace();
        split.next();
        split.next();
        let mut hex = split.next().unwrap().split("").filter(|c| !c.is_empty() && char::is_alphanumeric(c.chars().next().unwrap())).collect::<Vec<&str>>();
        let direction = Direction::from_int(hex.pop().unwrap().parse::<i32>().unwrap());
        let steps = i128::from_str_radix(&hex.concat(), 16).unwrap();
        boundary_points += steps;
        let vertex = direction.get_next_vertex(current, steps);
        vertices.push(vertex);
        current = vertex;
    }
    Polygon { vertices, boundary_points }
}

fn shoelace(vertices: &Vec<(i128, i128)>) -> i128 {
    let mut area = 0;
    for pair in vertices.windows(2) {
        let (x1, y1) = pair[0];
        let (x2, y2) = pair[1];
        area += (x1 * y2) - (x2 * y1);
    }
    area.abs() / 2
}

fn shoelace_and_pick(file: &str) -> i128 {
    let polygon = get_vertices(file);
    let area = shoelace(&polygon.vertices);
    // Pick's theorem: A = i + (b / 2) - 1, i.e. i = A - (b / 2) + 1
    // Here we want points including the boundary, so we add b and get i = A + (b / 2) + 1
    area + (polygon.boundary_points / 2) + 1 
}

fn main() {
    assert_eq!(flood_fill("example.txt", false), 62);
    assert_eq!(flood_fill("input.txt", false), 38188);
    assert_eq!(shoelace_and_pick("example.txt"), 952408144115);
    assert_eq!(shoelace_and_pick("input.txt"), 93325849869340);
}
