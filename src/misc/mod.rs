use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Context;
use log::debug;

use crate::misc::error::{AoCError, AoCResult};

pub mod error;

pub fn read_vec_ints(filename: String) -> anyhow::Result<Vec<usize>> {
    debug!("Opening file {}", filename);
    let file = File::open(filename.to_string()).context(format!("Failed to file {}", filename))?;
    let reader = BufReader::new(file);
    let mut data = Vec::new();
    for line in reader.lines() {
        data.push(line?.parse()?);
    }
    Ok(data)
}

pub fn read_comma_separated_ints(filename: String) -> anyhow::Result<Vec<usize>> {
    debug!("Opening file {}", filename);
    let file = File::open(filename.to_string()).context(format!("Failed to file {}", filename))?;
    let reader = BufReader::new(file);
    let mut data = Vec::new();
    for line in reader.lines() {
        for digit in line?.split(",") {
            println!("{}",digit);
            data.push(digit.parse().unwrap());
        }
    }
    Ok(data)
}

pub fn read_vec_string(filename: String) -> anyhow::Result<Vec<String>> {
    debug!("Opening file {}", filename);
    let file = File::open(filename.to_string()).context(format!("Failed to file {}", filename))?;
    let reader = BufReader::new(file);
    let mut data = Vec::new();
    for line in reader.lines() {
        data.push(line?);
    }
    Ok(data)
}

/// Attempts to retrieve a value from a vec, and converts to an AoC Error if it fails
pub fn get_values<T: std::fmt::Debug>(data: &[T], index: usize) -> AoCResult<&T> {
    data.get(index).ok_or_else(|| {
        AoCError::new(format!(
            "Couldn't index vector!\nIndex: {}\nData: {:?}",
            index, data,
        ))
    })
}
