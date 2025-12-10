use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "NN"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
<TEST-INPUT>
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    let args: Vec<String> = std::env::args().collect();
    let part = args.get(1).map(|s| s.as_str()).unwrap_or("both");
    let run_part1 = part == "1" || part == "both";
    let _run_part2 = part == "2" || part == "both";

    //region Part 1
    if run_part1 {
        println!("=== Part 1 ===");

        fn part1<R: BufRead>(reader: R) -> Result<usize> {
            // TODO: Solve Part 1 of the puzzle
            let answer = reader.lines().flatten().count();
            Ok(answer)
        }

        // TODO: Set the expected answer for the test input
        assert_eq!(0, part1(BufReader::new(TEST.as_bytes()))?);

        let input_file = BufReader::new(File::open(INPUT_FILE)?);
        let result = time_snippet!(part1(input_file)?);
        println!("Result = {}", result);
    }
    //endregion

    //region Part 2
    // if run_part2 {
    //     println!("\n=== Part 2 ===");
    //
    //     fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //         Ok(0)
    //     }
    //
    //     assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    //     let input_file = BufReader::new(File::open(INPUT_FILE)?);
    //     let result = time_snippet!(part2(input_file)?);
    //     println!("Result = {}", result);
    // }
    //endregion

    Ok(())
}
