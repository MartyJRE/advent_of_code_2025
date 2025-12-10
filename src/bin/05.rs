use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let mut ranges = Vec::new();
    let mut values = Vec::new();
    let mut crossed_over = false;
    for line in reader.lines() {
        let line = &line?;
        if line.is_empty() {
            crossed_over = true;
            continue;
        }
        if !crossed_over {
            let (start, end) = line.rsplit_once('-').unwrap();
            ranges.push((start.parse::<u64>()?, end.parse::<u64>()?));
        } else {
            values.push(line.parse::<u64>()?);
        }
    }

    let mut result = 0;
    for &value in &values {
        for &(start, end) in &ranges {
            if (start..=end).contains(&value) {
                result += 1;
                break;
            }
        }
    }
    Ok(result)
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let mut ranges = Vec::new();
    for line in reader.lines() {
        let line = &line?;
        if line.is_empty() {
            break;
        }
        let (start, end) = line.rsplit_once('-').unwrap();
        ranges.push((start.parse::<u64>()?, end.parse::<u64>()?));
    }

    ranges.sort_unstable_by_key(|it| it.0);
    let mut merged_ranges = Vec::with_capacity(ranges.len());
    merged_ranges.push(ranges[0]);
    for &(start, end) in &ranges {
        let last: &mut (u64, u64) = merged_ranges.last_mut().unwrap();
        if start <= last.1 {
            last.1 = last.1.max(end);
        } else {
            merged_ranges.push((start, end));
        }
    }
    let mut result = 0;
    for &(start, end) in &merged_ranges {
        result += (start..=end).count();
    }
    Ok(result)
}

fn main() -> Result<()> {
    start_day(DAY);

    let args: Vec<String> = std::env::args().collect();
    let part = args.get(1).map(|s| s.as_str()).unwrap_or("both");
    let run_part1 = part == "1" || part == "both";
    let run_part2 = part == "2" || part == "both";

    //region Part 1
    if run_part1 {
        println!("=== Part 1 ===");

        assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

        let input_file = BufReader::new(File::open(INPUT_FILE)?);
        let result = time_snippet!(part1(input_file)?);
        println!("Result = {}", result);
    }
    //endregion

    //region Part 2
    if run_part2 {
        println!("\n=== Part 2 ===");

        assert_eq!(14, part2(BufReader::new(TEST.as_bytes()))?);

        let input_file = BufReader::new(File::open(INPUT_FILE)?);
        let result = time_snippet!(part2(input_file)?);
        println!("Result = {}", result);
    }
    //endregion

    Ok(())
}
