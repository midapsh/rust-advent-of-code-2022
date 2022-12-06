const _DUMMY_INPUT: &str = include_str!("data/day6-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day6-real.txt");

fn private_solve_part_1(values: &str) -> String {
    let line_chars = values.lines().next().unwrap().chars();

    let mut start_of_packet_marker: Vec<char> = vec![];

    for (pos, letter) in line_chars.enumerate() {
        match start_of_packet_marker.iter().position(|&c| c == letter) {
            Some(index) => {
                start_of_packet_marker.drain(..=index);
                start_of_packet_marker.push(letter);
            }
            None => {
                start_of_packet_marker.push(letter);
                if start_of_packet_marker.len() == 4 {
                    return (pos + 1).to_string();
                }
            }
        }
    }

    0.to_string()
}

fn private_solve_part_2(values: &str) -> String {
    let line_chars = values.lines().next().unwrap().chars();

    let mut start_of_packet_marker: Vec<char> = vec![];

    for (pos, letter) in line_chars.enumerate() {
        match start_of_packet_marker.iter().position(|&c| c == letter) {
            Some(index) => {
                start_of_packet_marker.drain(..=index);
                start_of_packet_marker.push(letter);
            }
            None => {
                start_of_packet_marker.push(letter);
                if start_of_packet_marker.len() == 14 {
                    return (pos + 1).to_string();
                }
            }
        }
    }

    0.to_string()
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
        assert_eq!("7", _solve_part_1_dummy());
    }
    #[test]
    fn test_part_2_dummy() {
        assert_eq!("19", _solve_part_2_dummy());
    }
    #[test]
    fn test_part_1_real() {
        println!("{}", solve_part_1_real()); // 1953
    }
    #[test]
    fn test_part_2_real() {
        println!("{}", solve_part_2_real()); // 2301
    }
}
