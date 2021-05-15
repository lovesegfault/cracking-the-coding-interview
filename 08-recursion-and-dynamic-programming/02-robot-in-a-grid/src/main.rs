// Imagine a robot sitting on the upper left corner of grid with r rows and c columns. The robot
// can only move in two direction, right and down, but certain cells are "off limits" such that the
// robot cannot set on them. Design an algorithm to find a path for the robot from the top left to
// the bottom right.
use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Direction {
    Right,
    Down,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn walk(&self, direction: Direction) -> Self {
        use Direction::*;
        let mut new = self.clone();
        match direction {
            Right => new.x -= 1,
            Down => new.y -= 1,
        }
        new
    }

    pub fn is_origin(&self) -> bool {
        self.x == 0 && self.y == 0
    }
}

fn robot_in_a_grid(grid: &[Vec<bool>]) -> Vec<Point> {
    fn inner(
        grid: &[Vec<bool>],
        pos: Point,
        path: &mut Vec<Point>,
        failed_points: &mut HashSet<Point>,
    ) -> bool {
        if pos.is_origin()
            || inner(grid, pos.walk(Direction::Right), path, failed_points)
            || inner(grid, pos.walk(Direction::Down), path, failed_points)
        {
            path.push(pos);
            true
        } else {
            failed_points.insert(pos);
            false
        }
    }

    if grid.as_ref().len() == 0 {
        return Vec::new();
    }

    let mut failed_points: HashSet<Point> = HashSet::new();
    let mut path = Vec::new();

    let x = grid.len() - 1;
    let y = grid[0].len() - 1;
    let start = Point::new(x, y);

    if inner(grid, start, &mut path, &mut failed_points) {
        path
    } else {
        Vec::new()
    }
}

fn main() {
    println!("Hello, world!");
}
