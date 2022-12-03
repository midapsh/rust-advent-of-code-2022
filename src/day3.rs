use std::collections::HashSet;

const _DUMMY_INPUT: &str = include_str!("data/day3-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day3-real.txt");

fn table_points(c: char) -> i32 {
    match c {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => 0,
    }
}

fn private_solve_part_1(values: &str) -> String {
    values
        .lines()
        .map(|line| {
            let (list1, list2) = line.split_at(line.len() / 2);
            // UNSAFE(hspadim): the default Rust char is not ASCII, it is UTF-8. Be aware
            // of unexpected behaviors
            let list1: HashSet<i32> = list1
                .chars()
                .map(|c| table_points(c))
                .filter(|&x| x != 0)
                .collect();
            let list2: HashSet<i32> = list2
                .chars()
                .map(|c| table_points(c))
                .filter(|&x| x != 0)
                .collect();

            // let interset: HashSet<u8> = list1.intersection(&list2).map(|&x| x).collect();
            list1.intersection(&list2).sum::<i32>()
        })
        .sum::<i32>()
        .to_string()
}

fn private_solve_part_2(values: &str) -> String {
    let mut total_points = 0;

    let mut lines = values.lines();
    while let (Some(line1), Some(line2), Some(line3)) = (lines.next(), lines.next(), lines.next()) {
        let list1: HashSet<i32> = line1
            .chars()
            .map(|c| table_points(c))
            .filter(|&x| x != 0)
            .collect();
        let list2: HashSet<i32> = line2
            .chars()
            .map(|c| table_points(c))
            .filter(|&x| x != 0)
            .collect();
        let list3: HashSet<i32> = line3
            .chars()
            .map(|c| table_points(c))
            .filter(|&x| x != 0)
            .collect();

        total_points += list1
            .intersection(&list2)
            .map(|&x| x)
            .collect::<HashSet<i32>>()
            .intersection(&list3)
            .sum::<i32>()
    }
    total_points.to_string()
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
        assert_eq!("157", _solve_part_1_dummy());
    }
    #[test]
    fn test_part_2_dummy() {
        assert_eq!("70", _solve_part_2_dummy());
    }
    #[test]
    fn test_part_1_real() {
        println!("{}", solve_part_1_real()); // 7980
    }
    #[test]
    fn test_part_2_real() {
        println!("{}", solve_part_2_real()); // 2881
    }
}
