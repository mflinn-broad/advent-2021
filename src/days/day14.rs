use crate::util;
use std::collections::HashMap;

type Input = (
    HashMap<String, String>,
    HashMap<String, usize>,
    HashMap<String, usize>,
);

pub fn run() {
    let raw_input = util::read_input("inputs/day14.txt").unwrap();
    let input = process(&raw_input);
    println!("Part 1: {}", part_1(input));
}

fn part_1(input: Input) -> usize {
    let (mappings, mut singles, pairs) = input;
    (0..10).fold(pairs, |old_pairs, _| {
        let mut new_pairs: HashMap<String, usize> = HashMap::new();

        old_pairs.iter().for_each(|(pair, pair_count)| {
            let single = mappings.get(pair).unwrap();
            let single_count = singles.get_mut(single).unwrap();
            *single_count += pair_count;

            let new_pair_a = format!("{}{}", pair.chars().nth(0).unwrap(), single);
            let new_pair_b = format!("{}{}", single, pair.chars().nth(1).unwrap());

            let new_pair_a_count = new_pairs.entry(new_pair_a).or_insert(0);
            *new_pair_a_count += pair_count;

            let new_pair_b_count = new_pairs.entry(new_pair_b).or_insert(0);
            *new_pair_b_count += pair_count;
        });

        new_pairs
    });

    let min = singles.values().min().unwrap();
    let max = singles.values().max().unwrap();

    max - min
}

fn process(input: &str) -> Input {
    let mut singles: HashMap<String, usize> = HashMap::new();
    let mut pairs: HashMap<String, usize> = HashMap::new();
    let mut mappings: HashMap<String, String> = HashMap::new();

    let (initial, instructions) = input.split_once("\n\n").unwrap();

    initial.trim().chars().for_each(|poly| {
        let poly_count = singles.entry(poly.to_string()).or_insert(0);
        *poly_count += 1;
    });

    let initial: Vec<char> = initial.trim().chars().collect();
    initial.windows(2).for_each(|pair| {
        let pair: String = pair.iter().collect();
        let pair_count = pairs.entry(pair).or_insert(0);
        *pair_count += 1;
    });

    instructions.lines().for_each(|line| {
        let (pair, poly) = line.trim().split_once(" -> ").unwrap();
        mappings.insert(pair.to_string(), poly.to_string());
    });

    (mappings, singles, pairs)
}
