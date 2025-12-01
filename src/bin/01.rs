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

    fn parts<R: BufRead>(reader: R, count_between: bool) -> Result<usize> {
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
            let mut count = u16::from_str(&line[1..])?;
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
                if count_between && start == 0 {
                    zeros += 1;
                }
            }
            if !count_between && start == 0 {
                zeros += 1;
            }
        }
        Ok(zeros)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(3, parts(BufReader::new(TEST.as_bytes()),false)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(parts(input_file, false)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    assert_eq!(6, parts(BufReader::new(TEST.as_bytes()), true)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(parts(input_file, true)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
