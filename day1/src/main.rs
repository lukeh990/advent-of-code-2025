// main.rs
// AoC 2025 Day 01 ; 2025-12-01 ; Copyright (c) 2025 Luke Harding ; MIT LICENSE

// Accepts a string of rotations in the format `<L|R>1-N` which move a dial
// left or right the number of units specified. The dial goes from 0-99 and is
// circular. Each time the dial stops at 0, add 1 to the password.

use std::fs;
use std::io;

use thiserror::Error;

#[cfg(test)]
mod tests;

fn main() -> io::Result<()> {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)?;

    // Part 1
    println!("Part 1:");
    match analyze_roatations(50, &contents, false) {
        Ok(password) => println!("\tThe password is: {}", password),
        Err(e) => eprintln!("\t{}", e),
    }

    // Part 2
    println!("Part 2:");
    match analyze_roatations(50, &contents, true) {
        Ok(password) => println!("\tThe password is: {}", password),
        Err(e) => eprintln!("\t{}", e),
    }

    Ok(())
}

pub fn analyze_roatations<S: AsRef<str>>(
    starting_pos: u32,
    rotation_str: S,
    part_two: bool,
) -> Result<u32, ParseError> {
    let rotation_str = rotation_str.as_ref();
    let mut position = starting_pos;
    let mut password = 0;

    for line in rotation_str.lines() {
        let rotation = parse_rotation(line)?;

        //eprint!("Start: {} -> {:?}", position, rotation);

        match rotation {
            Rotation::Left(distance) => match position.checked_sub(distance) {
                Some(result) => position = result,
                None => {
                    let mut tmp_signed = position as i32 - (distance as i32 % 100);
                    //eprint!(" -> TMP: {}", tmp_signed);

                    if part_two {
                        password += distance / 100;
                        //eprint!(" -> CLICK x{}", distance / 100);
                    }

                    if tmp_signed < 0 {
                        //eprint!(" -> ROLLOVER");
                        tmp_signed += 100;

                        if part_two && position != 0 {
                            password += 1;
                            //eprint!(" ->  EXTRA CLICK");
                        }
                    }

                    position = tmp_signed as u32;
                }
            },
            Rotation::Right(distance) => {
                position += distance % 100;
                //eprint!(" -> INT: {}", position);

                if part_two {
                    password += distance / 100;
                    //eprint!(" -> CLICK x{}", distance / 100);
                }

                if position > 99 {
                    //eprint!(" -> ROLLOVER");

                    position -= 100;

                    if part_two && position != 0 {
                        password += 1;
                        //eprint!(" -> EXTRA CLICK");
                    }
                }
            }
        }

        //eprint!(" -> Final: {}", position);

        if position == 0 {
            password += 1;
            //eprint!(" -> END ZERO");
        }

        //eprintln!(" -> PASS: {}", password);
    }

    Ok(password)
}

pub fn parse_rotation(str: &str) -> Result<Rotation, ParseError> {
    let mut chars = str.chars();
    let direction; // 1 for Left; 2 for Right
    let mut distance = 0;

    // First Character
    if let Some(ch) = chars.next() {
        match ch {
            'L' => direction = 1,
            'R' => direction = 2,
            _ => return Err(ParseError::InvalidDirection(ch)),
        }
    } else {
        return Err(ParseError::EmptyLine);
    }

    // Distance Characters to u32
    for ch in chars {
        if let Some(digit) = ch.to_digit(10) {
            if distance > u32::MAX / 10 {
                return Err(ParseError::Overflow(String::from(str)));
            }
            distance *= 10;

            if distance > u32::MAX - digit {
                return Err(ParseError::Overflow(String::from(str)));
            }
            distance += digit;
        } else {
            return Err(ParseError::InvalidDigit(ch));
        }
    }

    match direction {
        1 => Ok(Rotation::Left(distance)),
        2 => Ok(Rotation::Right(distance)),
        _ => Err(ParseError::Unkown),
    }
}

#[derive(Debug, PartialEq)]
pub enum Rotation {
    Left(u32),
    Right(u32),
}

#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("Invalid Direction: `{0}`; Expected 'L' | 'R'")]
    InvalidDirection(char),
    #[error("No First Character. Empty Line?")]
    EmptyLine,
    #[error("Overflow Detected. Number too big for u32: `{0}`")]
    Overflow(String),
    #[error("Non-numeric character detected when parsing distance. Found; `{0}`")]
    InvalidDigit(char),
    #[error("Unknown Error.")]
    Unkown,
}
