use crate::util;
use pathfinding::directed::dijkstra;

const SURROUNDING_POINTS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

pub fn run() {
    let raw_input = util::read_input("inputs/day15.txt").unwrap();
    let input = process(&raw_input);
    println!("Part 1: {}", part_1(input.clone()));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: Vec<Vec<i32>>) -> i32 {
    let start = (0, 0);
    let goal = (input.len() as i32 - 1, input[0].len() as i32 - 1);

    dijkstra::dijkstra(
        &start,
        |(x, y)| {
            SURROUNDING_POINTS
                .iter()
                .map(|(x_move, y_move)| {
                    input
                        .get((x + x_move) as usize)
                        .and_then(|row| row.get((y + y_move) as usize))
                        .map(|risk| ((x + x_move, y + y_move), *risk))
                })
                .flatten()
                .collect::<Vec<_>>()
        },
        |&node| node == goal,
    )
    .unwrap()
    .1
}

fn part_2(input: Vec<Vec<i32>>) -> i32 {
    let square_size = input.len();
    let start = (0, 0);
    let goal = (square_size as i32 * 5 - 1, square_size as i32 * 5 - 1);

    dijkstra::dijkstra(
        &start,
        |(x, y)| {
            SURROUNDING_POINTS
                .iter()
                .map(|(x_move, y_move)| ((x + x_move) as usize, (y + y_move) as usize))
                .filter(|(x, y)| (x / 5 < square_size && y / 5 < square_size))
                .map(|(x, y)| {
                    input
                        .get(x % square_size)
                        .and_then(|row| row.get(y % square_size))
                        .map(|risk| {
                            (
                                (x as i32, y as i32),
                                ((*risk as usize + (x / square_size) + (y / square_size) - 1) % 9
                                    + 1) as i32,
                            )
                        })
                })
                .flatten()
                .collect::<Vec<_>>()
        },
        |&node| node == goal,
    )
    .unwrap()
    .1
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
