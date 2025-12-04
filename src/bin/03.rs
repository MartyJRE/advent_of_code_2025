use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use num_traits::{CheckedAdd, CheckedMul, FromPrimitive, Pow, Zero};
use std::fmt::Display;
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

    fn parts<R: BufRead, N>(reader: R, len: usize) -> Result<N>
    where
        N: FromPrimitive
            + Zero
            + Ord
            + Copy
            + Display
            + CheckedMul
            + CheckedAdd
            + Pow<usize, Output = N>,
    {
        let mut res = N::zero();
        let banks = reader.lines().flatten().collect::<Vec<_>>();
        for bank in &banks {
            let mut possible_positions = bank.len() - len;
            let actual_bank = bank
                .chars()
                .map(|c| {
                    N::from_u32(c.to_digit(10).expect("only digits allowed")).expect("must convert")
                })
                .collect::<Vec<_>>();
            let mut stack = Vec::with_capacity(bank.len());
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
            let digits_of_stack = stack.iter().take(len).collect::<Vec<_>>();
            let mut answer = N::zero();
            for idx in 0..digits_of_stack.len() {
                let digit = *digits_of_stack.get(idx).expect("must be within bounds");
                let inverse = digits_of_stack.len() - idx - 1;
                answer = answer.add(*digit * N::from_i32(10).unwrap().pow(inverse));
            }
            res = res.add(answer);
        }
        Ok(res)
    }

    assert_eq!(357, parts::<_, u16>(BufReader::new(TEST.as_bytes()), 2)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(parts::<_, u16>(input_file, 2)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    assert_eq!(
        3121910778619,
        parts::<_, u64>(BufReader::new(TEST.as_bytes()), 12)?
    );

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(parts::<_, u64>(input_file, 12)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
