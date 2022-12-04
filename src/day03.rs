use std::fs::File;
use std::io::{BufRead, BufReader};
use anyhow::Context;
use crate::{EXAMPLE_INPUT_PATH, INPUT_PATH};


pub fn part_1(use_example_input: bool) -> anyhow::Result<usize> {
    let filename = if use_example_input {
        EXAMPLE_INPUT_PATH.to_owned() + "01/A.txt"
    } else {
        INPUT_PATH.to_owned() + "01.txt"
    };
    let file = File::open(filename.to_string()).context(format!("Failed to file {}", filename))?;
    let reader = BufReader::new(file);
    let mut current_elf_index = 1;
    let mut current_calorie_count = 0;
    let mut most_calories = 0;
    let mut most_calories_index = 0;
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            if current_calorie_count > most_calories {
                most_calories = current_calorie_count;
                most_calories_index = current_elf_index;
            }
            current_elf_index += 1;
            current_calorie_count = 0;
        } else {
            let calories: usize = line.parse()?;
            current_calorie_count += calories;
        }
    }
    Ok(most_calories)
}

pub fn part_2(use_example_input: bool) -> anyhow::Result<usize> {
    let filename = if use_example_input {
        EXAMPLE_INPUT_PATH.to_owned() + "01/A.txt"
    } else {
        INPUT_PATH.to_owned() + "01.txt"
    };
    let file = File::open(filename.to_string()).context(format!("Failed to file {}", filename))?;
    let reader = BufReader::new(file);
    let mut current_calorie_count = 0;
    let mut calories_per_elf = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            calories_per_elf.push(current_calorie_count);
            current_calorie_count = 0;
        } else {
            let calories: usize = line.parse()?;
            current_calorie_count += calories;
        }
    }
    calories_per_elf.push(current_calorie_count);
    calories_per_elf.sort_unstable();
    calories_per_elf.reverse();
    let total = calories_per_elf[0] + calories_per_elf[1] + calories_per_elf[2];
    Ok(total)
}
