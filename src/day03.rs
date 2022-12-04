use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use anyhow::Context;
use crate::{EXAMPLE_INPUT_PATH, INPUT_PATH};
use crate::misc::read_vec_string;


pub fn part_1(use_example_input: bool) -> anyhow::Result<usize> {
    let filename = if use_example_input {
        EXAMPLE_INPUT_PATH.to_owned() + "03/A.txt"
    } else {
        INPUT_PATH.to_owned() + "03.txt"
    };
    let data = read_vec_string(filename)?;
    let mut score = 0;
    for line in data {
        let (one, two) = line.split_at(line.len() / 2);
        let one: HashSet<char> = HashSet::from_iter(one.chars());
        let two: HashSet<char> = HashSet::from_iter(two.chars());
        let duplicates = one.intersection(&two);
        for char in duplicates {
            let mut char_int = *char as u32;
            if char_int < 91 {
                char_int -= 65;
                char_int += 27;
            } else {
                char_int -= 97;
                char_int += 1;
            }
            //print!("{} -> {},\t", char, char_int);
            score += char_int as usize;
        }
        /*        for char in one.chars() {
                    if two.contains(&char){
                        println!("'{}' -> '{}'",char,char as u32);
                    }
                }*/
    }
    Ok(score)
}

pub fn part_2(use_example_input: bool) -> anyhow::Result<usize> {
    let filename = if use_example_input {
        EXAMPLE_INPUT_PATH.to_owned() + "03/A.txt"
    } else {
        INPUT_PATH.to_owned() + "03.txt"
    };
    let data = read_vec_string(filename)?;
    let mut score = 0;
    let mut elf_group: Vec<HashSet<char>> = Vec::with_capacity(3);
    for line in data {
        elf_group.push(HashSet::from_iter(line.chars()));
        if elf_group.len() == 3 {
            let a = elf_group[1].intersection(&elf_group[2]);
            let b: HashSet<char> = a.map(|x| *x).collect();
            let unique = elf_group[0].intersection(&b);
            for char in unique {
                let mut char_int = *char as u32;
                if char_int < 91 {
                    char_int -= 65;
                    char_int += 27;
                } else {
                    char_int -= 97;
                    char_int += 1;
                }
                //println!("{} -> {},\t", char, char_int);
                score += char_int as usize;
            }
            elf_group = Vec::with_capacity(3);
        }
    }
    Ok(score)
}
