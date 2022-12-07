use itertools::Itertools;

type Rucksack<'a> = (&'a str, &'a str);
type Group<'a> = (&'a str, &'a str, &'a str);

fn rucksacks(input: &str) -> impl Iterator<Item = Rucksack> {
    input.lines().map(|line| line.split_at(line.len() / 2))
}

fn groups(input: &str) -> impl Iterator<Item = Group> {
    input.split_ascii_whitespace().tuples()
}

fn bits(line: &str) -> u64 {
    line.bytes()
        .map(priority)
        .fold(0, |bits, bit| bits | 1 << bit)
}

fn find_extra_item((left, right): Rucksack) -> u32 {
    let common = bits(left) & bits(right);
    u64::BITS - common.leading_zeros()
}

fn find_badge((a, b, c): Group) -> u32 {
    let common = bits(a) & bits(b) & bits(c);
    u64::BITS - common.leading_zeros()
}

fn priority(c: u8) -> u8 {
    match c {
        b'a'..=b'z' => c - b'a',
        b'A'..=b'Z' => c - b'A' + 26,
        _ => unreachable!(),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(rucksacks(input).map(find_extra_item).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(groups(input).map(find_badge).sum())
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
