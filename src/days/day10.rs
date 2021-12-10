use crate::util;
use std::collections::HashMap;

pub fn run() {
    let raw_input = util::read_input("inputs/day10.txt").unwrap();
    let input = process(&raw_input);

    println!("Part 1: {:?}", part_1(input));
}

fn process(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part_1(input: Vec<Vec<char>>) -> usize {
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
