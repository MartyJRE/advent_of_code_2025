use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result::Ok;
use std::str::FromStr;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut zeros = 0;
        let mut start = 50;
        for line in reader.lines() {
            let line = line?;
            let first = line.chars().next();
            let direction = match first {
                Some(c) => {
                    if c == 'R' {
                        1
                    } else if c == 'L' {
                        -1
                    } else {
                        0
                    }
                }
                None => 0,
            };
            let mut count = i32::from_str(&line[1..])?;
            while count != 0 {
                if direction == 1 {
                    start += 1;
                    count -= 1;
                } else if direction == -1 {
                    start -= 1;
                    count -= 1;
                }
                if start > 99 {
                    start = 0;
                } else if start < 0 {
                    start = 99;
                }
            }
            if start == 0 {
                zeros += 1;
            }
        }
        Ok(zeros)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {

        let mut zeros = 0;
        let mut start = 50;
        for line in reader.lines() {
            let line = line?;
            let first = line.chars().next();
            let direction = match first {
                Some(c) => {
                    if c == 'R' {
                        1
                    } else if c == 'L' {
                        -1
                    } else {
                        0
                    }
                }
                None => 0,
            };
            let mut count = i32::from_str(&line[1..])?;
            while count != 0 {
                if direction == 1 {
                    start += 1;
                    count -= 1;
                } else if direction == -1 {
                    start -= 1;
                    count -= 1;
                }
                if start > 99 {
                    start = 0;
                } else if start < 0 {
                    start = 99;
                }
                if start == 0 {
                    zeros += 1;
                }
            }
        }
        Ok(zeros)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
