use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse_line(line: &str) -> (Vec<u8>, Vec<Vec<usize>>) {
        let bracket_start = line.find('[').unwrap();
        let bracket_end = line.find(']').unwrap();
        let target_str = &line[bracket_start + 1..bracket_end];
        let target: Vec<u8> = target_str
            .chars()
            .map(|c| if c == '#' { 1 } else { 0 })
            .collect();

        let buttons_section = &line[bracket_end + 1..];
        let buttons_end = buttons_section.find('{').unwrap_or(buttons_section.len());
        let buttons_str = &buttons_section[..buttons_end];

        let mut buttons: Vec<Vec<usize>> = Vec::new();
        let mut i = 0;
        let chars: Vec<char> = buttons_str.chars().collect();
        while i < chars.len() {
            if chars[i] == '(' {
                let start = i + 1;
                while i < chars.len() && chars[i] != ')' {
                    i += 1;
                }
                let inner = &buttons_str[start..i];
                let indices: Vec<usize> = inner
                    .split(',')
                    .filter(|s| !s.trim().is_empty())
                    .map(|s| s.trim().parse().unwrap())
                    .collect();
                buttons.push(indices);
            }
            i += 1;
        }

        (target, buttons)
    }

    fn solve_machine(target: &[u8], buttons: &[Vec<usize>]) -> usize {
        let n_lights = target.len();
        let n_buttons = buttons.len();

        if n_buttons == 0 {
            return if target.iter().all(|&x| x == 0) {
                0
            } else {
                usize::MAX
            };
        }

        let mut matrix: Vec<Vec<u8>> = vec![vec![0; n_buttons + 1]; n_lights];

        for (col, button) in buttons.iter().enumerate() {
            for &light in button {
                if light < n_lights {
                    matrix[light][col] = 1;
                }
            }
        }
        for (row, &t) in target.iter().enumerate() {
            matrix[row][n_buttons] = t;
        }

        let mut pivot_cols: Vec<usize> = Vec::new();
        let mut row = 0;
        for col in 0..n_buttons {
            let mut pivot_row = None;
            for r in row..n_lights {
                if matrix[r][col] == 1 {
                    pivot_row = Some(r);
                    break;
                }
            }

            if let Some(pr) = pivot_row {
                matrix.swap(row, pr);
                pivot_cols.push(col);

                for r in 0..n_lights {
                    if r != row && matrix[r][col] == 1 {
                        for c in 0..=n_buttons {
                            matrix[r][c] ^= matrix[row][c];
                        }
                    }
                }
                row += 1;
            }
        }

        for r in row..n_lights {
            if matrix[r][n_buttons] == 1 {
                return usize::MAX;
            }
        }

        let free_cols: Vec<usize> = (0..n_buttons).filter(|c| !pivot_cols.contains(c)).collect();

        let n_free = free_cols.len();
        let mut min_presses = usize::MAX;

        for mask in 0..(1u64 << n_free) {
            let mut solution = vec![0u8; n_buttons];

            for (i, &col) in free_cols.iter().enumerate() {
                solution[col] = ((mask >> i) & 1) as u8;
            }

            for (pivot_row, &pivot_col) in pivot_cols.iter().enumerate().rev() {
                let mut val = matrix[pivot_row][n_buttons];
                for c in (pivot_col + 1)..n_buttons {
                    val ^= matrix[pivot_row][c] * solution[c];
                }
                solution[pivot_col] = val;
            }

            let presses: usize = solution.iter().map(|&x| x as usize).sum();
            min_presses = min_presses.min(presses);
        }

        min_presses
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut total = 0;
        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            let (target, buttons) = parse_line(&line);
            let presses = solve_machine(&target, &buttons);
            if presses == usize::MAX {
                return Err(anyhow!("No solution found for line: {}", line));
            }
            total += presses;
        }
        Ok(total)
    }

    assert_eq!(7, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn parse_line_p2(line: &str) -> (Vec<i64>, Vec<Vec<usize>>) {
        let bracket_end = line.find(']').unwrap();
        let buttons_section = &line[bracket_end + 1..];

        let brace_start = buttons_section.find('{').unwrap();
        let brace_end = buttons_section.find('}').unwrap();
        let joltage_str = &buttons_section[brace_start + 1..brace_end];
        let target: Vec<i64> = joltage_str
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();

        let buttons_str = &buttons_section[..brace_start];
        let mut buttons: Vec<Vec<usize>> = Vec::new();
        let mut i = 0;
        let chars: Vec<char> = buttons_str.chars().collect();
        while i < chars.len() {
            if chars[i] == '(' {
                let start = i + 1;
                while i < chars.len() && chars[i] != ')' {
                    i += 1;
                }
                let inner = &buttons_str[start..i];
                let indices: Vec<usize> = inner
                    .split(',')
                    .filter(|s| !s.trim().is_empty())
                    .map(|s| s.trim().parse().unwrap())
                    .collect();
                buttons.push(indices);
            }
            i += 1;
        }

        (target, buttons)
    }

    fn solve_joltage(target: &[i64], buttons: &[Vec<usize>]) -> i64 {
        let n_counters = target.len();
        let n_buttons = buttons.len();

        if n_buttons == 0 {
            return if target.iter().all(|&x| x == 0) {
                0
            } else {
                i64::MAX
            };
        }

        let mut matrix: Vec<Vec<i64>> = vec![vec![0; n_buttons + 1]; n_counters];
        for (col, button) in buttons.iter().enumerate() {
            for &counter in button {
                if counter < n_counters {
                    matrix[counter][col] = 1;
                }
            }
        }
        for (row, &t) in target.iter().enumerate() {
            matrix[row][n_buttons] = t;
        }

        fn gcd(a: i64, b: i64) -> i64 {
            if b == 0 { a.abs() } else { gcd(b, a % b) }
        }

        let mut pivot_cols: Vec<usize> = Vec::new();
        let mut pivot_rows: Vec<usize> = Vec::new();
        let mut row = 0;

        for col in 0..n_buttons {
            let mut pivot_row = None;
            for r in row..n_counters {
                if matrix[r][col] != 0 {
                    pivot_row = Some(r);
                    break;
                }
            }

            if let Some(pr) = pivot_row {
                matrix.swap(row, pr);
                pivot_cols.push(col);
                pivot_rows.push(row);

                let pivot_val = matrix[row][col];
                for r in 0..n_counters {
                    if r != row && matrix[r][col] != 0 {
                        let factor = matrix[r][col];
                        for c in 0..=n_buttons {
                            matrix[r][c] = matrix[r][c] * pivot_val - matrix[row][c] * factor;
                        }
                        let row_gcd = matrix[r].iter().map(|&x| x.abs()).filter(|&x| x > 0).fold(0, gcd);
                        if row_gcd > 1 {
                            for c in 0..=n_buttons {
                                matrix[r][c] /= row_gcd;
                            }
                        }
                    }
                }
                row += 1;
            }
        }

        for r in row..n_counters {
            if matrix[r][n_buttons] != 0 {
                return i64::MAX;
            }
        }

        let free_cols: Vec<usize> = (0..n_buttons).filter(|c| !pivot_cols.contains(c)).collect();
        let n_free = free_cols.len();

        if n_free > 20 {
            return i64::MAX;
        }

        fn compute_solution(
            free_vals: &[i64],
            free_cols: &[usize],
            pivot_cols: &[usize],
            pivot_rows: &[usize],
            matrix: &[Vec<i64>],
            n_buttons: usize,
        ) -> Option<Vec<i64>> {
            let mut solution = vec![0i64; n_buttons];
            for (i, &col) in free_cols.iter().enumerate() {
                solution[col] = free_vals[i];
            }

            for (&pivot_row, &pivot_col) in pivot_rows.iter().zip(pivot_cols.iter()).rev() {
                let mut val = matrix[pivot_row][n_buttons];
                for c in (pivot_col + 1)..n_buttons {
                    val -= matrix[pivot_row][c] * solution[c];
                }
                let divisor = matrix[pivot_row][pivot_col];
                if val % divisor != 0 {
                    return None;
                }
                solution[pivot_col] = val / divisor;
            }

            if solution.iter().any(|&x| x < 0) {
                return None;
            }
            Some(solution)
        }

        let max_free: i64 = *target.iter().max().unwrap_or(&0);
        let mut min_presses = i64::MAX;

        fn enumerate_free(
            idx: usize,
            free_vals: &mut Vec<i64>,
            free_cols: &[usize],
            pivot_cols: &[usize],
            pivot_rows: &[usize],
            matrix: &[Vec<i64>],
            n_buttons: usize,
            max_free: i64,
            min_presses: &mut i64,
        ) {
            if idx == free_cols.len() {
                if let Some(solution) = compute_solution(free_vals, free_cols, pivot_cols, pivot_rows, matrix, n_buttons) {
                    let presses: i64 = solution.iter().sum();
                    *min_presses = (*min_presses).min(presses);
                }
                return;
            }

            let current_sum: i64 = free_vals.iter().sum();
            if current_sum >= *min_presses {
                return;
            }

            for v in 0..=max_free {
                if current_sum + v >= *min_presses {
                    break;
                }
                free_vals.push(v);
                enumerate_free(idx + 1, free_vals, free_cols, pivot_cols, pivot_rows, matrix, n_buttons, max_free, min_presses);
                free_vals.pop();
            }
        }

        let mut free_vals: Vec<i64> = Vec::new();
        enumerate_free(0, &mut free_vals, &free_cols, &pivot_cols, &pivot_rows, &matrix, n_buttons, max_free, &mut min_presses);

        min_presses
    }

    fn part2<R: BufRead>(reader: R) -> Result<i64> {
        let lines: Vec<String> = reader.lines().flatten().filter(|l| !l.trim().is_empty()).collect();
        let total: i64 = lines
            .par_iter()
            .map(|line| {
                let (target, buttons) = parse_line_p2(line);
                solve_joltage(&target, &buttons)
            })
            .sum();
        Ok(total)
    }

    assert_eq!(33, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
