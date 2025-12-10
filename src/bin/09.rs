use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use num_traits::abs;
use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

fn part1<R: BufRead>(reader: R) -> Result<i64> {
    let coords = reader
        .lines()
        .map(|it| {
            let line = it.unwrap();
            let split = line.split_once(',').unwrap();
            (
                split.0.parse::<i64>().unwrap(),
                split.1.parse::<i64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let mut max_val = i64::MIN;
    for i in 0..coords.len() {
        let start = coords[i];
        for j in i + 1..coords.len() {
            let end = coords[j];
            let distance = (abs(start.0 - end.0) + 1) * (abs(start.1 - end.1) + 1);
            if distance > max_val {
                max_val = distance;
            }
        }
    }
    Ok(max_val)
}

fn get_poly_x_range(y: i64, vs: &[(i64, i64)]) -> Option<(i64, i64)> {
    let count = vs.len();
    let mut x_crossings: Vec<i64> = Vec::new();

    for i in 0..count {
        let (cur_x, cur_y) = vs[i];
        let (next_x, next_y) = vs[(i + 1) % count];

        if cur_x == next_x {
            let min_edge_y = min(cur_y, next_y);
            let max_edge_y = max(cur_y, next_y);
            if min_edge_y <= y && y <= max_edge_y {
                x_crossings.push(cur_x);
            }
        } else if cur_y == y {
            x_crossings.push(min(cur_x, next_x));
            x_crossings.push(max(cur_x, next_x));
        }
    }

    if x_crossings.is_empty() {
        return None;
    }
    x_crossings.sort();
    Some((*x_crossings.first().unwrap(), *x_crossings.last().unwrap()))
}

fn is_rect_inside(
    rect_min_x: i64,
    rect_max_x: i64,
    rect_min_y: i64,
    rect_max_y: i64,
    vs: &[(i64, i64)],
) -> bool {
    let mut critical_ys: Vec<i64> = vec![rect_min_y, rect_max_y];
    for (_, vy) in vs {
        if *vy > rect_min_y && *vy < rect_max_y {
            critical_ys.push(*vy);
        }
    }
    critical_ys.sort();
    critical_ys.dedup();

    for y in critical_ys {
        if let Some((poly_min_x, poly_max_x)) = get_poly_x_range(y, vs) {
            if rect_min_x < poly_min_x || rect_max_x > poly_max_x {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn part2<R: BufRead>(reader: R) -> Result<i64> {
    let coords = reader
        .lines()
        .map(|it| {
            let line = it.unwrap();
            let split = line.split_once(',').unwrap();
            (
                split.0.parse::<i64>().unwrap(),
                split.1.parse::<i64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let mut max_area = i64::MIN;
    for i in 0..coords.len() {
        let start = coords[i];
        for j in i + 1..coords.len() {
            let end = coords[j];
            let rect_min_x = min(start.0, end.0);
            let rect_max_x = max(start.0, end.0);
            let rect_min_y = min(start.1, end.1);
            let rect_max_y = max(start.1, end.1);

            if !is_rect_inside(rect_min_x, rect_max_x, rect_min_y, rect_max_y, &coords) {
                continue;
            }
            let distance = (rect_max_x - rect_min_x + 1) * (rect_max_y - rect_min_y + 1);
            if distance > max_area {
                max_area = distance;
            }
        }
    }
    Ok(max_area)
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

        assert_eq!(50, part1(BufReader::new(TEST.as_bytes()))?);

        let input_file = BufReader::new(File::open(INPUT_FILE)?);
        let result = time_snippet!(part1(input_file)?);
        println!("Result = {}", result);
    }
    //endregion

    //region Part 2
    if run_part2 {
        println!("\n=== Part 2 ===");

        assert_eq!(24, part2(BufReader::new(TEST.as_bytes()))?);

        let input_file = BufReader::new(File::open(INPUT_FILE)?);
        let result = time_snippet!(part2(input_file)?);
        println!("Result = {}", result);
    }
    //endregion

    Ok(())
}
