use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use anyhow::Context;
use crate::{EXAMPLE_INPUT_PATH, INPUT_PATH};
use crate::misc::read_vec_string;


pub fn part_1(use_example_input: bool) -> anyhow::Result<usize> {
    let filename = if use_example_input {
        EXAMPLE_INPUT_PATH.to_owned() + "04/A.txt"
    } else {
        INPUT_PATH.to_owned() + "04.txt"
    };
    let data = read_vec_string(filename)?;
    let mut overlap_count = 0;
    for line in data {
        let mut elf_ranges = Vec::new();
        let all_ranges = line.split(",");
        for range in all_ranges {
            let mut nums = range.split("-");
            let start: u32 = nums.next().unwrap().parse()?;
            let end: u32 = nums.next().unwrap().parse()?;
            elf_ranges.push((start, end));
        }
        let (start, end) = elf_ranges[0];
        let (sub_start, sub_end) = elf_ranges[1];
        if (start <= sub_start && sub_end <= end) || (sub_start <= start && end <= sub_end) {
            //println!("\t{},{} -> {},{}", start, end, sub_start, sub_end);
            overlap_count += 1;
        }
    }
    Ok(overlap_count)
}

pub fn part_2(use_example_input: bool) -> anyhow::Result<usize> {
    let filename = if use_example_input {
        EXAMPLE_INPUT_PATH.to_owned() + "04/A.txt"
    } else {
        INPUT_PATH.to_owned() + "04.txt"
    };
    let data = read_vec_string(filename)?;
    let mut overlap_count = 0;
    for line in data {
        let mut elf_ranges = Vec::new();
        let all_ranges = line.split(",");
        for range in all_ranges {
            let mut nums = range.split("-");
            let start: u32 = nums.next().unwrap().parse()?;
            let end: u32 = nums.next().unwrap().parse()?;
            elf_ranges.push((start, end));
        }
        let (start, end) = elf_ranges[0];
        let (sub_start, sub_end) = elf_ranges[1];
        if (sub_start <= start && start <= sub_end) || (sub_start <= end && end <= sub_end) || (start <= sub_start && sub_start <= end) || (start <= sub_end && sub_end <= end) {
            //println!("\t{},{} -> {},{}", start, end, sub_start, sub_end);
            overlap_count += 1;
        }
    }
    Ok(overlap_count)
}
