use regex::Regex;

const _DUMMY_INPUT: &str = include_str!("data/day11-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day11-real.txt");

macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

struct Monkey {
    id: usize,
    items: Vec<i32>,
    operation: Box<dyn FnMut(i32) -> (usize, i32)>,
}

impl Monkey {
    fn new(id: usize, items: Vec<i32>, operation: Box<dyn FnMut(i32) -> (usize, i32)>) -> Self {
        Self {
            id,
            items,
            operation,
        }
    }

    fn parse_operation(
        raw_operation: &str,
        test_divider: i32,
        case_true_monkey_id: usize,
        case_false_monkey_id: usize,
    ) -> Box<dyn FnMut(i32) -> (usize, i32)> {
        let value = 1;
        if raw_operation.contains("*") {
            if raw_operation.matches("old").count() == 2 {
                let a = |old: i32| {old * old};
            } else {
                let a = |old: i32| {value * old};
            }

        } else { // else if raw_operation.contains("+") {
            if raw_operation.matches("old").count() == 2 {
                let a = |old: i32| {old + old};
            } else {
                let a = |old: i32| {value + old};
            }
        }
        unimplemented!()
    }

    fn add_item(&mut self, new: i32) {
        self.items.push(new);
    }

    fn run_operation(&mut self) -> Option<(usize, i32)> {
        if let Some(item) = self.items.pop() {
            Some((self.operation)(item))
        } else {
            None
        }
    }
}

fn private_solve_part_1(values: &str) -> String {
    let re_monkey_id: &Regex = regex!(r##"^Monkey (?P<monkey_id>\d*)"##); // Use find_one
    let re_starting_items: &Regex = regex!(r##"\d+"##); // Use match to get all
    let re_operation: &Regex = regex!(r##""##); // Use find_one
    let re_test_divider: &Regex = regex!(r##"\d+"##); // Use find_one
    let re_first_test_phrase: &Regex =
        regex!(r##"If true: throw to monkey (?P<other_monkey_id>\d*)"##);
    let re_second_test_phrase: &Regex =
        regex!(r##"If false: throw to monkey (?P<other_monkey_id>\d*)"##);

    let monkey_list: Vec<Monkey> = vec![];

    values.split("\n\n").map(|chunk| {
        let mut iter_chunk = chunk.split("\n");

        let str_monkey_id = iter_chunk.next().unwrap().trim();
        let str_starting_items = iter_chunk.next().unwrap().trim();
        let str_operation = iter_chunk.next().unwrap().trim();
        let str_test_divider = iter_chunk.next().unwrap().trim();
        let str_first_test_phrase = iter_chunk.next().unwrap().trim();
        let str_second_test_phrase = iter_chunk.next().unwrap().trim();

        let monkey_id = re_monkey_id.shortest_match(str_monkey_id);
        let starting_items = re_starting_items
            .find_iter(str_starting_items)
            .filter_map(|digits| digits.as_str().parse::<i32>().ok())
            .collect::<Vec<i32>>();
        let raw_operation = re_operation.shortest_match(str_operation);
        let test_divider = re_test_divider.shortest_match(str_test_divider);
        let case_true_monkey_id = re_first_test_phrase.shortest_match(str_first_test_phrase);
        let case_false_monkey_id = re_second_test_phrase.shortest_match(str_second_test_phrase);
    });
    unimplemented!()
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
        assert_eq!("10605", _solve_part_1_dummy());
    }
    #[test]
    fn test_part_2_dummy() {
        assert_eq!("", _solve_part_2_dummy());
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
