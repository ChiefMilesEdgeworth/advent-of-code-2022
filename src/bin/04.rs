use itertools::Itertools;

fn is_fully_contained(elf: &&str) -> bool {
    let (ll, lr, rl, rr) = elf
        .replace('-', ",")
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .tuples()
        .next()
        .expect("There must be exactly four elements");

    (rr >= lr && rl <= ll) || ll <= rl && lr >= rr
}

fn is_overlapping(elf: &&str) -> bool {
    let (ll, lr, rl, rr) = elf
        .replace('-', ",")
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .tuples()
        .next()
        .expect("There must be exactly four elements");

    (ll >= rl && ll <= rr)
        || (lr >= rl && lr <= rr)
        || (rl >= ll && rl <= lr)
        || (rr >= ll && rr <= lr)
}

fn solve_part(input: &str, filter_fn: impl Fn(&&str) -> bool) -> Option<usize> {
    Some(input.lines().filter(filter_fn).count())
}

pub fn part_one(input: &str) -> Option<usize> {
    solve_part(input, is_fully_contained)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve_part(input, is_overlapping)
}

fn main() {
    let input = &aoc::read_file("inputs", 4);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
