use crate::util;
use pathfinding::directed::dijkstra;

const SURROUNDING_POINTS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

pub fn run() {
    let raw_input = util::read_input("inputs/day15.txt").unwrap();
    let input = process(&raw_input);
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &[Vec<i32>]) -> i32 {
    let start = (0, 0);
    let goal = (input.len() as i32 - 1, input[0].len() as i32 - 1);

    dijkstra::dijkstra(
        &start,
        |curr| successors_small(curr, input),
        |&node| node == goal,
    )
    .unwrap()
    .1
}

fn successors_small(current: &(i32, i32), grid: &[Vec<i32>]) -> Vec<((i32, i32), i32)> {
    let (x, y) = current;
    SURROUNDING_POINTS
        .iter()
        .map(|(x_move, y_move)| {
            grid.get((x + x_move) as usize)
                .and_then(|row| row.get((y + y_move) as usize))
                .map(|risk| ((x + x_move, y + y_move), *risk))
        })
        .flatten()
        .collect::<Vec<_>>()

}

fn part_2(input: &[Vec<i32>]) -> i32 {
    let square_size = input.len();
    let start = (0, 0);
    let goal = (square_size as i32 * 5 - 1, square_size as i32 * 5 - 1);

    dijkstra::dijkstra(
        &start,
        |curr| successors_large(curr, input),
        |&node| node == goal,
    )
    .unwrap()
    .1
}

fn successors_large(current: &(i32, i32), grid: &[Vec<i32>]) -> Vec<((i32, i32), i32)> {
    let (x, y) = current;
    let square_size = grid.len();
    SURROUNDING_POINTS
        .iter()
        .map(|(x_move, y_move)| ((x + x_move) as usize, (y + y_move) as usize))
        .filter(|(x, y)| (x / 5 < square_size && y / 5 < square_size))
        .map(|(x, y)| {
            grid.get(x % square_size)
                .and_then(|row| row.get(y % square_size))
                .map(|risk| {
                    (
                        (x as i32, y as i32),
                        ((*risk as usize + (x / square_size) + (y / square_size) - 1) % 9 + 1)
                            as i32,
                    )
                })
        })
        .flatten()
        .collect::<Vec<_>>()
}

fn process(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|ch| ch.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let raw_input = util::read_input("inputs/day15.txt").unwrap();
        b.iter(|| {
            let input = process(&raw_input);
            part_1(&input);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let raw_input = util::read_input("inputs/day15.txt").unwrap();
        b.iter(|| {
            let input = process(&raw_input);
            part_2(&input);
        });
    }
}
