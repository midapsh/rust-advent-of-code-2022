use std::collections::BTreeSet;

const _DUMMY_INPUT: &str = include_str!("data/day9-dummy.txt");
const _DUMMY_INPUT2: &str = include_str!("data/day9-dummy2.txt");
const REAL_INPUT: &str = include_str!("data/day9-real.txt");

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => panic!("Invalid Direction"),
        }
    }
}

impl From<(i32, i32)> for Direction {
    fn from(value: (i32, i32)) -> Self {
        match value {
            (-1, 0) => Self::Up,
            (1, 0) => Self::Down,
            (0, -1) => Self::Left,
            (0, 1) => Self::Right,
            _ => panic!("Invalid Direction"),
        }
    }
}

impl From<&Direction> for (i32, i32) {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

fn parse(values: &str) -> Vec<(Direction, i32)> {
    values
        .lines()
        .map(|x| {
            let (raw_direction, raw_number_of_steps) = x.split_once(' ').unwrap();
            (
                Direction::from(raw_direction),
                raw_number_of_steps.parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<_>>()
}

#[derive(Default)]
struct Snake {
    head: (i32, i32),
    tail: (i32, i32),
    visited: BTreeSet<(i32, i32)>,
}

impl Snake {
    fn make_move(&mut self, direction: &Direction) {
        let delta: (i32, i32) = direction.into();
        self.head.0 += delta.0;
        self.head.1 += delta.1;

        let row_diff = self.head.0 - self.tail.0;
        let col_diff = self.head.1 - self.tail.1;

        if (row_diff == 0) & (col_diff.abs() > 1) {
            // GOTO Right/Left
            self.tail.1 += col_diff.signum();
        } else if (col_diff == 0) & (row_diff.abs() > 1) {
            // GOTO Up/Down
            self.tail.0 += row_diff.signum();
        } else if (row_diff.abs() > 1) | (col_diff.abs() > 1) {
            // GOTO Diagonal
            self.tail.0 += row_diff.signum();
            self.tail.1 += col_diff.signum();
        }

        self.visited.insert(self.tail);
    }
}

struct BigSnake {
    segments: Vec<(i32, i32)>,
    visited: BTreeSet<(i32, i32)>,
}

impl BigSnake {
    fn new(len: usize) -> Self {
        if len < 2 {
            panic!("Snake should at least have a tail and a head (len>=2)");
        }
        Self {
            segments: vec![(0, 0); len],
            visited: BTreeSet::new(),
        }
    }
}

impl Default for BigSnake {
    fn default() -> Self {
        Self::new(2)
    }
}

impl BigSnake {
    fn make_move(&mut self, direction: &Direction) {
        let delta: (i32, i32) = direction.into();
        self.segments[0].0 += delta.0;
        self.segments[0].1 += delta.1;

        for i in 1..self.segments.len() {
            let row_diff = self.segments[i - 1].0 - self.segments[i].0;
            let col_diff = self.segments[i - 1].1 - self.segments[i].1;

            if (row_diff == 0) & (col_diff.abs() > 1) {
                // GOTO Right/Left
                self.segments[i].1 += col_diff.signum();
            } else if (col_diff == 0) & (row_diff.abs() > 1) {
                // GOTO Up/Down
                self.segments[i].0 += row_diff.signum();
            } else if (row_diff.abs() > 1) | (col_diff.abs() > 1) {
                // GOTO Diagonal
                self.segments[i].0 += row_diff.signum();
                self.segments[i].1 += col_diff.signum();
            }
        }
        self.visited.insert(self.segments[self.segments.len() - 1]);
    }
}

fn private_solve_part_1(values: &str) -> String {
    let steps = parse(values);
    let mut snake = Snake::default();

    for (direction, number_of_steps) in steps {
        for _ in 0..number_of_steps {
            snake.make_move(&direction);
        }
    }

    snake.visited.len().to_string()
}

fn private_solve_part_2(values: &str) -> String {
    const SNAKE_LEN: usize = 10;
    let steps = parse(values);
    let mut big_snake = BigSnake::new(SNAKE_LEN);

    for (direction, number_of_steps) in steps {
        for _ in 0..number_of_steps {
            big_snake.make_move(&direction);
        }
    }

    big_snake.visited.len().to_string()
}

fn _solve_part_1_dummy() -> String {
    private_solve_part_1(_DUMMY_INPUT)
}

pub fn solve_part_1_real() -> String {
    private_solve_part_1(REAL_INPUT)
}

fn _solve_part_2_dummy() -> String {
    private_solve_part_2(_DUMMY_INPUT)
}

fn _solve_part_2_dummy_2() -> String {
    private_solve_part_2(_DUMMY_INPUT2)
}

pub fn solve_part_2_real() -> String {
    private_solve_part_2(REAL_INPUT)
}

#[cfg(test)]
mod tests {
    use super::{
        _solve_part_1_dummy, _solve_part_2_dummy, _solve_part_2_dummy_2, solve_part_1_real,
        solve_part_2_real,
    };

    #[test]
    fn test_part_1_dummy() {
        assert_eq!("13", _solve_part_1_dummy());
    }
    #[test]
    fn test_part_2_dummy() {
        assert_eq!("1", _solve_part_2_dummy());
    }
    #[test]
    fn test_part_2_dummy2() {
        assert_eq!("36", _solve_part_2_dummy_2());
    }

    #[test]
    fn test_part_1_real() {
        println!("{}", solve_part_1_real()); // 5779
    }
    #[test]
    fn test_part_2_real() {
        println!("{}", solve_part_2_real()); // 2331
    }
}
