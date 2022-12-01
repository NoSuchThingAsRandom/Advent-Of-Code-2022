mod day01;

pub const EXAMPLE_INPUT_PATH: &str = "example_inputs/day";
pub const INPUT_PATH: &str = "inputs/day";

fn main() {
    println!("Day 1 Part 1: {}", day01::part_1(false).unwrap());
    println!("Day 1 Part 1: {}", day01::part_2(false).unwrap());
}
