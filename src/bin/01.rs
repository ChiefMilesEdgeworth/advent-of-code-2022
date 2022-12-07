fn list_of_sums(s: &str) -> impl Iterator<Item = u32> + '_ {
    s.split("\n\n")
        .map(|ns| ns.lines().filter_map(|n| n.parse::<u32>().ok()))
        .map(Iterator::sum::<u32>)
}

pub fn part_one(input: &str) -> Option<u32> {
    list_of_sums(input).max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sums: Vec<u32> = list_of_sums(input).collect();
    sums.sort_unstable();
    Some(sums.into_iter().rev().take(3).sum())
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
