// part2.rs
// AoC 2025 Day 02 ; 2025-12-02 ; Copyright (c) 2025 Luke Harding ; MIT LICENSE

use std::char;

use crate::{Puzzle, RunError};

pub fn process(puzzle: Puzzle) -> Result<u64, RunError> {
    let mut bad_ids = vec![];

    for id in puzzle.product_ids {
        let id_str = id.to_string();
        let id_str_len = id_str.len();
        let midpoint = id_str_len / 2;

        for i in (1..=midpoint).rev() {
            if id_str_len % i != 0 {
                continue;
            }

            let chars: Vec<char> = id_str.chars().collect();
            let mut chunks = chars.chunks(i);

            let master = chunks.next().ok_or(RunError::Chunk(id_str.clone()))?;

            if chunks.all(|chunk| chunk == master) {
                bad_ids.push(id);
                break;
            }
        }
    }

    Ok(bad_ids.into_iter().sum())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::RunError;
    use std::str::FromStr;

    #[test]
    fn test_process() -> Result<(), RunError> {
        let sample = include_str!("../sample.txt");

        let puzzle = Puzzle::from_str(sample)?;

        assert_eq!(process(puzzle)?, 4174379265);

        Ok(())
    }
}
