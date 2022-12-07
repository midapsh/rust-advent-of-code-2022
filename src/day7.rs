use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

const _DUMMY_INPUT: &str = include_str!("data/day7-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day7-real.txt");

struct Directory {
    _name: String,
    size: RefCell<usize>,
    parent: Option<Rc<Directory>>,
    sub_dir: RefCell<BTreeMap<String, Rc<Directory>>>,
}

impl Directory {
    fn new(
        _name: String,
        size: RefCell<usize>,
        parent: Option<Rc<Directory>>,
        sub_dir: RefCell<BTreeMap<String, Rc<Directory>>>,
    ) -> Self {
        Self {
            _name,
            size,
            parent,
            sub_dir,
        }
    }
}

impl Default for Directory {
    fn default() -> Self {
        Self {
            _name: Default::default(),
            size: RefCell::new(0_usize),
            parent: None,
            sub_dir: Default::default(),
        }
    }
}

impl Directory {
    fn get_size(&self) -> usize {
        *self.size.borrow()
            + self
                .sub_dir
                .borrow()
                .values()
                .fold(0_usize, |cur_size, directory| {
                    cur_size + directory.get_size()
                })
    }
}

fn parse(values: &str) -> Rc<Directory> {
    let root: Rc<Directory> = Rc::from(Directory::default());
    let mut cwd = Rc::clone(&root);

    for line in values.lines() {
        match line.split_ascii_whitespace().collect::<Vec<&str>>()[..] {
            ["$", "ls"] => {}
            ["$", "cd", directory_name] => match directory_name {
                "/" => cwd = Rc::clone(&root),
                ".." => cwd = Rc::clone(&cwd.parent.as_ref().unwrap()),
                directory_name => {
                    let new_directory = cwd.sub_dir.borrow().get(directory_name).unwrap().clone();
                    cwd = new_directory;
                }
            },
            ["dir", directory_name] => {
                cwd.sub_dir.borrow_mut().insert(
                    directory_name.to_string(),
                    Rc::new(Directory::new(
                        directory_name.to_string(),
                        RefCell::new(0_usize),
                        Some(Rc::clone(&cwd)),
                        RefCell::new(BTreeMap::new()),
                    )),
                );
            }
            [file_size, _file_name] => {
                let file_size = file_size.parse::<usize>().unwrap();
                *cwd.size.borrow_mut() += file_size;
            }
            _ => {}
        }
    }

    root.clone()
}

fn private_solve_part_1(values: &str) -> String {
    let root = parse(values);
    let mut to_visit = vec![Rc::clone(&root)];
    let mut total_value: usize = 0;

    while let Some(dir) = to_visit.pop() {
        for sub_dir in dir.sub_dir.borrow().values() {
            to_visit.push(Rc::clone(sub_dir));
        }

        let size = dir.get_size();
        if size <= 100_000 {
            total_value += size;
        }
    }

    total_value.to_string()
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
        assert_eq!("95437", _solve_part_1_dummy());
    }
    #[test]
    fn test_part_2_dummy() {
        assert_eq!("", _solve_part_2_dummy());
    }
    #[test]
    fn test_part_1_real() {
        println!("{}", solve_part_1_real()); // 1513699
    }
    #[test]
    fn test_part_2_real() {
        println!("{}", solve_part_2_real());
    }
}
