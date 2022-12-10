use hashbrown::HashSet;
use std::hash::Hash;

#[derive(Clone, Copy, Debug)]
enum Direction {
    U,
    D,
    L,
    R,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "U" => Self::U,
            "D" => Self::D,
            "L" => Self::L,
            "R" => Self::R,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Move {
    dir: Direction,
    count: u16,
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        let (dir, count) = s.split_once(' ').unwrap();
        let (dir, count) = (dir.into(), count.parse().unwrap());
        Self { dir, count }
    }
}

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Point(i16, i16);

fn new_tail_pos(Point(hx, hy): Point, tail: &mut Point) {
    let (dx, dy) = (hx - tail.0, hy - tail.1);
    if dx.abs() > 1 || dy.abs() > 1 {
        tail.0 += dx.signum();
        tail.1 += dy.signum();
    }
}

fn positions_visited<const N: usize>(moves: impl Iterator<Item = Move>) -> u32
where
    [(); N]: Sized,
{
    let mut tail_positions = HashSet::with_capacity(2000);
    tail_positions.insert(Point::default());
    let mut rope = [Point::default(); N];
    for Move { dir, count } in moves {
        'm: for _ in 0..count {
            match dir {
                Direction::U => rope[0].1 += 1,
                Direction::D => rope[0].1 -= 1,
                Direction::R => rope[0].0 += 1,
                Direction::L => rope[0].0 -= 1,
            }
            for i in 1..N {
                let last = rope[i];
                new_tail_pos(rope[i - 1], &mut rope[i]);
                if rope[i] == last {
                    continue 'm;
                }
            }
            tail_positions.insert(rope[N - 1]);
        }
    }
    tail_positions.len() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let moves = input.lines().map(|line| line.into());
    Some(positions_visited::<2>(moves))
}

pub fn part_two(input: &str) -> Option<u32> {
    let moves = input.lines().map(|line| line.into());
    Some(positions_visited::<10>(moves))
}

fn main() {
    let input = &aoc::read_file("inputs", 9);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
