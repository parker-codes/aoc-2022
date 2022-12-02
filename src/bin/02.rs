use std::convert::From;

pub fn part_one(input: &str) -> Option<u32> {
    let rounds = input.lines();

    let mut total_score = 0;

    for round in rounds {
        let mut plays = round.split_whitespace().into_iter();
        let opponent_play: Play = plays.next().unwrap().into();
        let my_play: Play = plays.next().unwrap().into();
        let result = my_play.against(opponent_play);
        let round_score = result.score() + my_play.score();
        total_score += round_score;
        // println!("Opponent: {opponent_play:?}, Me: {my_play:?}, Result: {result:?}");
        // println!(
        //     "Result score: {:?}, Play score: {:?}, Round score: {:?}",
        //     result.score(),
        //     my_play.score(),
        //     round_score
        // );
    }

    Some(total_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[derive(Debug, Copy, Clone)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Play {
    fn from(item: &str) -> Self {
        match item {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!(),
        }
    }
}

impl Play {
    fn against(&self, opponent_play: Self) -> MatchResult {
        match (self, opponent_play) {
            (Self::Rock, Self::Scissors)
            | (Self::Scissors, Self::Paper)
            | (Self::Paper, Self::Rock) => MatchResult::Win,
            (Self::Rock, Self::Rock)
            | (Self::Paper, Self::Paper)
            | (Self::Scissors, Self::Scissors) => MatchResult::Draw,
            _ => MatchResult::Lose,
        }
    }

    fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

#[derive(Debug)]
enum MatchResult {
    Win,
    Lose,
    Draw,
}

impl MatchResult {
    fn score(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Lose => 0,
            Self::Draw => 3,
        }
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
