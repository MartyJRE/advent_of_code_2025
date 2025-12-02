use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
";

fn is_repeating_twice(s: &str) -> bool {
    let len = s.len();
    if len % 2 != 0 || len == 0 {
        return false;
    }
    let half = len / 2;
    &s[..half] == &s[half..]
}

fn is_repeating_pattern(s: &str) -> bool {
    let len = s.len();
    if len < 2 {
        return false;
    }
    for pattern_len in 1..=len / 2 {
        if len % pattern_len == 0 {
            let pattern = &s[..pattern_len];
            let mut is_match = true;
            for i in (pattern_len..len).step_by(pattern_len) {
                if &s[i..i + pattern_len] != pattern {
                    is_match = false;
                    break;
                }
            }
            if is_match {
                return true;
            }
        }
    }
    false
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i64> {
        let line = reader.lines().next().unwrap()?;
        let parts = line.split(',').collect::<Vec<_>>();
        let mut sum = 0i64;
        for part in parts {
            let edges = part.split('-').collect::<Vec<_>>();
            assert!(edges.len() == 2);
            let start = edges[0].parse::<i64>()?;
            let end = edges[1].parse::<i64>()?;
            for id in start..=end {
                let str_id = id.to_string();
                if is_repeating_twice(&str_id) {
                    sum += id;
                }
            }
        }
        Ok(sum)
    }

    assert_eq!(1227775554, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i64> {
        let line = reader.lines().next().unwrap()?;
        let parts = line.split(',').collect::<Vec<_>>();
        let mut sum = 0i64;
        for part in parts {
            let edges = part.split('-').collect::<Vec<_>>();
            assert!(edges.len() == 2);
            let start = edges[0].parse::<i64>()?;
            let end = edges[1].parse::<i64>()?;
            for id in start..=end {
                let str_id = id.to_string();
                if is_repeating_pattern(&str_id) {
                    sum += id;
                }
            }
        }
        Ok(sum)
    }

    assert_eq!(4174379265, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
