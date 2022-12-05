use regex::Regex;

const _DUMMY_INPUT: &str = include_str!("data/day5-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day5-real.txt");

macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

fn transpose<T>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if matrix.is_empty() {
        return matrix;
    }

    let mut tranposed_matrix: Vec<Vec<T>> = vec![];
    for vector in matrix.into_iter() {
        for (pos, v) in vector.into_iter().enumerate() {
            // UNSAFE(hspadim): I'm assuming that the vector exists and is accessible.
            // This assumption also accepts that the pos is never going to breach the
            // 'pos' size limit (usize)
            if tranposed_matrix.len() == pos {
                tranposed_matrix.push(vec![v]);
            } else {
                tranposed_matrix[pos].push(v);
            }
        }
    }
    tranposed_matrix
}

fn parse_initial_state(raw_initial_state: &str) -> Vec<Vec<char>> {
    let start_at = 2 - 1;
    let step = 4;
    let transposed_initial_state: Vec<Vec<char>> = raw_initial_state
        .lines()
        .map(|line| {
            line.chars()
                .skip(start_at)
                .step_by(step)
                .filter(|c| c.is_ascii_alphabetic() | c.is_whitespace())
                .collect::<Vec<char>>()
        })
        .collect();
    transpose(transposed_initial_state)
        .iter()
        .map(|vv| {
            vv.into_iter()
                .rev()
                .filter(|c| !c.is_whitespace())
                .cloned()
                .collect()
        })
        .collect()
}

fn private_solve_part_1(values: &str) -> String {
    let values: Vec<&str> = values.split_terminator("\n\n").take(2).collect();

    // UNSAFE(hspadim): It's clear by the problem statement that we have only 2 parts
    let raw_initial_state = values[0];
    let raw_steps = values[1];

    let mut initial_state = parse_initial_state(raw_initial_state);

    let re: &Regex = regex!(
        r##"^move (?P<number_of_elements>\d*) from (?P<current_index>\d*) to (?P<next_index>\d*)"##,
    );
    raw_steps.lines().for_each(|raw_instruction| {
        let cap = re.captures(raw_instruction).unwrap();
        let number_of_elements = cap
            .name("number_of_elements")
            .map_or(0_usize, |m| m.as_str().parse::<usize>().unwrap());
        // UNSAFE(hspadim): Here I'm sure that the number (index in the puzzle context)
        // is always greater or equal to 1
        let current_index = cap
            .name("current_index")
            .map_or(0_usize, |m| m.as_str().parse::<usize>().unwrap() - 1);
        // UNSAFE(hspadim): Here I'm sure that the number(index in the puzzle context)
        // is always greater or equal to 1
        let next_index = cap
            .name("next_index")
            .map_or(0_usize, |m| m.as_str().parse::<usize>().unwrap() - 1);

        for _ in 0..number_of_elements {
            let item = initial_state[current_index].pop().unwrap();
            initial_state[next_index].push(item);
        }
    });

    initial_state
        .iter()
        .map(|v| v.last().unwrap())
        .collect::<String>()
}

fn private_solve_part_2(values: &str) -> String {
    let values: Vec<&str> = values.split_terminator("\n\n").take(2).collect();

    // UNSAFE(hspadim): It's clear by the problem statement that we have only 2 parts
    let raw_initial_state = values[0];
    let raw_steps = values[1];

    let mut initial_state = parse_initial_state(raw_initial_state);

    let re: &Regex = regex!(
        r##"^move (?P<number_of_elements>\d*) from (?P<current_index>\d*) to (?P<next_index>\d*)"##,
    );
    raw_steps.lines().for_each(|raw_instruction| {
        let cap = re.captures(raw_instruction).unwrap();
        let number_of_elements = cap
            .name("number_of_elements")
            .map_or(0_usize, |m| m.as_str().parse::<usize>().unwrap());
        // UNSAFE(hspadim): Here I'm sure that the number (index in the puzzle context)
        // is always greater or equal to 1
        let current_index = cap
            .name("current_index")
            .map_or(0_usize, |m| m.as_str().parse::<usize>().unwrap() - 1);
        // UNSAFE(hspadim): Here I'm sure that the number(index in the puzzle context)
        // is always greater or equal to 1
        let next_index = cap
            .name("next_index")
            .map_or(0_usize, |m| m.as_str().parse::<usize>().unwrap() - 1);

        let ref_vec = &mut initial_state[current_index];
        let start_value = ref_vec.len()-number_of_elements;
        let mut aux = ref_vec.drain(start_value..).collect();
        initial_state[next_index].append(&mut aux);
    });

    initial_state
        .iter()
        .map(|v| v.last().unwrap())
        .collect::<String>()
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
        assert_eq!("CMZ", _solve_part_1_dummy());
    }
    #[test]
    fn test_part_2_dummy() {
        assert_eq!("MCD", _solve_part_2_dummy());
    }
    #[test]
    fn test_part_1_real() {
        println!("{}", solve_part_1_real()); // HBTMTBSDC
    }
    #[test]
    fn test_part_2_real() {
        println!("{}", solve_part_2_real()); // PQTJRSHWS
    }
}
