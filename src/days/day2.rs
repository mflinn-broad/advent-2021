use crate::util;

#[derive(Debug)]
enum Direction {
    Forward,
    Down,
    Up,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => panic!(),
        }
    }
}

fn process_input(input: String) -> Vec<(Direction, i64)> {
    input
        .lines()
        .map(|line| {
            let cell: Vec<_> = line.trim().split_whitespace().collect();
            (Direction::from(cell[0]), cell[1].parse().unwrap())
        })
        .collect()
}

fn part_1(input: &Vec<(Direction, i64)>) -> i64 {
    let position = input.iter().fold((0, 0), |acc, (dir, amt)| match dir {
        Direction::Forward => (acc.0 + amt, acc.1),
        Direction::Down => (acc.0, acc.1 + amt),
        Direction::Up => (acc.0, acc.1 - amt),
    });
    position.0 * position.1
}

fn part_2(input: &Vec<(Direction, i64)>) -> i64 {
    let coordinates = input.iter().fold((0, 0, 0), |acc, (dir, amt)| match dir {
        Direction::Forward => (acc.0 + amt, acc.1 + amt * acc.2, acc.2),
        Direction::Down => (acc.0, acc.1, acc.2 + amt),
        Direction::Up => (acc.0, acc.1, acc.2 - amt),
    });
    coordinates.0 * coordinates.1
}

pub fn run() {
    let input = util::read_input("inputs/day2.txt");
    let input = process_input(input.unwrap());
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input = process_input(util::read_input("inputs/day2.txt").unwrap());
        b.iter(|| part_1(&input));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = process_input(util::read_input("inputs/day2.txt").unwrap());
        b.iter(|| part_2(&input))
    }
}
