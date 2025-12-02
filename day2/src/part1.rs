// part1.rs
// AoC 2025 Day 02 ; 2025-12-02 ; Copyright (c) 2025 Luke Harding ; MIT LICENSE

use crate::Puzzle;

pub fn process(puzzle: Puzzle) -> u64 {
    let mut bad_ids = vec![];

    for id in puzzle.product_ids {
        let id_str = id.to_string();
        let id_str_len = id_str.len();

        if id_str_len % 2 == 0 {
            let midpoint = id_str_len / 2;
            let first_half = &id_str[..midpoint];
            let last_half = &id_str[midpoint..];

            if first_half == last_half {
                bad_ids.push(id);
            }
        }
    }

    bad_ids.into_iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ParseError;
    use std::str::FromStr;

    #[test]
    fn test_process() -> Result<(), ParseError> {
        let sample = include_str!("../sample.txt");

        let puzzle = Puzzle::from_str(sample)?;

        assert_eq!(process(puzzle), 1227775554);

        Ok(())
    }
}
