use anyhow::{anyhow, Error};
use itertools::Itertools;
use std::{collections::HashMap, hash::Hash, str::FromStr};

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: u32,
}

#[derive(Debug, Clone)]
enum Command {
    List,
    CD(String),
}

#[derive(Debug, Clone)]
enum Line {
    Command(Command),
    Directory(String),
    File(File),
}

impl FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(cmd) = s.strip_prefix("$ ") {
            if let Some(dir) = cmd.trim().strip_prefix("cd ") {
                Ok(Self::Command(Command::CD(dir.to_string())))
            } else {
                Ok(Self::Command(Command::List))
            }
        } else if let Some(dir) = s.strip_prefix("dir ") {
            Ok(Line::Directory(dir.to_string()))
        } else {
            let (size, name) = s.split_once(' ').unwrap();
            Ok(Line::File(File {
                name: name.to_string(),
                size: size.parse().unwrap(),
            }))
        }
    }
}

#[derive(Debug, Default)]
struct DirectoryTree {
    subdirs: HashMap<String, DirectoryTree>,
    files: Vec<File>,
}

impl DirectoryTree {
    fn total_size(&self) -> u32 {
        let subdirs_sum: u32 = self
            .subdirs
            .values()
            .map(|subtree| subtree.total_size())
            .sum();
        let file_sum: u32 = self.files.iter().map(|file| file.size).sum();
        subdirs_sum + file_sum
    }
}

fn part1_small_dir_total(root: DirectoryTree) -> u32 {
    let mut dirs = Vec::with_capacity(300);
    let mut accum = 0;
    dirs.push(&root);
    while let Some(dir) = dirs.pop() {
        let size = dir.total_size();
        if size <= 100_000 {
            accum += size;
        }
        for subdir in dir.subdirs.values() {
            dirs.push(subdir);
        }
    }
    accum
}

fn part2_delete_smallest(root: DirectoryTree) -> u32 {
    let needed_space = 30_000_000 - (70_000_000 - root.total_size());
    let mut dirs = Vec::from_iter(root.subdirs.values());
    let mut min = 70_000_000;
    while let Some(dir) = dirs.pop() {
        let size = dir.total_size();
        if size < min && size >= needed_space {
            min = size;
        }
        dirs.extend(dir.subdirs.values());
    }
    min
}

fn move_working_tree<'a>(tree: &'a mut DirectoryTree, path: &'_ [String]) -> &'a mut DirectoryTree {
    let mut working_tree = tree;
    for part in path.iter().skip(1) {
        // Skip the root
        working_tree = working_tree.subdirs.get_mut(part).unwrap();
    }
    working_tree
}

fn make_tree(info: impl Iterator<Item = Line>) -> DirectoryTree {
    let mut path = vec![];
    let mut tree = DirectoryTree {
        subdirs: HashMap::new(),
        files: vec![],
    };
    let mut working_tree = &mut tree;
    for line in info {
        match line {
            Line::Command(Command::List) => (), // Do nothing, we'll handle the dirs and files further down.
            Line::Command(Command::CD(dir)) => {
                if &dir == ".." {
                    path.pop().unwrap();
                } else {
                    path.push(dir);
                }
                working_tree = move_working_tree(&mut tree, &path);
            }
            Line::Directory(dir) => {
                working_tree.subdirs.insert(dir, Default::default());
            }
            Line::File(file) => working_tree.files.push(file),
        }
    }
    tree
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().map(|line| line.parse::<Line>().unwrap());
    let tree = make_tree(lines);
    Some(part1_small_dir_total(tree))
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().map(|line| line.parse::<Line>().unwrap());
    let tree = make_tree(lines);
    Some(part2_delete_smallest(tree))
}

fn main() {
    let input = &aoc::read_file("inputs", 7);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
