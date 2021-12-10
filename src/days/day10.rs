use itertools::Itertools;

use crate::util;
use std::collections::HashMap;

pub fn run() {
    let raw_input = util::read_input("inputs/day10.txt").unwrap();
    let input = process(&raw_input);

    println!("Part 1: {:?}", part_1(&input));
    println!("Part 2: {:?}", part_2(&input));
}

fn process(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part_1(input: &Vec<Vec<char>>) -> usize {
    let mut error_tracker: HashMap<char, usize> = HashMap::new();
    input.iter().for_each(|line| {
        let mut stack: Vec<char> = Vec::new();
        for symbol in line {
            match symbol {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                '<' => stack.push('>'),
                x if *x == stack[stack.len() - 1] => {
                    stack.pop();
                }
                y => {
                    let error_entry = error_tracker.entry(*y).or_insert(0);
                    *error_entry += 1;
                    return;
                }
            }
        }
    });

    error_tracker
        .iter()
        .fold(0, |score, (symbol, count)| match symbol {
            ')' => score + (count * 3),
            ']' => score + (count * 57),
            '}' => score + (count * 1197),
            '>' => score + (count * 25137),
            _ => panic!("unreachable"),
        })
}

fn part_2(input: &Vec<Vec<char>>) -> usize {
    // remove corrupted and complete lines
    let completion_scores: Vec<usize> = input
        .iter()
        .map(|line| {
            let mut stack: Vec<char> = Vec::new();
            for symbol in line {
                match symbol {
                    '(' => stack.push(')'),
                    '[' => stack.push(']'),
                    '{' => stack.push('}'),
                    '<' => stack.push('>'),
                    x if x == stack.last().unwrap() => {
                        stack.pop();
                    }
                    _ => {
                        return Vec::new();
                    }
                }
            }
            stack
        })
        .filter(|completion| !completion.is_empty())
        .map(|completion| {
            completion.iter().rev().fold(0, |score, symbol| match symbol {
                ')' => (score * 5) + 1,
                ']' => (score * 5) + 2,
                '}' => (score * 5) + 3,
                '>' => (score * 5) + 4,
                _ => panic!("unreachable"),
            })
        })
        .sorted()
        .collect();

    completion_scores[completion_scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let raw_input = util::read_input("inputs/day10.txt").unwrap();
        b.iter(|| {
            let mut input = process(&raw_input);
            part_1(&mut input);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let raw_input = util::read_input("inputs/day10.txt").unwrap();
        b.iter(|| {
            let input = process(&raw_input);
            part_2(&input);
        });
    }
}
