// bin/part1.rs
// AoC 2025 Day 02 ; 2025-12-02 ; Copyright (c) 2025 Luke Harding ; MIT LICENSE

use day2::{Puzzle, RunError, part1::process};
use std::{fs, str::FromStr};

fn main() -> Result<(), RunError> {
    let input = fs::read_to_string("input.txt")?;
    let puzzle = Puzzle::from_str(&input)?;
    let result = process(puzzle);
    println!("Part 1: {}", result);
    Ok(())
}
