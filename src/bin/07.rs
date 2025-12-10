use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let grid = reader
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut start: (usize, usize) = (0, 0);
    let mut found = false;
    for (y, row) in grid.iter().enumerate() {
        if found {
            break;
        }
        for (x, val) in row.iter().enumerate() {
            if *val == 'S' {
                start = (x, y);
                found = true;
                break;
            }
        }
    }
    let mut streams = Vec::new();
    streams.push((start.0, start.1 + 1));
    let mut splits = 0;
    let mut split_locs = HashSet::new();
    while let Some((x, y)) = streams.pop() {
        if y == grid.len() - 1 {
            continue;
        }
        if grid[y + 1][x] == '^' {
            if split_locs.contains(&(x, y + 1)) {
                continue;
            }
            split_locs.insert((x, y + 1));
            splits += 1;
            streams.push((x - 1, y + 1));
            streams.push((x + 1, y + 1));
        } else if grid[y + 1][x] == '.' {
            streams.push((x, y + 1));
        }
    }
    Ok(splits)
}

fn part2<R: BufRead>(reader: R) -> Result<u64> {
    let grid = reader
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut start: (usize, usize) = (0, 0);
    let mut found = false;
    for (y, row) in grid.iter().enumerate() {
        if found {
            break;
        }
        for (x, val) in row.iter().enumerate() {
            if *val == 'S' {
                start = (x, y);
                found = true;
                break;
            }
        }
    }
    let mut paths: HashMap<usize, u64> = HashMap::new();
    paths.insert(start.0, 1);

    for y in 1..grid.len() {
        let mut next_paths: HashMap<usize, u64> = HashMap::new();

        for (&x, &count) in &paths {
            match grid[y][x] {
                '^' => {
                    *next_paths.entry(x - 1).or_insert(0) += count;
                    *next_paths.entry(x + 1).or_insert(0) += count;
                }
                '.' => {
                    *next_paths.entry(x).or_insert(0) += count;
                }
                _ => {}
            }
        }
        paths = next_paths;
    }

    Ok(paths.values().sum())
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

        assert_eq!(21, part1(BufReader::new(TEST.as_bytes()))?);

        let input_file = BufReader::new(File::open(INPUT_FILE)?);
        let result = time_snippet!(part1(input_file)?);
        println!("Result = {}", result);
    }
    //endregion

    //region Part 2
    if run_part2 {
        println!("\n=== Part 2 ===");

        assert_eq!(40, part2(BufReader::new(TEST.as_bytes()))?);

        let input_file = BufReader::new(File::open(INPUT_FILE)?);
        let result = time_snippet!(part2(input_file)?);
        println!("Result = {}", result);
    }
    //endregion

    Ok(())
}
