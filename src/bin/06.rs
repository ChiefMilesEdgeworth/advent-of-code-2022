use itertools::Itertools;
use std::collections::HashSet;

fn no_duplicates(window: &[u8], size: usize) -> bool {
    let n = window
        .iter()
        .fold(0u32, |c, b| c | 1 << b - b'a')
        .count_ones();
    n == size as u32
}

fn message_start(input: &str, size: usize) -> Option<usize> {
    input
        .as_bytes()
        .windows(size)
        .position(|w| no_duplicates(w, size))
        .map(|x| x + size)
}

fn alt_message_start(input: &str, size: usize) -> Option<usize> {
    let masks: Vec<u32> = input
        .trim()
        .as_bytes()
        .into_iter()
        .map(|b| 1 << (b - b'a'))
        .collect();
    let mut accum = 0u32;
    for mask in &masks[..size] {
        accum ^= mask;
    }
    for (i, (lower, upper)) in masks.iter().zip(masks.iter().skip(size)).enumerate() {
        accum ^= lower;
        accum ^= upper;
        if accum.count_ones() as usize == size {
            return Some(i + 1 + size);
        }
    }
    unreachable!()
}

pub fn part_one(input: &str) -> Option<usize> {
    alt_message_start(input, 4)
}

pub fn part_two(input: &str) -> Option<usize> {
    alt_message_start(input, 14)
}

fn main() {
    let input = &aoc::read_file("inputs", 6);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let inputs = [
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        ];
        assert_eq!(
            inputs.map(part_one),
            [Some(7), Some(5), Some(6), Some(10), Some(11)]
        );
    }

    #[test]
    fn test_part_two() {
        let inputs = [
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        ];
        assert_eq!(
            inputs.map(part_two),
            [Some(19), Some(23), Some(23), Some(29), Some(26)]
        );
    }
}
