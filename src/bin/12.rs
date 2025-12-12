use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "12";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

#[derive(Clone, PartialEq, Eq)]
struct Shape {
    cells: Vec<(i32, i32)>,
}

impl Shape {
    fn from_grid(grid: &[Vec<u8>]) -> Self {
        let mut cells = Vec::new();
        for (r, row) in grid.iter().enumerate() {
            for (c, &cell) in row.iter().enumerate() {
                if cell == 1 {
                    cells.push((r as i32, c as i32));
                }
            }
        }
        Self { cells }.normalized()
    }

    fn normalized(mut self) -> Self {
        if self.cells.is_empty() {
            return self;
        }
        let min_r = self.cells.iter().map(|&(r, _)| r).min().unwrap();
        let min_c = self.cells.iter().map(|&(_, c)| c).min().unwrap();
        self.cells = self
            .cells
            .iter()
            .map(|&(r, c)| (r - min_r, c - min_c))
            .collect();
        self.cells.sort();
        self
    }

    fn rotate_90(&self) -> Self {
        Self {
            cells: self.cells.iter().map(|&(r, c)| (c, -r)).collect(),
        }
        .normalized()
    }

    fn flip_horizontal(&self) -> Self {
        Self {
            cells: self.cells.iter().map(|&(r, c)| (r, -c)).collect(),
        }
        .normalized()
    }

    fn all_orientations(&self) -> Vec<Shape> {
        let mut orientations = Vec::new();
        let mut current = self.clone().normalized();

        for _ in 0..4 {
            if !orientations.contains(&current) {
                orientations.push(current.clone());
            }
            current = current.rotate_90();
        }

        current = self.flip_horizontal();
        for _ in 0..4 {
            if !orientations.contains(&current) {
                orientations.push(current.clone());
            }
            current = current.rotate_90();
        }

        orientations
    }

    fn len(&self) -> usize {
        self.cells.len()
    }

    fn can_place(
        &self,
        grid: &[Vec<bool>],
        row: i32,
        col: i32,
        height: usize,
        width: usize,
    ) -> bool {
        for &(dr, dc) in &self.cells {
            let r = row + dr;
            let c = col + dc;
            if r < 0 || r >= height as i32 || c < 0 || c >= width as i32 {
                return false;
            }
            if grid[r as usize][c as usize] {
                return false;
            }
        }
        true
    }

    fn place(&self, grid: &mut [Vec<bool>], row: i32, col: i32) {
        for &(dr, dc) in &self.cells {
            grid[(row + dr) as usize][(col + dc) as usize] = true;
        }
    }

    fn unplace(&self, grid: &mut [Vec<bool>], row: i32, col: i32) {
        for &(dr, dc) in &self.cells {
            grid[(row + dr) as usize][(col + dc) as usize] = false;
        }
    }
}

fn count_empty(grid: &[Vec<bool>]) -> usize {
    grid.iter()
        .flat_map(|row| row.iter())
        .filter(|&&c| !c)
        .count()
}

fn solve(
    grid: &mut Vec<Vec<bool>>,
    shapes_to_place: &[Vec<Shape>],
    idx: usize,
    height: usize,
    width: usize,
    remaining_cells: usize,
) -> bool {
    if idx == shapes_to_place.len() {
        return true;
    }

    // Pruning: check if remaining shapes can even fit
    let empty = count_empty(grid);
    if empty < remaining_cells {
        return false;
    }

    let orientations = &shapes_to_place[idx];
    let shape_size = orientations[0].len();

    for row in 0..height as i32 {
        for col in 0..width as i32 {
            for orientation in orientations {
                if orientation.can_place(grid, row, col, height, width) {
                    orientation.place(grid, row, col);
                    if solve(
                        grid,
                        shapes_to_place,
                        idx + 1,
                        height,
                        width,
                        remaining_cells - shape_size,
                    ) {
                        return true;
                    }
                    orientation.unplace(grid, row, col);
                }
            }
        }
    }

    false
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

        fn part1<R: BufRead>(reader: R) -> Result<usize> {
            let mut shapes_grid = Vec::new();
            let mut regions = Vec::new();

            let mut lines = reader.lines();
            while let Some(line) = lines.next() {
                let line = line?;
                if let Some((digits, rest)) = line.split_once(':') {
                    if !rest.is_empty() {
                        let (width, height) = digits.split_once('x').unwrap();
                        let width = width.parse::<usize>()?;
                        let height = height.parse::<usize>()?;
                        let counts = rest.trim().split_whitespace().enumerate();
                        regions.push((
                            width,
                            height,
                            counts
                                .map(|(idx, num)| (idx, num.parse::<usize>().unwrap()))
                                .collect::<Vec<_>>(),
                        ));
                        continue;
                    }
                    let idx = digits.parse::<usize>()?;
                    assert_eq!(idx, shapes_grid.len());
                    let mut shape = Vec::new();
                    while let Some(next_line) = lines.next() {
                        let next_line = next_line?;
                        if next_line.is_empty() {
                            break;
                        }
                        let mut row = Vec::with_capacity(next_line.len());
                        for ch in next_line.chars() {
                            row.push(match ch {
                                '#' => 1,
                                '.' => 0,
                                _ => panic!("This should never happen"),
                            });
                        }
                        shape.push(row);
                    }
                    shapes_grid.push(shape);
                }
            }

            let shapes = shapes_grid
                .iter()
                .map(|grid| Shape::from_grid(grid).all_orientations())
                .collect::<Vec<_>>();

            let mut answer = 0;

            for (width, height, shape_indices) in regions {
                let mut shapes_to_place = Vec::new();
                for (idx, count) in shape_indices {
                    for _ in 0..count {
                        shapes_to_place.push(shapes[idx].clone());
                    }
                }

                shapes_to_place.sort_by(|a, b| b[0].len().cmp(&a[0].len()));

                let total_cells: usize = shapes_to_place.iter().map(|s| s[0].len()).sum();
                let mut grid = vec![vec![false; width]; height];
                if solve(&mut grid, &shapes_to_place, 0, height, width, total_cells) {
                    answer += 1;
                }
            }

            Ok(answer)
        }

        assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

        let input_file = BufReader::new(File::open(INPUT_FILE)?);
        let result = time_snippet!(part1(input_file)?);
        println!("Result = {}", result);
    }
    //endregion

    //region Part 2
    if run_part2 {
        println!("\n=== Part 2 ===");

        fn part2() {
            // no part 2 :(
        }

        time_snippet!(part2());
    }
    //endregion

    Ok(())
}
