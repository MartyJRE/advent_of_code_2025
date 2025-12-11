use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST1: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

const TEST2: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

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
            let mut tree = BTreeMap::new();
            for line in reader.lines() {
                let line = line?;
                let mut splits = line.split_whitespace();
                let mut name = splits.next().unwrap().chars();
                name.next_back();
                let name = name.as_str();
                tree.insert(
                    String::from(name),
                    splits.map(|it| String::from(it)).collect::<Vec<_>>(),
                );
            }
            let mut stack = vec![tree.get_key_value("you").unwrap()];
            let mut outs = 0usize;
            while let Some((_, values)) = stack.pop() {
                for value in values {
                    let found = tree.get_key_value(value);
                    if let Some(found) = found {
                        stack.push(found);
                    } else {
                        outs += 1;
                    }
                }
            }
            Ok(outs)
        }

        assert_eq!(5, part1(BufReader::new(TEST1.as_bytes()))?);

        let input_file = BufReader::new(File::open(INPUT_FILE)?);
        let result = time_snippet!(part1(input_file)?);
        println!("Result = {}", result);
    }
    //endregion

    //region Part 2
    if run_part2 {
        println!("\n=== Part 2 ===");

        fn part2<R: BufRead>(reader: R) -> Result<usize> {
            let mut tree = BTreeMap::new();
            for line in reader.lines() {
                let line = line?;
                let mut splits = line.split_whitespace();
                let mut name = splits.next().unwrap().chars();
                name.next_back();
                let name = name.as_str();
                tree.insert(
                    String::from(name),
                    splits.map(|it| String::from(it)).collect::<Vec<_>>(),
                );
            }

            let mut cache = HashMap::new();

            fn dfs(
                node: &str,
                visited_dac: bool,
                visited_fft: bool,
                tree: &BTreeMap<String, Vec<String>>,
                cache: &mut HashMap<(String, bool, bool), usize>,
            ) -> usize {
                let key = (node.to_string(), visited_dac, visited_fft);
                if let Some(&result) = cache.get(&key) {
                    return result;
                }

                let result = if let Some(children) = tree.get(node) {
                    let new_dac = visited_dac || node == "dac";
                    let new_fft = visited_fft || node == "fft";
                    children
                        .iter()
                        .map(|c| dfs(c, new_dac, new_fft, tree, cache))
                        .sum()
                } else {
                    if visited_dac && visited_fft {
                        1
                    } else {
                        0
                    }
                };

                cache.insert(key, result);
                result
            }

            Ok(dfs("svr", false, false, &tree, &mut cache))
        }

        assert_eq!(2, part2(BufReader::new(TEST2.as_bytes()))?);

        let input_file = BufReader::new(File::open(INPUT_FILE)?);
        let result = time_snippet!(part2(input_file)?);
        println!("Result = {}", result);
    }
    //endregion

    Ok(())
}
