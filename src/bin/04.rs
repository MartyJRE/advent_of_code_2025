use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let grid = reader
            .lines()
            .map(|line| line.unwrap().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut possible_to_access = 0;
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                let mut neighbors = 0;
                for y_off in -1..=1 {
                    for x_off in -1..=1 {
                        if y_off == 0 && x_off == 0 {
                            continue;
                        }
                        let actual_y = (y as isize) + y_off;
                        let actual_x = (x as isize) + x_off;
                        if actual_y < 0 || (actual_y as usize) >= grid.len() {
                            continue;
                        }
                        if actual_x < 0 || (actual_x as usize) >= grid[actual_y as usize].len() {
                            continue;
                        }
                        let actual_y = actual_y as usize;
                        let actual_x = actual_x as usize;
                        match grid[actual_y][actual_x] {
                            '@' => neighbors += 1,
                            _ => continue,
                        }
                    }
                }
                if grid[y][x] == '@' && neighbors < 4 {
                    possible_to_access += 1;
                }
            }
        }
        Ok(possible_to_access)
    }

    assert_eq!(13, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut grid = reader
            .lines()
            .map(|line| line.unwrap().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut possible_to_access = 0;
        let mut removed = Vec::<(usize, usize)>::with_capacity(grid.len() * grid[0].len());
        let mut first = true;

        while first || !removed.is_empty() {
            first = false;
            while let Some((x, y)) = removed.pop() {
                grid[y][x] = '.';
            }
            for y in 0..grid.len() {
                for x in 0..grid[y].len() {
                    let mut neighbors = 0;
                    for y_off in -1..=1 {
                        for x_off in -1..=1 {
                            if y_off == 0 && x_off == 0 {
                                continue;
                            }
                            let actual_y = (y as isize) + y_off;
                            let actual_x = (x as isize) + x_off;
                            if actual_y < 0 || (actual_y as usize) >= grid.len() {
                                continue;
                            }
                            if actual_x < 0 || (actual_x as usize) >= grid[actual_y as usize].len()
                            {
                                continue;
                            }
                            let actual_y = actual_y as usize;
                            let actual_x = actual_x as usize;
                            match grid[actual_y][actual_x] {
                                '@' => neighbors += 1,
                                _ => continue,
                            }
                        }
                    }
                    if grid[y][x] == '@' && neighbors < 4 {
                        removed.push((x, y));
                        possible_to_access += 1;
                    }
                }
            }
        }
        Ok(possible_to_access)
    }

    assert_eq!(43, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
