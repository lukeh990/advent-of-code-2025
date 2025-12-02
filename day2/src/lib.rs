// lib.rs
// AoC 2025 Day 02 ; 2025-12-02 ; Copyright (c) 2025 Luke Harding ; MIT LICENSE

// Crate structure inspired by: lunar-mycroft/advent_of_code

use std::{io, num::ParseIntError, str::FromStr};
use thiserror::Error;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Puzzle {
    product_ids: Vec<u64>,
}

impl FromStr for Puzzle {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        let split = s.split(',');
        let mut product_ids = vec![];

        for group in split {
            let mut range_ends = group.split('-');

            //eprintln!("G: {} ;", group);

            let lower: u64 = (range_ends
                .next()
                .ok_or(ParseError::GroupNoBounds(String::from(group)))?)
            .parse()?;

            //eprintln!(" LB: {} ;", lower);

            let upper: u64 = (range_ends
                .next()
                .ok_or(ParseError::GroupNoBounds(String::from(group)))?)
            .parse()?;

            //eprintln!(" UP: {} ;", upper);

            if lower >= upper {
                eprintln!(
                    "Potential error: Parsing `{}` to bounds `{}` -> `{}`. Upper bound smaller or equal to lower.",
                    group, lower, upper
                );
            }

            product_ids.extend(lower..=upper);
        }

        Ok(Puzzle { product_ids })
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("No upper or lower bound detected. Found: `{0}`")]
    GroupNoBounds(String),
    #[error("Error parsing strings to integers.")]
    IntParse(#[from] ParseIntError),
}

#[derive(Debug, Error)]
pub enum RunError {
    #[error("IO Error")]
    IO(#[from] io::Error),
    #[error("Parse Error")]
    Parse(#[from] ParseError),
    #[error("Chunking Failure. String: `{0}`")]
    Chunk(String),
}
