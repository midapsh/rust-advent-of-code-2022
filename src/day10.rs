const _DUMMY_INPUT: &str = include_str!("data/day10-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day10-real.txt");

fn cmd_noop(x: &mut i32, count_cycles: &mut i32) -> i32 {
    match count_cycles {
        19 | 59 | 99 | 139 | 179 | 219 => {
            *count_cycles += 1;
            let result = (*x) * (*count_cycles);
            // println!("x ({}) result @ {}th cycle: {} with cmd noop", *x, *count_cycles, result);
            result
        }
        _ => {
            *count_cycles += 1;
            0
        }
    }
}

fn cmd_addx(x: &mut i32, add_value: i32, count_cycles: &mut i32) -> i32 {
    match count_cycles {
        18 | 58 | 98 | 138 | 178 | 218 => {
            *count_cycles += 2;
            let result = (*x) * (*count_cycles);
            *x += add_value;
            // println!("x ({}) result @ {}th cycle: {} with cmd addx1 {}", *x, *count_cycles, result, add_value);
            result
        }
        19 | 59 | 99 | 139 | 179 | 219 => {
            let result = (*x) * (*count_cycles + 1);
            // println!("x ({}) result @ {}th cycle: {} with cmd addx2 {}", *x, *count_cycles+1, result, add_value);
            *x += add_value;
            *count_cycles += 2;
            result
        }
        _ => {
            *count_cycles += 2;
            *x += add_value;
            0
        }
    }
}

fn private_solve_part_1(values: &str) -> String {
    let mut count_cycles: i32 = 0;
    let mut x = 1;

    values
        .lines()
        .map(|line| {
            let mut iter = line.split_ascii_whitespace();
            let command = iter.next().unwrap();
            match command {
                "noop" => cmd_noop(&mut x, &mut count_cycles),
                "addx" => {
                    let add_value = iter.next().unwrap().parse::<i32>().unwrap();
                    cmd_addx(&mut x, add_value, &mut count_cycles)
                }
                _ => 0,
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
        assert_eq!("13140", _solve_part_1_dummy());
    }
    #[test]
    fn test_part_2_dummy() {
        assert_eq!("", _solve_part_2_dummy());
    }
    #[test]
    fn test_part_1_real() {
        println!("{}", solve_part_1_real()); // 14160
    }
    #[test]
    fn test_part_2_real() {
        println!("{}", solve_part_2_real());
    }
}
