use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Forward(u8),
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum AbsoluteDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Node {
    coordinate: (usize, usize),
    state: Direction,
    current: AbsoluteDirection,
}

impl Node {
    fn h(&self, target: &(usize, usize)) -> usize {
        let (x1, y1) = self.coordinate;
        let (x2, y2) = target;

        x1.abs_diff(*x2) + y1.abs_diff(*y2)
    }

    fn up(&self) -> Option<Node> {
        let (x, y) = self.coordinate;

        if y == 0 {
            return None;
        }

        let builder = |state| {
            Some(Node {
                coordinate: (x, y - 1),
                state,
                current: AbsoluteDirection::Up,
            })
        };

        match (&self.current, &self.state) {
            (AbsoluteDirection::Up, Direction::Forward(1)) => None,
            (AbsoluteDirection::Up, Direction::Forward(s)) => builder(Direction::Forward(*s + 1)),
            (AbsoluteDirection::Up, _) => builder(Direction::Forward(0)),
            (AbsoluteDirection::Left, _) => builder(Direction::Right),
            (AbsoluteDirection::Right, _) => builder(Direction::Left),
            (AbsoluteDirection::Down, _) => None,
        }
    }

    fn left(&self) -> Option<Node> {
        let (x, y) = self.coordinate;

        if x == 0 {
            return None;
        }

        let builder = |state| {
            Some(Node {
                coordinate: (x - 1, y),
                state,
                current: AbsoluteDirection::Left,
            })
        };

        match (&self.current, &self.state) {
            (AbsoluteDirection::Left, Direction::Forward(1)) => None,
            (AbsoluteDirection::Left, Direction::Forward(s)) => builder(Direction::Forward(*s + 1)),
            (AbsoluteDirection::Left, _) => builder(Direction::Forward(0)),
            (AbsoluteDirection::Up, _) => builder(Direction::Right),
            (AbsoluteDirection::Down, _) => builder(Direction::Left),
            (AbsoluteDirection::Right, _) => None,
        }
    }

    fn right(&self, target: &(usize, usize)) -> Option<Node> {
        let (x, y) = self.coordinate;

        if x >= target.0 {
            return None;
        }

        let builder = |state| {
            Some(Node {
                coordinate: (x + 1, y),
                state,
                current: AbsoluteDirection::Right,
            })
        };

        match (&self.current, &self.state) {
            (AbsoluteDirection::Right, Direction::Forward(1)) => None,
            (AbsoluteDirection::Right, Direction::Forward(s)) => {
                builder(Direction::Forward(*s + 1))
            }
            (AbsoluteDirection::Right, _) => builder(Direction::Forward(0)),
            (AbsoluteDirection::Up, _) => builder(Direction::Left),
            (AbsoluteDirection::Down, _) => builder(Direction::Right),
            (AbsoluteDirection::Left, _) => None,
        }
    }

    fn down(&self, target: &(usize, usize)) -> Option<Node> {
        let (x, y) = self.coordinate;

        if y >= target.1 {
            return None;
        }

        let builder = |state| {
            Some(Node {
                coordinate: (x, y + 1),
                state,
                current: AbsoluteDirection::Down,
            })
        };

        match (&self.current, &self.state) {
            (AbsoluteDirection::Down, Direction::Forward(1)) => None,
            (AbsoluteDirection::Down, Direction::Forward(s)) => builder(Direction::Forward(*s + 1)),
            (AbsoluteDirection::Down, _) => builder(Direction::Forward(0)),
            (AbsoluteDirection::Left, _) => builder(Direction::Right),
            (AbsoluteDirection::Right, _) => builder(Direction::Left),
            (AbsoluteDirection::Up, _) => None,
        }
    }

    fn neighbours(&self, target: &(usize, usize)) -> Vec<Node> {
        vec![
            self.up(),
            self.left(),
            self.right(target),
            self.down(target),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    let matrix: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|x| x.to_digit(10).unwrap() as u64)
                .collect()
        })
        .collect();

    let n = matrix[0].len();

    if matrix.iter().any(|x| x.len() != n) {
        panic!("Invalid input");
    }

    matrix
}

pub fn part_one(input: &str) -> i64 {
    let matrix = parse_input(input);

    let right = Node {
        coordinate: (1, 0),
        state: Direction::Forward(0),
        current: AbsoluteDirection::Right,
    };

    let down = Node {
        coordinate: (0, 1),
        state: Direction::Forward(0),
        current: AbsoluteDirection::Down,
    };

    let right = a_star(right, &matrix).unwrap_or(i64::MAX);
    let down = a_star(down, &matrix).unwrap_or(i64::MAX);

    right.min(down)
}

pub fn part_two(_input: &str) -> i64 {
    0
}

fn a_star(node: Node, matrix: &[Vec<u64>]) -> Option<i64> {
    let mut queue = PriorityQueue::new();

    let target = (matrix[0].len() - 1, matrix.len() - 1);

    let priority = node.h(&target) as u64;

    queue.push(node, Reverse(priority));

    let mut g_score = HashMap::new();

    let (x, y) = node.coordinate;

    g_score.insert(node.coordinate, matrix[y][x]);

    while !queue.is_empty() {
        let (node, _) = queue.pop().unwrap();

        if node.coordinate == target {
            return Some(*g_score.get(&target).unwrap() as i64);
        }

        for neighbour in node.neighbours(&target) {
            let (x, y) = neighbour.coordinate;

            let current = *g_score.get(&neighbour.coordinate).unwrap_or(&u64::MAX);
            let tentative = *g_score.get(&node.coordinate).unwrap() + matrix[y][x];

            let h = neighbour.h(&target);

            let priority = tentative + h as u64;

            if tentative < current {
                queue.push(neighbour, Reverse(priority));
                g_score.insert(neighbour.coordinate, tentative);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533"#;

        assert_eq!(part_one(input), 102);
    }
}
