use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let mut res = 0;
        let banks = reader.lines().flatten().collect::<Vec<_>>();
        for bank in &banks {
            let actual_bank = bank
                .chars()
                .map(|c| c.to_digit(10).expect("only digits allowed") as i32)
                .collect::<Vec<_>>();
            let last_possible = bank.len() - 2;
            let mut max = i32::MIN;
            for i in 0..=last_possible {
                for j in i + 1..=last_possible + 1 {
                    let tens = *actual_bank.get(i).expect("must be within bounds");
                    let ones = *actual_bank.get(j).expect("must be within bounds");
                    let answer = tens * 10 + ones;
                    if answer > max {
                        max = answer;
                    }
                }
            }
            res += max;
        }
        Ok(res)
    }

    assert_eq!(357, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i64> {
        let mut res = 0;
        let banks = reader.lines().flatten().collect::<Vec<_>>();
        for bank in &banks {
            let mut possible_positions = bank.len() - 12;
            let actual_bank = bank
                .chars()
                .map(|c| c.to_digit(10).expect("only digits allowed") as i64)
                .collect::<Vec<_>>();
            let mut stack = Vec::<i64>::with_capacity(bank.len());
            for digit in &actual_bank {
                while !stack.is_empty()
                    && stack.last().expect("stack is not empty") < digit
                    && possible_positions > 0
                {
                    stack.pop();
                    possible_positions -= 1;
                }
                stack.push(*digit);
            }
            let digits_of_stack = stack.iter().take(12).collect::<Vec<_>>();
            let mut answer = 0;
            for idx in 0..digits_of_stack.len() {
                let digit = *digits_of_stack.get(idx).expect("must be within bounds");
                let inverse = (digits_of_stack.len() - idx - 1) as u32;
                answer += digit * 10i64.pow(inverse);
            }
            res += answer;
        }
        Ok(res)
    }

    assert_eq!(3121910778619, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
