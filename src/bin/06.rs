fn no_duplicates(window: &[u8], size: usize) -> bool {
    let n = window
        .iter()
        .fold(0u32, |c, b| c | 1 << (b - b'a'))
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

pub fn part_one(input: &str) -> Option<usize> {
    message_start(input, 4)
}

pub fn part_two(input: &str) -> Option<usize> {
    message_start(input, 14)
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
