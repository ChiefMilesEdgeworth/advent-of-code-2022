use anyhow::Error;
use hashbrown::hash_map::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct File {
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
            let (size, _) = s.split_once(' ').unwrap();
            Ok(Line::File(File {
                size: size.parse().unwrap(),
            }))
        }
    }
}

#[derive(Debug, Default)]
struct DirectoryTree {
    subdirs: HashMap<String, DirectoryTree>,
    files: Vec<File>,
    size: u32,
}

impl DirectoryTree {
    fn populate_sizes(&mut self) {
        for dir in self.subdirs.values_mut() {
            dir.populate_sizes()
        }
        self.size = self.files.iter().map(|file| file.size).sum::<u32>()
            + self.subdirs.values().map(|subdir| subdir.size).sum::<u32>()
    }
}

fn part1_small_dir_total(root: &DirectoryTree) -> u32 {
    let subdir_total: u32 = root.subdirs.values().map(part1_small_dir_total).sum();
    subdir_total + if root.size <= 100_000 { root.size } else { 0 }
}

fn part2_delete_smallest(root: DirectoryTree) -> u32 {
    let needed_space = 30_000_000 - (70_000_000 - root.size);
    let mut dirs = Vec::from_iter(root.subdirs.values());
    let mut min = 70_000_000;
    while let Some(dir) = dirs.pop() {
        if dir.size < min && dir.size >= needed_space {
            min = dir.size;
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
        size: 0,
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
    let mut tree = make_tree(lines);
    tree.populate_sizes();
    Some(part1_small_dir_total(&tree))
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().map(|line| line.parse::<Line>().unwrap());
    let mut tree = make_tree(lines);
    tree.populate_sizes();
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
