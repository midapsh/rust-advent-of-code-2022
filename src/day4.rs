const _DUMMY_INPUT: &str = include_str!("data/day4-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day4-real.txt");

fn private_solve_part_1(values: &str) -> String {
    values
        .lines()
        .map(|line| {
            let (range1, range2) = line.split_once(',').unwrap();

            let (start1, end1) = range1.split_once('-').unwrap();
            let (start2, end2) = range2.split_once('-').unwrap();

            let (start1, end1) = (start1.parse::<i32>().unwrap(), end1.parse::<i32>().unwrap());
            let (start2, end2) = (start2.parse::<i32>().unwrap(), end2.parse::<i32>().unwrap());

            if ((start1 <= start2) & (end2 <= end1)) | ((start2 <= start1) & (end1 <= end2)) {
                1
            } else {
                0
            }
        })
        .sum::<i32>()
        .to_string()
}

fn private_solve_part_2(values: &str) -> String {
    unimplemented!()
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
        assert_eq!("2", _solve_part_1_dummy());
    }
    #[test]
    fn test_part_2_dummy() {
        assert_eq!("", _solve_part_2_dummy());
    }
    #[test]
    fn test_part_1_real() {
        println!("{}", solve_part_1_real()); // 471
    }
    #[test]
    fn test_part_2_real() {
        println!("{}", solve_part_2_real());
    }
}
