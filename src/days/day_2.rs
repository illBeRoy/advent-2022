use crate::day::Day;
use crate::input::read_input;

const INPUT_FILE: &str = "day2.txt";

#[derive(Clone, Copy)]
pub struct Day2 {}

impl Day for Day2 {
    fn title(&self) -> &'static str {
        "Rock Paper Scissors"
    }

    fn task_1(&self) -> String {
        let input = read_input(INPUT_FILE);

        fn parse_match_line(match_line: &str) -> Match {
            let their_hand = match match_line.chars().nth(0) {
                Some('A') => Hand::Rock,
                Some('B') => Hand::Paper,
                Some('C') => Hand::Scissors,
                _ => panic!("invalid input"),
            };

            let your_hand = match match_line.chars().nth(2) {
                Some('X') => Hand::Rock,
                Some('Y') => Hand::Paper,
                Some('Z') => Hand::Scissors,
                _ => panic!("invalid input"),
            };

            Match {
                your_hand,
                their_hand,
            }
        }

        let total_score: u32 = input.lines().map(parse_match_line).map(|m| m.score()).sum();

        format!("total score: {}", total_score)
    }

    fn task_2(&self) -> String {
        let input = read_input(INPUT_FILE);

        fn parse_match_line(match_line: &str) -> Match {
            let their_hand = match match_line.chars().nth(0) {
                Some('A') => Hand::Rock,
                Some('B') => Hand::Paper,
                Some('C') => Hand::Scissors,
                _ => panic!("invalid input"),
            };

            let your_hand = match match_line.chars().nth(2) {
                Some('X') => their_hand.wins_over(),
                Some('Y') => their_hand.clone(),
                Some('Z') => their_hand.loses_to(),
                _ => panic!("invalid input"),
            };

            Match {
                your_hand,
                their_hand,
            }
        }

        let total_score: u32 = input.lines().map(parse_match_line).map(|m| m.score()).sum();

        format!("total score: {}", total_score)
    }
}

#[derive(PartialEq, Debug, Clone)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Hand {
    fn loses_to(&self) -> Self {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }

    fn wins_over(&self) -> Self {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }
}

struct Match {
    your_hand: Hand,
    their_hand: Hand,
}

impl Match {
    fn did_win(&self) -> bool {
        (self.your_hand == Hand::Rock && self.their_hand == Hand::Scissors)
            || (self.your_hand == Hand::Paper && self.their_hand == Hand::Rock)
            || (self.your_hand == Hand::Scissors && self.their_hand == Hand::Paper)
    }

    fn score(&self) -> u32 {
        let hand_score: u8 = match self.your_hand {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        };

        let draw = self.your_hand == self.their_hand;
        let win_score: u8 = if draw {
            3
        } else if self.did_win() {
            6
        } else {
            0
        };

        (hand_score + win_score) as u32
    }
}
