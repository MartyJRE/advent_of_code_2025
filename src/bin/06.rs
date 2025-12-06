use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<_>>();
        let ops = lines.last().unwrap().split_whitespace();
        let rows = (&lines[..lines.len() - 1])
            .iter()
            .map(|it| {
                it.split_whitespace()
                    .map(|it| it.parse::<u64>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut result = 0u64;
        for (idx, op) in ops.enumerate() {
            let mut sub_result = *rows.first().unwrap().get(idx).unwrap();
            for row in &rows[1..] {
                let val = *row.get(idx).unwrap();
                match op.chars().next().unwrap() {
                    '+' => {
                        sub_result += val;
                    }
                    '*' => {
                        sub_result *= val;
                    }
                    bad_op => panic!("unknown operation {bad_op}"),
                }
            }
            result += sub_result;
        }
        Ok(result)
    }

    assert_eq!(4277556, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u64> {
        let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<_>>();
        let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);

        let lines: Vec<String> = lines
            .iter()
            .map(|s| format!("{:width$}", s, width = max_len))
            .collect();

        let op_row = &lines[lines.len() - 1];
        let data_rows = &lines[..lines.len() - 1];

        let is_separator: Vec<bool> = (0..max_len)
            .map(|i| {
                data_rows
                    .iter()
                    .all(|row| row.chars().nth(i).unwrap() == ' ')
            })
            .collect();

        let mut result = 0u64;
        let mut current_cols: Vec<usize> = vec![];
        let mut current_op: Option<char> = None;

        for i in 0..=max_len {
            let is_sep = i == max_len || is_separator[i];

            if is_sep && !current_cols.is_empty() {
                let numbers: Vec<u64> = current_cols
                    .iter()
                    .rev()
                    .filter_map(|&col_idx| {
                        let num_str: String = data_rows
                            .iter()
                            .filter_map(|row| {
                                let c = row.chars().nth(col_idx)?;
                                if c.is_ascii_digit() {
                                    Some(c)
                                } else {
                                    None
                                }
                            })
                            .collect();
                        if num_str.is_empty() {
                            None
                        } else {
                            num_str.parse().ok()
                        }
                    })
                    .collect();

                let problem_result: u64 = match current_op.unwrap() {
                    '+' => numbers.iter().sum(),
                    '*' => numbers.iter().product(),
                    op => panic!("Unknown operator {}", op),
                };
                result += problem_result;
                current_cols.clear();
                current_op = None;
            } else if !is_sep {
                current_cols.push(i);
                let op_char = op_row.chars().nth(i).unwrap();
                if op_char == '+' || op_char == '*' {
                    current_op = Some(op_char);
                }
            }
        }

        Ok(result)
    }

    assert_eq!(3263827, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
