use crate::util;

pub fn run() {
    let raw_input = util::read_input("inputs/day7.txt").unwrap();
    let input = process(&raw_input);

    println!("Part 1: {}", part_1(&mut input.clone()));
    println!("Part 2: {}", part_2(&input));
}

fn process(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|pos| pos.parse().unwrap())
        .collect()
}

fn part_1(input: &mut Vec<usize>) -> usize {
    input.sort();

    let median = input[(input.len() / 2) + (input.len() % 2)];
    cost_at(median, &input)
}

fn part_2(input: &Vec<usize>) -> usize {
    let mean: usize = input.iter().sum::<usize>() / input.len();
    (mean - 5..mean + 5)
        .map(|pos| cost_at_v2(pos, input))
        .min()
        .unwrap()
}

fn cost_at(pos: usize, crabs: &Vec<usize>) -> usize {
    crabs.iter().map(|crab_pos| pos.abs_diff(*crab_pos)).sum()
}

fn cost_at_v2(pos: usize, crabs: &Vec<usize>) -> usize {
    crabs
        .iter()
        .map(|crab_pos| {
            let abs_dist = pos.abs_diff(*crab_pos);
            abs_dist * (1 + abs_dist) / 2
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
        let raw_input = util::read_input("inputs/day7.txt").unwrap();
        b.iter(|| {
            let mut input = process(&raw_input);
            part_1(&mut input);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let raw_input = util::read_input("inputs/day7.txt").unwrap();
        b.iter(|| {
            let input = process(&raw_input);
            part_2(&input);
        });
    }
}
