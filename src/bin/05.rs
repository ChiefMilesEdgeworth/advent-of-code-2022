type Stack = Vec<u8>;

#[derive(Clone, Copy, Debug)]
struct Move {
    num: usize,
    src: usize,
    dest: usize,
}

fn parts(input: &str) -> (&str, &str) {
    input.split_once("\n\n").unwrap()
}

fn parse_stack(stack: &str) -> Vec<Stack> {
    let mut s = vec![Vec::with_capacity(20); 9];
    for l in stack.lines().rev().skip(1) {
        l.bytes()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, c)| c != &b' ')
            .for_each(|(i, c)| s[i].push(c))
    }
    s
}

fn parse_moves(moves: &str) -> impl Iterator<Item = Move> + '_ {
    moves.lines().map(|line| {
        let mut parts = line.split_whitespace().skip(1).step_by(2);
        let num = parts.next().unwrap().parse::<usize>().unwrap();
        let src = parts.next().unwrap().parse::<usize>().unwrap() - 1;
        let dest = parts.next().unwrap().parse::<usize>().unwrap() - 1;
        Move { num, src, dest }
    })
}

fn run_moves_p1(mut stacks: Vec<Stack>, moves: impl Iterator<Item = Move>) -> Vec<Stack> {
    for m in moves {
        for _ in 0..m.num {
            let c = stacks[m.src].pop().unwrap();
            stacks[m.dest].push(c);
        }
    }
    stacks
}

fn run_moves_p2(mut stacks: Vec<Stack>, moves: impl Iterator<Item = Move>) -> Vec<Stack> {
    let mut swap = vec![0; 64];
    for Move { num, src, dest } in moves {
        let swap = &mut swap[..num];
        let len = stacks[src].len();
        let range = len - num..len;
        swap.copy_from_slice(&stacks[src][range]);
        stacks[src].truncate(len - num);
        stacks[dest].extend(swap.iter());
    }
    stacks
}
pub fn part_one(input: &str) -> Option<String> {
    let (stacks, moves) = parts(input);
    let (stacks, moves) = (parse_stack(stacks), parse_moves(moves));
    let stacks = run_moves_p1(stacks, moves);
    Some(
        stacks
            .into_iter()
            .filter_map(|stack| stack.last().copied())
            .map(|c| c as char)
            .collect::<String>(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let (stacks, moves) = parts(input);
    let (stacks, moves) = (parse_stack(stacks), parse_moves(moves));
    let stacks = run_moves_p2(stacks, moves);
    Some(
        stacks
            .into_iter()
            .filter_map(|stack| stack.last().copied())
            .map(|c| c as char)
            .collect::<String>(),
    )
}

fn main() {
    let input = &aoc::read_file("inputs", 5);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
