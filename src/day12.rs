use std::{
    collections::{BTreeMap, BinaryHeap},
    str::FromStr,
};

const _DUMMY_INPUT: &str = include_str!("data/day12-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day12-real.txt");

struct Map {
    map: BTreeMap<(usize, usize), usize>,
    width: usize,
    height: usize,
    src: (usize, usize),
    dest: (usize, usize),
}

#[derive(Debug, PartialEq, Eq)]
struct ParseMapError;

impl Map {
    fn new(
        map: BTreeMap<(usize, usize), usize>,
        width: usize,
        height: usize,
        src: (usize, usize),
        dest: (usize, usize),
    ) -> Self {
        Self {
            map,
            width,
            height,
            src,
            dest,
        }
    }
}

impl FromStr for Map {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();

        let mut map = BTreeMap::new();
        let mut src = (0, 0);
        let mut dest = (0, 0);

        let height = lines.len();
        let width = lines[0].len();

        for x in 0..height {
            for y in 0..width {
                if lines[x][y] == 'S' {
                    src = (x, y);
                    lines[x][y] = 'a';
                } else if lines[x][y] == 'E' {
                    dest = (x, y);
                    lines[x][y] = 'z';
                }

                map.insert((x, y), lines[x][y] as usize - 'a' as usize);
            }
        }

        Ok(Self::new(map, width, height, src, dest))
    }
}

fn neighbors(map: &Map, node: (usize, usize)) -> Vec<(usize, usize)> {
    let edges = &map.map;
    let height = map.height as isize;
    let width = map.width as isize;

    let value = edges.get(&node).unwrap();

    let (x, y) = node;

    [(0, 1), (0, -1), (-1, 0), (1, 0)]
        .into_iter()
        .filter_map(|(i, j)| {
            let x_n = x as isize + i;
            let y_n = y as isize + j;

            // Note(hspadim): This is a saturation function
            // to keep things inside the map boundary
            if x_n < 0 || x_n >= height || y_n < 0 || y_n >= width {
                return None;
            }

            let x_n = x_n as usize;
            let y_n = y_n as usize;

            let value_n = edges.get(&(x_n, y_n)).unwrap();

            if value + 1 >= *value_n {
                Some((x_n, y_n))
            } else {
                None
            }
        })
        .collect()
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

impl State {
    fn new(cost: usize, position: (usize, usize)) -> Self {
        Self { cost, position }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(map: &Map) -> Option<usize> {
    let start = map.src;
    let goal = map.dest;
    let edges = &map.map;

    let mut dist: BTreeMap<_, _> = edges.keys().map(|&k| (k, usize::MAX)).collect();

    let mut heap = BinaryHeap::new();

    dist.insert(start, 0);
    heap.push(State::new(0, start));

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }

        if cost > *dist.get(&position).unwrap() {
            continue;
        }

        for edge in neighbors(map, position) {
            if cost + 1 < *dist.get(&edge).unwrap() {
                heap.push(State::new(cost + 1, edge));
                dist.insert(edge, cost + 1);
            }
        }
    }
    return None;
}

fn private_solve_part_1(values: &str) -> String {
    let map = Map::from_str(values).unwrap();

    shortest_path(&map).unwrap().to_string()
}

fn private_solve_part_2(values: &str) -> String {
    let map = Map::from_str(values).unwrap();

    map.map
        .iter()
        .filter(|(_, &v)| return v == 0)
        .filter_map(|(&k, _)| {
            let test_input = Map::new(map.map.clone(), map.width, map.height, k, map.dest);

            return shortest_path(&test_input);
        })
        .min()
        .unwrap()
        .to_string()
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

pub fn solve_part_2_real() -> String {
    private_solve_part_2(REAL_INPUT)
}

#[cfg(test)]
mod tests {
    use super::{_solve_part_1_dummy, _solve_part_2_dummy, solve_part_1_real, solve_part_2_real};

    #[test]
    fn test_part_1_dummy() {
        assert_eq!("31", _solve_part_1_dummy());
    }
    #[test]
    fn test_part_2_dummy() {
        assert_eq!("29", _solve_part_2_dummy());
    }
    #[test]
    fn test_part_1_real() {
        println!("{}", solve_part_1_real()); // 504
    }
    #[test]
    fn test_part_2_real() {
        println!("{}", solve_part_2_real()); // 500
    }
}
