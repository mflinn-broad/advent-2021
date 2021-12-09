use crate::util;
use itertools::Itertools;

pub fn run() {
    let raw_input = util::read_input("inputs/day8.txt").unwrap();
    let input = process(&raw_input);
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn process(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    input
        .lines()
        .map(|line| {
            let (input, output) = line.trim().split_once(" | ").unwrap();
            (
                input.split_whitespace().collect(),
                output.split_whitespace().collect(),
            )
        })
        .collect()
}

fn part_1(input: &Vec<(Vec<&str>, Vec<&str>)>) -> u32 {
    let unique_segment_counts = vec![2, 3, 4, 7];
    let outputs: Vec<Vec<&str>> = input.iter().map(|line| line.1.clone()).collect();
    outputs
        .iter()
        .map(|out| {
            out.iter()
                .filter(|digit| unique_segment_counts.contains(&digit.len()))
                .collect()
        })
        .fold(0, |unique_segment_counter, uniques: Vec<&&str>| {
            unique_segment_counter + uniques.len() as u32
        })
}

fn part_2(input: &Vec<(Vec<&str>, Vec<&str>)>) -> u32 {
    input
        .iter()
        .map(|(inputs, output)| {
            let mut patterns = vec![""; 10];
            // check for 1, 4, 7, and 8. remove from inputs and store in mapping tracker
            let mut inputs = inputs.clone();
            inputs.retain(|input| match input.len() {
                2 => {
                    patterns[1] = input;
                    false
                }
                3 => {
                    patterns[7] = input;
                    false
                }
                4 => {
                    patterns[4] = input;
                    false
                }
                7 => {
                    patterns[8] = input;
                    false
                }
                _ => true,
            });
            // extract 3  and 9
            inputs.retain(|input| {
                let contains_4 = patterns[4].chars().all(|segment| input.contains(segment));
                let contains_7 = patterns[7].chars().all(|segment| input.contains(segment));
                let contains_1 = patterns[1].chars().all(|segment| input.contains(segment));
                match input.len() {
                    6 if contains_4 && contains_7 => {
                        patterns[9] = input;
                        false
                    }
                    5 if contains_1 => {
                        patterns[3] = input;
                        false
                    }
                    _ => true,
                }
            });
            // check for 2
            inputs.retain(|input| {
                if input.len() == 5 && !input.chars().all(|segment| patterns[9].contains(segment)) {
                    patterns[2] = input;
                    return false;
                }
                true
            });
            // 5 is the only thing left with length 5
            inputs.retain(|input| {
                if input.len() == 5 {
                    patterns[5] = input;
                    return false;
                }
                true
            });

            // 6 will contain 5 and isn't also 9
            inputs.retain(|input| {
                let contains_5 = patterns[5].chars().all(|segment| input.contains(segment));
                match input.len() {
                    6 if input != &patterns[9] && contains_5 => {
                        patterns[6] = input;
                        false
                    }
                    _ => true,
                }
            });

            // 0 is the only one left
            patterns[0] = inputs.first().unwrap();
            let patterns: Vec<String> = patterns
                .iter()
                .map(|digit| digit.chars().sorted().collect::<String>())
                .collect();
            output
                .iter()
                .fold("".to_string(), |num, digit| {
                    let digit_numeric = patterns
                        .iter()
                        .position(|pos| digit.chars().sorted().collect::<String>() == *pos);
                    if let Some(digit_num) = digit_numeric {
                        return num + &digit_num.to_string();
                    }
                    num
                })
                .parse::<u32>()
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let raw_input = util::read_input("inputs/day8.txt").unwrap();
        b.iter(|| {
            let mut input = process(&raw_input);
            part_1(&mut input);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let raw_input = util::read_input("inputs/day8.txt").unwrap();
        b.iter(|| {
            let input = process(&raw_input);
            part_2(&input);
        });
    }
}
