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
        EXAMPLE_INPUT_PATH.to_owned() + "05/A.txt"
    } else {
        INPUT_PATH.to_owned() + "05.txt"
    };
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    let mut data = read_vec_string(filename)?;
    let mut stack_build = true;
    for line in data {
        let mut stack_index = 0;
        let mut space_count: Option<u32> = None;
        if stack_build {
            for letter in line.chars() {
                if letter == ' ' {
                    space_count = match space_count {
                        Some(space_count) => Some(space_count + 1),
                        None => Some(1),
                    };
                } else if letter == '[' {
                    if space_count == Some(1) {
                        stack_index += 1;
                    } else if let Some(count) = space_count {
                        stack_index += 1 + (count / 4);
                    }
                    space_count = None;
                } else if letter.is_alphabetic() {
                    while stacks.len() <= stack_index as usize {
                        stacks.push(VecDeque::new());
                    }
                    stacks.get_mut(stack_index as usize).unwrap().push_front(letter);
                } else if letter.is_numeric() {
                    stack_build = false;
                    break;
                }
            }
        } else {
            if line.trim().is_empty() {
                continue;
            }
            println!("---------\n");
            for stack in &stacks {
                for el in stack {
                    print!("{}", el)
                }
                println!("");
            }
            let mut split = line.split("from");
            let amount: usize = split.next().unwrap().replace("move ", "").trim().parse().unwrap();
            let mut locations = split.next().unwrap().split(" to ");
            let mut source: usize = locations.next().unwrap().trim().parse().unwrap();
            let mut dest: usize = locations.next().unwrap().trim().parse().unwrap();
            let mut moved = Vec::with_capacity(amount);
            source -= 1;
            dest -= 1;
            let current_stack = stacks.get_mut(source).unwrap();
            for i in 0..amount {
                if let Some(element) = current_stack.pop_back() {
                    moved.push(element);
                }
            }
            let current_stack = stacks.get_mut(dest).unwrap();
            for moved_crate in moved {
                current_stack.push_back(moved_crate);
            }
        }
    }
    let mut out = String::new();
    for stack in stacks {
        out.push(*stack.back().unwrap());
    }
    println!("Output: '{}'", out);
    Ok(0)
}

pub fn part_2(use_example_input: bool) -> anyhow::Result<usize> {
    let filename = if use_example_input {
        EXAMPLE_INPUT_PATH.to_owned() + "05/A.txt"
    } else {
        INPUT_PATH.to_owned() + "05.txt"
    };
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    let mut data = read_vec_string(filename)?;
    let mut stack_build = true;
    for line in data {
        let mut stack_index = 0;
        let mut space_count: Option<u32> = None;
        if stack_build {
            for letter in line.chars() {
                if letter == ' ' {
                    space_count = match space_count {
                        Some(space_count) => Some(space_count + 1),
                        None => Some(1),
                    };
                } else if letter == '[' {
                    if space_count == Some(1) {
                        stack_index += 1;
                    } else if let Some(count) = space_count {
                        stack_index += 1 + (count / 4);
                    }
                    space_count = None;
                } else if letter.is_alphabetic() {
                    while stacks.len() <= stack_index as usize {
                        stacks.push(VecDeque::new());
                    }
                    stacks.get_mut(stack_index as usize).unwrap().push_front(letter);
                } else if letter.is_numeric() {
                    stack_build = false;
                    break;
                }
            }
        } else {
            if line.trim().is_empty() {
                continue;
            }

            println!("---------\n");
            for stack in &stacks {
                for el in stack {
                    print!("{}", el)
                }
                println!("");
            }
            let mut split = line.split("from");
            let amount: usize = split.next().unwrap().replace("move ", "").trim().parse().unwrap();
            let mut locations = split.next().unwrap().split(" to ");
            let mut source: usize = locations.next().unwrap().trim().parse().unwrap();
            let mut dest: usize = locations.next().unwrap().trim().parse().unwrap();
            let mut moved: VecDeque<char> = VecDeque::with_capacity(amount);
            println!("{} crate from {} to {}", amount, source, dest);
            source -= 1;
            dest -= 1;
            let current_stack = stacks.get_mut(source).unwrap();
            for i in 0..amount {
                println!("\t{:?}", current_stack);
                moved.push_front(current_stack.pop_back().unwrap());
                /*if let Some(element) =  {

                }*/
            }
            let current_stack = stacks.get_mut(dest).unwrap();
            for moved_crate in moved {
                current_stack.push_back(moved_crate);
            }
        }
    }
    let mut out = String::new();
    for stack in stacks {
        out.push(*stack.back().unwrap());
    }
    println!("Output: '{}'", out);
    return Ok(0);
}
