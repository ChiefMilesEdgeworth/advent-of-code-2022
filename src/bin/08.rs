use itertools::Itertools;

type Direction = usize;

const NUM_DIRECTIONS: usize = 4;

const U: Direction = 0;
const D: Direction = 1;
const L: Direction = 2;
const R: Direction = 3;
const DIRECTIONS: [Direction; NUM_DIRECTIONS] = [U, D, L, R];

fn next_point(point: usize, direction: Direction, size: usize) -> Option<usize> {
    let square = size * size;
    match direction {
        U => point.checked_sub(size),
        D => ((point + size) < square).then_some(point + size),
        L => ((point - 1) % size != size - 1).then_some(point - 1),
        R => ((point + 1) % size != 0).then_some(point + 1),
        _ => unreachable!(),
    }
}

fn get_visibility(point: usize, direction: Direction, size: usize, bytes: &[u8]) -> bool {
    let mut np = next_point(point, direction, size);
    while let Some(next) = np {
        if bytes[next] >= bytes[point] {
            return false;
        }
        np = next_point(next, direction, size);
    }
    true
}

fn viewing_distance(point: usize, bytes: &[u8], size: usize) -> u32 {
    DIRECTIONS
        .map(|direction| {
            let mut np = next_point(point, direction, size);
            let mut accum = 0;
            while let Some(next) = np {
                accum += 1;
                if bytes[next] >= bytes[point] {
                    break;
                }
                np = next_point(next, direction, size);
            }
            accum
        })
        .into_iter()
        .product()
}

fn visible_from_outside(bytes: &[u8]) -> u32 {
    let size = (bytes.len() as f64).sqrt() as usize;
    let outside = size * 2 + (size - 2) * 2;
    let inner = (0..size * size)
        .filter(|&p| {
            p > size && p < ((size * size) - size) && p % size != 0 && p % size != (size - 1)
        })
        .filter(|&p| {
            DIRECTIONS
                .into_iter()
                .any(|dir| get_visibility(p, dir, size, bytes))
        })
        .count();
    outside as u32 + inner as u32
}

fn max_viewing_distance(bytes: &[u8]) -> u32 {
    let size = (bytes.len() as f64).sqrt() as usize;
    (0..size * size)
        .filter(|&p| {
            p > size && p < ((size * size) - size) && p % size != 0 && p % size != (size - 1)
        })
        .map(|p| viewing_distance(p, bytes, size))
        .max()
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let bytes = input
        .lines()
        .flat_map(|line| line.as_bytes().iter().map(|b| b - b'0'))
        .collect_vec();
    Some(visible_from_outside(&bytes))
}

pub fn part_two(input: &str) -> Option<u32> {
    let bytes = input
        .lines()
        .flat_map(|line| line.as_bytes().iter().map(|b| b - b'0'))
        .collect_vec();
    Some(max_viewing_distance(&bytes))
}

fn main() {
    let input = &aoc::read_file("inputs", 8);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
