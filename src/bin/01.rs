fn lists(s: &str) -> Vec<Vec<u32>> {
    s.split("\n\n")
        .map(|ns| {
            ns.split('\n')
                .filter(|n| !n.is_empty())
                .map(|n| n.trim().parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let elves = lists(input);
    elves.into_iter().map(|elf| elf.into_iter().sum()).max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let elves = lists(input);
    let mut sums = elves
        .into_iter()
        .map(|elf| elf.into_iter().sum::<u32>())
        .collect::<Vec<_>>();
    sums.sort_unstable_by(|a, b| b.cmp(a));
    Some(sums.into_iter().take(3).sum())
}

fn main() {
    let input = &aoc::read_file("inputs", 1);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
