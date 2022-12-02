#[derive(Clone, Copy, Debug)]
enum Game {
    Win,
    Lose,
    Tie,
}

impl Game {
    fn from(opponent: Throw, me: Throw) -> Game {
        if opponent.wins_against() == me {
            Game::Lose
        } else if opponent.loses_to() == me {
            Game::Win
        } else {
            Game::Tie
        }
    }
    fn value(self) -> u32 {
        match self {
            Game::Win => 6,
            Game::Tie => 3,
            Game::Lose => 0,
        }
    }

    fn needed_throw(self, opponent: Throw) -> Throw {
        match self {
            Game::Win => opponent.loses_to(),
            Game::Lose => opponent.wins_against(),
            Game::Tie => opponent,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Throw {
    Rock,
    Paper,
    Scissors,
}

impl Throw {
    fn value(self) -> u32 {
        match self {
            Throw::Rock => 1,
            Throw::Paper => 2,
            Throw::Scissors => 3,
        }
    }

    fn wins_against(self) -> Self {
        match self {
            Throw::Rock => Throw::Scissors,
            Throw::Paper => Throw::Rock,
            Throw::Scissors => Throw::Paper,
        }
    }

    fn loses_to(self) -> Self {
        match self {
            Throw::Rock => Throw::Paper,
            Throw::Paper => Throw::Scissors,
            Throw::Scissors => Throw::Rock,
        }
    }
}

fn decrypt_p1(s: &str) -> Throw {
    match s {
        "A" | "X" => Throw::Rock,
        "B" | "Y" => Throw::Paper,
        "C" | "Z" => Throw::Scissors,
        _ => unreachable!(),
    }
}

fn decrypt_p2((opponent, game): (&str, &str)) -> (Throw, Game) {
    (
        match opponent {
            "A" => Throw::Rock,
            "B" => Throw::Paper,
            "C" => Throw::Scissors,
            _ => unreachable!(),
        },
        match game {
            "X" => Game::Lose,
            "Y" => Game::Tie,
            "Z" => Game::Win,
            _ => unreachable!(),
        },
    )
}

fn split_input(input: &str) -> impl Iterator<Item = (&str, &str)> {
    input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().split_once(' ').expect("Only A-C and X-Z"))
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        split_input(input)
            .map(|(left, right)| (decrypt_p1(left), decrypt_p1(right)))
            .map(|(opponent, me)| me.value() + Game::from(opponent, me).value())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        split_input(input)
            .map(decrypt_p2)
            .map(|(opponent, game)| game.needed_throw(opponent).value() + game.value())
            .sum(),
    )
}

fn main() {
    let input = &aoc::read_file("inputs", 2);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
