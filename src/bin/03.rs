use itertools::Itertools;
use std::cmp::Ordering::*;

type Rucksack<'a> = (&'a str, &'a str);
type Group<'a> = (&'a str, &'a str, &'a str);

fn rucksacks(input: &str) -> impl Iterator<Item = Rucksack> {
    input
        .split_ascii_whitespace()
        .map(|line| line.split_at(line.len() / 2))
}

fn groups(input: &str) -> impl Iterator<Item = Group> {
    input.split_ascii_whitespace().tuples()
}

fn find_extra_item((left, right): Rucksack) -> char {
    let (mut left, mut right) = (
        left.chars().sorted_unstable(),
        right.chars().sorted_unstable(),
    );
    let (mut a, mut b) = (left.next().unwrap(), right.next().unwrap());
    loop {
        match a.cmp(&b) {
            std::cmp::Ordering::Less => a = left.next().unwrap(),
            std::cmp::Ordering::Greater => b = right.next().unwrap(),
            std::cmp::Ordering::Equal => return a,
        }
    }
}

fn find_badge((a, b, c): Group) -> char {
    let (mut first, mut second, mut third) = (
        a.chars().sorted_unstable(),
        b.chars().sorted_unstable(),
        c.chars().sorted_unstable(),
    );
    let (mut a, mut b, mut c) = (
        first.next().unwrap(),
        second.next().unwrap(),
        third.next().unwrap(),
    );
    loop {
        match (a.cmp(&b), b.cmp(&c)) {
            (Less, _) => a = first.next().unwrap(),
            (Greater, _) | (Equal, Less) => b = second.next().unwrap(),
            (Equal, Greater) => c = third.next().unwrap(),
            (Equal, Equal) => return a,
        }
    }
}

fn priority(c: char) -> u8 {
    match c {
        'a'..='z' => c as u8 - b'a' + 1,
        'A'..='Z' => c as u8 - b'A' + 27,
        _ => unreachable!(),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        rucksacks(input)
            .map(find_extra_item)
            .map(priority)
            .map_into::<u32>()
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        groups(input)
            .map(find_badge)
            .map(priority)
            .map_into::<u32>()
            .sum(),
    )
}

fn main() {
    let input = &aoc::read_file("inputs", 3);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
