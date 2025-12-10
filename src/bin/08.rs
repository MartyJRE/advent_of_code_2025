use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let px = self.find(x);
        let py = self.find(y);
        if px == py {
            return;
        }
        if self.rank[px] < self.rank[py] {
            self.parent[px] = py;
        } else if self.rank[px] > self.rank[py] {
            self.parent[py] = px;
        } else {
            self.parent[py] = px;
            self.rank[px] += 1;
        }
    }
}

#[derive(Copy, Clone)]
struct JB {
    x: i64,
    y: i64,
    z: i64,
}

impl JB {
    fn new(x: i64, y: i64, z: i64) -> Self {
        JB { x, y, z }
    }

    fn distance(self, other: &JB) -> u64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz) as u64
    }
}

fn part1<R: BufRead>(reader: R, con_count: usize) -> Result<u64> {
    let boxes = reader
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let mut coords = line.split(',').map(|s| s.parse::<i64>().unwrap());
            JB::new(
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
            )
        })
        .collect::<Vec<JB>>();

    let n = boxes.len();

    let mut pairs: Vec<(u64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            pairs.push((boxes.get(i).unwrap().distance(&boxes[j]), i, j));
        }
    }

    pairs.sort_by_key(|p| p.0);

    let mut uf = UnionFind::new(n);
    for (_, i, j) in pairs.iter().take(con_count) {
        uf.union(*i, *j);
    }

    let mut circuit_sizes = HashMap::new();
    for i in 0..n {
        let root = uf.find(i);
        *circuit_sizes.entry(root).or_insert(0) += 1;
    }

    let mut sizes = circuit_sizes.values().cloned().collect::<Vec<_>>();
    sizes.sort_by(|a, b| b.cmp(a));

    Ok(sizes.iter().take(3).product())
}

fn part2<R: BufRead>(reader: R) -> Result<i64> {
    let boxes = reader
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let mut coords = line.split(',').map(|s| s.parse::<i64>().unwrap());
            JB::new(
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
            )
        })
        .collect::<Vec<JB>>();

    let n = boxes.len();

    let mut pairs: Vec<(u64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            pairs.push((boxes[i].distance(&boxes[j]), i, j));
        }
    }

    pairs.sort_by_key(|p| p.0);

    let mut uf = UnionFind::new(n);
    let mut num_circuits = n;

    for (_, i, j) in pairs.iter() {
        let pi = uf.find(*i);
        let pj = uf.find(*j);
        if pi != pj {
            uf.union(*i, *j);
            num_circuits -= 1;
            if num_circuits == 1 {
                return Ok(boxes[*i].x * boxes[*j].x);
            }
        }
    }

    Ok(0)
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

        assert_eq!(40, part1(BufReader::new(TEST.as_bytes()), 10)?);

        let input_file = BufReader::new(File::open(INPUT_FILE)?);
        let result = time_snippet!(part1(input_file, 1000)?);
        println!("Result = {}", result);
    }
    //endregion

    //region Part 2
    if run_part2 {
        println!("\n=== Part 2 ===");

        assert_eq!(25272, part2(BufReader::new(TEST.as_bytes()))?);

        let input_file = BufReader::new(File::open(INPUT_FILE)?);
        let result = time_snippet!(part2(input_file)?);
        println!("Result = {}", result);
    }
    //endregion

    Ok(())
}
