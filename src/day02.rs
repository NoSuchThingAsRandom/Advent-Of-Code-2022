use std::fs::File;
use std::io::{BufRead, BufReader};
use anyhow::Context;
use itertools::Itertools;
use crate::{EXAMPLE_INPUT_PATH, INPUT_PATH};
use crate::misc::read_vec_string;


#[derive(PartialOrd, PartialEq, Clone, Copy, Debug)]
enum ShapeScore {
    Rock = 1,
    Paper = 2,
    Scissors = 3,

}

#[derive(Debug)]
enum OutcomeScore {
    Lose = 0,
    Draw = 3,
    Win = 6,

}

impl OutcomeScore {
    pub fn from_string(choice: &str) -> OutcomeScore {
        match choice {
            "X" => OutcomeScore::Lose,
            "Y" => OutcomeScore::Draw,
            "Z" => OutcomeScore::Win,
            _ => { panic!("Unknown choice!") }
        }
    }
}

impl ShapeScore {

    pub fn from_opponent_string(choice: &str) -> ShapeScore {
        match choice {
            "A" => ShapeScore::Rock,
            "B" => ShapeScore::Paper,
            "C" => ShapeScore::Scissors,
            _ => { panic!("Unknown choice: '{}'", choice) }
        }
    }
    pub fn from_me_string(choice: &str) -> ShapeScore {
        match choice {
            "X" => ShapeScore::Rock,
            "Y" => ShapeScore::Paper,
            "Z" => ShapeScore::Scissors,
            _ => { panic!("Unknown choice!") }
        }
    }
}

pub fn part_1(use_example_input: bool) -> anyhow::Result<usize> {
    let data = if use_example_input {
        "A Y\nB X\nC Z".split("\n").map(|line| line.to_string()).collect()
    } else {
        read_vec_string(INPUT_PATH.to_owned() + "02.txt")?
    };
    let mut score = 0;
    for line in data {
        let mut choices = line.split(" ");
        let opponent = choices.next().unwrap();
        let opponent: ShapeScore = ShapeScore::from_opponent_string(opponent);
        let me = choices.next().unwrap();
        let me: ShapeScore = ShapeScore::from_me_string(me);
        let outcome = if me == opponent {
            OutcomeScore::Draw
        } else if (me as u32) + 1 == opponent as u32 || (me == ShapeScore::Scissors && opponent == ShapeScore::Rock) {
            OutcomeScore::Lose
        } else {
            OutcomeScore::Win
        };
        let round_score = (me as u32) + (outcome as u32);
        //#println!("'{}'\tOpponent: {:?}, Me: {:?}, Outcome: {:?}, Score: {}",line, opponent, me, outcome, round_score);
        score += round_score as usize;
    }
    Ok(score)
}

pub fn part_2(use_example_input: bool) -> anyhow::Result<usize> {
    let data = if use_example_input {
        "A Y\nB X\nC Z".split("\n").map(|line| line.to_string()).collect()
    } else {
        read_vec_string(INPUT_PATH.to_owned() + "02.txt")?
    };
    let mut score = 0;
    for line in data {
        let mut choices = line.split(" ");
        let opponent = choices.next().unwrap();
        let opponent: ShapeScore = ShapeScore::from_opponent_string(opponent);
        let expected_outcome = choices.next().unwrap();
        let expected_outcome: OutcomeScore = OutcomeScore::from_string(expected_outcome);
        let me: ShapeScore = match expected_outcome {
            OutcomeScore::Lose => {
                match opponent {
                    ShapeScore::Rock => { ShapeScore::Scissors }
                    ShapeScore::Paper => { ShapeScore::Rock }
                    ShapeScore::Scissors => { ShapeScore::Paper }
                }
            }
            OutcomeScore::Draw => {
                opponent
            }
            OutcomeScore::Win => {
                match opponent {
                    ShapeScore::Rock => { ShapeScore::Paper }
                    ShapeScore::Paper => { ShapeScore::Scissors }
                    ShapeScore::Scissors => { ShapeScore::Rock }
                }
            }
        };
        let round_score = (me as u32) + (expected_outcome as u32);
        //#println!("'{}'\tOpponent: {:?}, Me: {:?}, Outcome: {:?}, Score: {}",line, opponent, me, outcome, round_score);
        score += round_score as usize;
    }
    Ok(score)
}
