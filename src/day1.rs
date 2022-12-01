const _DUMMY_INPUT: &str = include_str!("data/day1-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day1-real.txt");

fn private_solve_part_1(values: &str) -> String {
    let mut elfs_callories: Vec<i32> = vec![0];

    for line in values.lines() {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            elfs_callories.push(0);
            continue;
        }
        let food_callories = trimmed_line.parse::<i32>().unwrap();
        if let Some(last_elf_callories) = elfs_callories.last_mut() {
            *last_elf_callories += food_callories;
        }
    }

    elfs_callories.iter().max().unwrap().to_string()
}

fn private_solve_part_2(values: &str) -> String {
    let mut elfs_callories: Vec<i32> = vec![0];

    for line in values.lines() {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            elfs_callories.push(0);
            continue;
        }
        let food_callories = trimmed_line.parse::<i32>().unwrap();
        if let Some(last_elf_callories) = elfs_callories.last_mut() {
            *last_elf_callories += food_callories;
        }
    }
    elfs_callories.sort_unstable();
    elfs_callories.iter().rev().take(3).sum::<i32>().to_string()
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
        assert_eq!("24000", _solve_part_1_dummy());
    }
    #[test]
    fn test_part_2_dummy() {
        assert_eq!("45000", _solve_part_2_dummy());
    }
    #[test]
    fn test_part_1_real() {
        println!("{}", solve_part_1_real());
    }
    #[test]
    fn test_part_2_real() {
        println!("{}", solve_part_2_real());
    }
}
