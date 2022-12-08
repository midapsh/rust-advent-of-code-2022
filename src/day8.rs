const _DUMMY_INPUT: &str = include_str!("data/day8-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day8-real.txt");

/// This function only works for number from 0 to 9 in char
/// PANICS when it's outside the 0 to 9 range
fn parse_byte_to_u8_number(b: u8) -> u8 {
    match b {
        b'0'..=b'9' => (b - b'0') as u8,
        _ => panic!("Not a valid value"),
    }
}

/// UNSAFE: I don't check if position is in boundaries with matrix
fn is_not_visible(matrix: &Vec<Vec<(u8, bool)>>, cell_value: u8, position: (usize, usize)) -> bool {
    let (i_row, i_col) = position;

    let can_see_on_left = matrix[i_row][..i_col].iter().all(|(c, _)| *c < cell_value);
    let can_see_on_right = matrix[i_row][i_col + 1..]
        .iter()
        .all(|(c, _)| *c < cell_value);

    let can_see_on_top = matrix[..i_row]
        .iter()
        .map(|row| row[i_col])
        .all(|(c, _)| c < cell_value);
    let can_see_on_bottom = matrix[i_row + 1..]
        .iter()
        .map(|row| row[i_col])
        .all(|(c, _)| c < cell_value);

    return !(can_see_on_left | can_see_on_right | can_see_on_top | can_see_on_bottom);
}

fn print_matrix_result(matrix: &Vec<Vec<(u8, bool)>>) {
    let matrix_result = matrix
        .iter()
        .map(|row| {
            row.iter()
                .map(|(cell_value, cell_is_visible)| {
                    if *cell_is_visible {
                        'X'
                    } else {
                        (cell_value + b'0') as char
                    }
                })
                .collect::<String>()
                + "\n"
        })
        .collect::<String>();

    println!("{}", matrix_result);
}

fn private_solve_part_1(values: &str) -> String {
    let mut matrix = values
        .lines()
        .map(|x| {
            x.bytes()
                .map(|c| (parse_byte_to_u8_number(c), true))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let row_len = matrix.len();
    let col_len = matrix.first().unwrap().len();
    let aux_matrix = matrix.clone();

    for (i_row, row) in matrix[1..row_len - 1].iter_mut().enumerate() {
        for (i_col, cell) in row[1..col_len - 1].iter_mut().enumerate() {
            if is_not_visible(&aux_matrix, cell.0, (i_row + 1, i_col + 1)) {
                cell.1 = false;
            }
        }
    }

    // print_matrix_result(&matrix);

    matrix
        .iter()
        .map(|row| {
            row.iter()
                .filter(|(_, cell_is_visible)| *cell_is_visible)
                .count()
        })
        .sum::<usize>()
        .to_string()
}

fn private_solve_part_2(_values: &str) -> String {
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
        assert_eq!("21", _solve_part_1_dummy());
    }
    #[test]
    fn test_part_2_dummy() {
        assert_eq!("", _solve_part_2_dummy());
    }
    #[test]
    fn test_part_1_real() {
        println!("{}", solve_part_1_real()); // 1801
    }
    #[test]
    fn test_part_2_real() {
        println!("{}", solve_part_2_real());
    }
}
