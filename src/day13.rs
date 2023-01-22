use std::cmp::Ordering;
use std::str::Chars;

const _DUMMY_INPUT: &str = include_str!("data/day13-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day13-real.txt");

#[derive(Debug)]
enum Val {
    Num(i32),
    List(Vec<Self>),
}

impl Val {
    fn parse(value: &str) -> Self {
        let mut c = value.chars();
        if c.next().unwrap() != '[' {
            panic!("Bad input");
        }
        Self::parse_into(&mut c)
    }

    fn parse_into(c: &mut Chars) -> Self {
        let mut result = vec![];
        let mut num: i32 = -1;

        while let Some(ch) = c.next() {
            match ch {
                '[' => result.push(Self::parse_into(c)),
                ',' => {
                    if !num.is_negative() {
                        result.push(Self::Num(num));
                        num = -1;
                    }
                }
                ']' => {
                    if !num.is_negative() {
                        result.push(Self::Num(num));
                    }
                    return Self::List(result);
                }
                '0'..='9' => {
                    if num.is_negative() {
                        num = (ch as u8 - b'0') as i32;
                    } else {
                        num = (num * 10) + (ch as u8 - b'0') as i32;
                    }
                }
                _ => panic!("Bad char '{ch}'"),
            }
        }
        Self::List(result)
    }

    fn compare(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Val::List(left), Val::List(right)) => {
                let mut idx = 0;
                loop {
                    if left.len() <= idx || right.len() <= idx {
                        if left.len() < right.len() {
                            return Ordering::Less;
                        } else if left.len() == right.len() {
                            return Ordering::Equal;
                        } else {
                            return Ordering::Greater;
                        }
                    }
                    match (&left[idx], &right[idx]) {
                        (Val::Num(l), Val::Num(r)) => {
                            if l < r {
                                return Ordering::Less;
                            } else if l > r {
                                return Ordering::Greater;
                            }
                        }
                        (Val::List(_l), Val::Num(r)) => {
                            let check = left[idx].compare(&Val::List(vec![Val::Num(*r)]));
                            if check != Ordering::Equal {
                                return check;
                            }
                        }
                        (Val::Num(l), Val::List(_r)) => {
                            let check = Val::List(vec![Val::Num(*l)]).compare(&right[idx]);
                            if check != Ordering::Equal {
                                return check;
                            }
                        }
                        // (Val::List(l), Val::List(r)) => {
                        (Val::List(_l), Val::List(_r)) => {
                            let check = left[idx].compare(&right[idx]);
                            if check != Ordering::Equal {
                                return check;
                            }
                        }
                    }
                    idx += 1;
                }
            }
            _ => panic!("Bad input"),
        }
    }
}

fn private_solve_part_1(values: &str) -> String {
    let mut all_lists: Vec<[Val; 2]> = Vec::with_capacity(2000);

    values.split("\n\n").for_each(|chunk| {
        let mut iter_chunk = chunk.split("\n");
        let left_list = Val::parse(iter_chunk.next().unwrap().trim());
        let right_list = Val::parse(iter_chunk.next().unwrap().trim());
        all_lists.push([left_list, right_list]);
    });

    all_lists
        .iter()
        .enumerate()
        .map(|(pos, [left_list, right_list])| {
            if left_list.compare(&right_list) == Ordering::Less {
                pos + 1
            } else {
                0
            }
        })
        .sum::<usize>()
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
        assert_eq!("13", _solve_part_1_dummy());
    }
    #[test]
    fn test_part_2_dummy() {
        assert_eq!("", _solve_part_2_dummy());
    }
    #[test]
    fn test_part_1_real() {
        println!("{}", solve_part_1_real()); // 5208
    }
    #[test]
    fn test_part_2_real() {
        println!("{}", solve_part_2_real());
    }
}
