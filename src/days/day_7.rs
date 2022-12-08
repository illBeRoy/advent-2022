use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

use crate::day::Day;
use crate::input::read_input;

const INPUT_FILE: &str = "day7.txt";

#[derive(Clone, Copy)]
pub struct Day7 {}

impl Day for Day7 {
    fn title(&self) -> &'static str {
        "No Space Left On Device"
    }

    fn description(&self) -> &'static str {
        "
        That was a HARD one. Not because the puzzle itself was hard, but because rust just HATES trees.

        Simply put, I wanted to build a two-way tree of the file system's directories. Given rust's approach
        to both mutability and ownership, creating the same node and referencing it from multiple places was near
        impossible to handle.

        The second iteration was to actually use strings instead: the tree struct would hold all nodes
        in a flat HashMap, and they would reference each other by string.
        
        This worked! And actually allowed me to efficiently run through the tree both for building it and for
        scanning it, looking for directories that fulfill the size constraints.

        In addition, each line of the input is translated into a well structured command or list result type.

        The tasks themselves do not differ so much.
        
        In the first, we iterate over the resulting tree, filter all dirs under 100kb, and sum.

        In the second, we iterate over the resulting tree, filter all dirs that are large enough to enable us
        to install the update, and then return the smallest one out of those.
        "
    }

    fn task_1(&self) -> String {
        let input = read_input(INPUT_FILE);
        let dir_tree = parse_input_into_dir_tree(&input);

        let dirs_under_100000 = dir_tree
            .dirs
            .iter()
            .map(|(_, dir)| dir)
            .filter(|dir| dir.size.le(&100_000))
            .collect_vec();

        format!(
            "there are {} dirs sized under 100000, with total size of {}",
            dirs_under_100000.len(),
            dirs_under_100000.iter().map(|dir| dir.size).sum::<u64>(),
        )
    }

    fn task_2(&self) -> String {
        let input = read_input(INPUT_FILE);
        let dir_tree = parse_input_into_dir_tree(&input);

        const TOTAL_DISK_SIZE: u64 = 70_000_000;
        const REQUIRED_DISK_SIZE: u64 = 30_000_000;

        let total_taken_size = dir_tree.get(&"/".to_string()).unwrap().size;
        let disk_space_to_free = total_taken_size - (TOTAL_DISK_SIZE - REQUIRED_DISK_SIZE);

        let possible_dirs_to_delete = dir_tree
            .dirs
            .iter()
            .map(|(_, dir)| dir)
            .filter(|dir| dir.size >= disk_space_to_free);

        let dir_to_delete = possible_dirs_to_delete
            .min_by(|a, b| a.size.cmp(&b.size))
            .expect("really? no dirs?");

        format!("the smallest dir to delete that will yield us enough space for update has total size of {}", dir_to_delete.size)
    }
}

struct DirTree {
    dirs: HashMap<String, Directory>,
}

impl DirTree {
    fn new() -> Self {
        Self {
            dirs: HashMap::from([(
                "/".to_string(),
                Directory {
                    size: 0,
                    sub_dirs: vec![],
                    parent: None,
                },
            )]),
        }
    }

    fn get(&self, path: &String) -> Option<&Directory> {
        self.dirs.get(path)
    }

    fn get_mut(&mut self, path: &String) -> Option<&mut Directory> {
        self.dirs.get_mut(path)
    }

    fn insert_dir(&mut self, dirname: String, parent: String) {
        let path = format!("{}{}/", parent, dirname);

        let dir = Directory {
            size: 0,
            sub_dirs: vec![],
            parent: Some(parent.clone()),
        };

        self.get_mut(&parent)
            .expect(format!("no parent directory at {}", parent).as_str())
            .sub_dirs
            .push(path.clone());

        self.dirs.insert(path, dir);
    }

    fn insert_file(&mut self, size: &u64, path: &String) {
        let mut next_path_to_traverse = Some(path.clone());

        while let Some(cur_path) = next_path_to_traverse {
            let node = self
                .get_mut(&cur_path)
                .expect(format!("path not found: {}", cur_path).as_str());
            node.size += size;
            next_path_to_traverse = node.parent.clone();
        }
    }
}

struct Directory {
    size: u64,
    sub_dirs: Vec<String>,
    parent: Option<String>,
}

#[derive(Debug)]
enum Commands {
    CD(String),
    LS,
}

impl Commands {
    fn from(command: &str) -> Self {
        if command == "$ ls" {
            return Commands::LS;
        }

        let cd_matcher = Regex::new(r"^\$ cd (?P<dirname>.+)$").unwrap();
        if let Some(matched) = cd_matcher.captures(command) {
            let dirname = matched.name("dirname").unwrap().as_str().to_string();
            return Commands::CD(dirname);
        }

        panic!("unknown command: {}", command);
    }
}

enum ListResults {
    Dir(String),
    File(u64),
}

impl ListResults {
    fn from(list_result: &str) -> Self {
        let dir_matcher = Regex::new(r"^dir (?P<dirname>.+)$").unwrap();
        if let Some(matched) = dir_matcher.captures(list_result) {
            return Self::Dir(matched.name("dirname").unwrap().as_str().to_string());
        }

        let file_matcher = Regex::new(r"^(?P<size>\d+) .+").unwrap();
        if let Some(matched) = file_matcher.captures(list_result) {
            let size = matched
                .name("size")
                .unwrap()
                .as_str()
                .parse::<u64>()
                .unwrap();
            return Self::File(size);
        }

        panic!("unknown list result: {}", list_result);
    }
}

fn parse_input_into_dir_tree(input: &String) -> DirTree {
    let mut dir_tree = DirTree::new();
    let mut current_path = "/".to_string();

    let mut lines = input.lines().peekable();
    while let Some(line) = lines.next() {
        let command = Commands::from(line);
        match command {
            Commands::CD(to) => match to.as_str() {
                "/" => current_path = "/".to_string(),
                ".." => {
                    current_path = dir_tree
                        .get(&current_path)
                        .unwrap()
                        .parent
                        .as_ref()
                        .unwrap()
                        .clone();
                }
                into_dir => {
                    current_path = format!("{}{}/", current_path, into_dir);
                }
            },
            Commands::LS => {
                while lines.peek().is_some() && !lines.peek().unwrap().starts_with("$") {
                    let list_result = ListResults::from(lines.next().unwrap());
                    match list_result {
                        ListResults::File(size) => dir_tree.insert_file(&size, &current_path),
                        ListResults::Dir(name) => {
                            dir_tree.insert_dir(name, current_path.clone());
                        }
                    }
                }
            }
        };
    }

    dir_tree
}
