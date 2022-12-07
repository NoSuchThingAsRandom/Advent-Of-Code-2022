use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use anyhow::Context;
use crate::{EXAMPLE_INPUT_PATH, INPUT_PATH};
use crate::misc::read_vec_string;

pub struct Movements {
    amount: usize,
    source: usize,
    dest: usize,
}

pub fn part_1(use_example_input: bool) -> anyhow::Result<usize> {
    let filename = if use_example_input {
        EXAMPLE_INPUT_PATH.to_owned() + "06/C.txt"
    } else {
        INPUT_PATH.to_owned() + "06.txt"
    };
    let mut data = read_vec_string(filename)?.first().unwrap().to_string();
    let mut stack = VecDeque::new();
    for (index, letter) in data.chars().enumerate() {
        println!("Index: {}, Stack: {:?}", index, stack);
        if stack.len() < 4 {
            stack.push_back(letter);
        } else {
            stack.push_back(letter);
            let mut conflicts = false;
            for (index, check) in stack.iter().enumerate() {
                for (sub_index, sub_check) in stack.iter().enumerate() {
                    if index != sub_index && check == sub_check {
                        conflicts = true;
                        break;
                    }
                }
            }

            stack.pop_front();

            if !conflicts {
                //println!("Found at index {}", index);
                return Ok(index);
            }
        }
    }
    Ok(0)
}

pub fn part_2(use_example_input: bool) -> anyhow::Result<usize> {
    let filename = if use_example_input {
        EXAMPLE_INPUT_PATH.to_owned() + "06/E.txt"
    } else {
        INPUT_PATH.to_owned() + "06.txt"
    };
    let mut data = read_vec_string(filename)?.first().unwrap().to_string();
    let mut stack = VecDeque::new();
    for (index, letter) in data.chars().enumerate() {
        //println!("Index: {}, Stack: {:?}", index, stack);
        if stack.len() < 13 {
            stack.push_back(letter);
        } else {
            stack.push_back(letter);
            let mut conflicts = false;
            for (index, check) in stack.iter().enumerate() {
                for (sub_index, sub_check) in stack.iter().enumerate() {
                    if index != sub_index && check == sub_check {
                        conflicts = true;
                        break;
                    }
                }
            }

            stack.pop_front();

            if !conflicts {
                //println!("Found at index {}", index);
                return Ok(index+1);
            }
        }
    }
    Ok(0)
}
