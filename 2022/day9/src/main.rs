use std::{cmp::Ordering, collections::HashSet, fs::read_to_string};

#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash)]
struct Point2D {
    x: i32,
    y: i32,
}

impl From<char> for Point2D {
    fn from(direction: char) -> Self {
        match direction {
            'U' => Point2D::new(0, 1),
            'D' => Point2D::new(0, -1),
            'L' => Point2D::new(-1, 0),
            'R' => Point2D::new(1, 0),
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Debug)]
struct Move {
    direction: char,
    distance: i32,
}

impl Point2D {
    fn new(x: i32, y: i32) -> Point2D {
        Point2D { x, y }
    }

    fn step(&mut self, delta: Point2D) {
        self.x += delta.x;
        self.y += delta.y;
    }

    fn is_touching(&self, other: Point2D) -> bool {
        (self.x == other.x && (self.y - other.y).abs() == 1)
            || (self.y == other.y && (self.x - other.x).abs() == 1)
            || ((self.x - other.x).abs() == 1 && (self.y - other.y).abs() == 1)
    }

    fn get_delta_to(&self, target: Point2D) -> Point2D {
        let x = match self.x.cmp(&target.x) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            Ordering::Equal => 0,
        };
        let y = match self.y.cmp(&target.y) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            Ordering::Equal => 0,
        };
        Point2D::new(x, y)
    }
}

fn main() {
    let path = "input.txt";
    let input = read_to_string(path).unwrap();

    let moves: Vec<Move> = input
        .lines()
        .map(|line| {
            let (direction, distance) = line.split_at(1);
            Move {
                direction: direction.chars().next().unwrap(),
                distance: distance.trim().parse().unwrap(),
            }
        })
        .collect();

    let mut knots = vec![Point2D::new(0, 0); 10];
    // Create new hashmap for visited tail positions
    let mut visited: HashSet<Point2D> = HashSet::new();
    visited.insert(Point2D::new(0, 0));

    for move_ in moves {
        for _ in 0..move_.distance {
            let delta = Point2D::from(move_.direction);
            knots[0].step(delta);

            for i in 1..knots.len() {
                if !knots[i].is_touching(knots[i - 1]) {
                    let delta = knots[i].get_delta_to(knots[i - 1]);
                    knots[i].step(delta);
                }
            }

            visited.insert(knots[knots.len() - 1]);
        }
    }

    println!("Visited: {:?}", visited.len());
}
