use crate::util;
use std::collections::VecDeque;

pub fn run() {
    let raw_input = util::read_input("inputs/day9.txt").unwrap();
    let input = process(&raw_input);
    println!("Part 1: {:?}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

pub fn process(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|num| num.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_local_minima(target: u8, adjacents: Vec<u8>) -> bool {
    adjacents.iter().all(|val| target < *val)
}

fn get_adjacents(grid: &Vec<Vec<u8>>, pos: (usize, usize)) -> Vec<u8> {
    let (row, col) = pos;
    let mut adjacents: Vec<u8> = Vec::new();
    if row != 0 {
        adjacents.push(grid[row - 1][col]);
    }
    if col != 0 {
        adjacents.push(grid[row][col - 1]);
    }
    if row != (grid.len() - 1) {
        adjacents.push(grid[row + 1][col]);
    }
    if col != (grid[0].len() - 1) {
        adjacents.push(grid[row][col + 1]);
    }
    adjacents
}

fn get_adjacent_points(grid: &Vec<Vec<u8>>, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let (row, col) = pos;
    let mut adjacents: Vec<(usize, usize)> = Vec::new();
    if row != 0 {
        adjacents.push((row - 1, col));
    }
    if col != 0 {
        adjacents.push((row, col - 1));
    }
    if row != (grid.len() - 1) {
        adjacents.push((row + 1, col));
    }
    if col != (grid[0].len() - 1) {
        adjacents.push((row, col + 1));
    }
    adjacents
}

fn get_basin_size(grid: &Vec<Vec<u8>>, pos: (usize, usize)) -> usize {
    let mut size: usize = 0;
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut seen: Vec<(usize, usize)> = Vec::new();
    queue.push_back(pos);
    while let Some(point) = queue.pop_front() {
        size += 1;
        let adjacents = get_adjacent_points(grid, point);
        adjacents.iter().for_each(|(row, col)| {
            let is_increasing = grid[*row][*col] > grid[point.0][point.1];
            let already_seen = seen.contains(&(*row, *col));
            if is_increasing && !already_seen && grid[*row][*col] != 9 {
                seen.push((*row, *col));
                queue.push_back((*row, *col));
            }
        })
    }
    size
}

fn part_1(input: &Vec<Vec<u8>>) -> u32 {
    input
        .iter()
        .enumerate()
        .fold(0, |risk_score, (row_idx, row)| {
            risk_score
                + row.iter().enumerate().fold(0, |row_risk, (col, height)| {
                    let adjacents = get_adjacents(input, (row_idx, col));
                    if is_local_minima(*height, adjacents) {
                        row_risk + (*height as u32) + 1
                    } else {
                        row_risk
                    }
                })
        })
}

fn part_2(input: &Vec<Vec<u8>>) -> usize {
    let basins: Vec<usize> = Vec::new();
    let mut basins = input
        .iter()
        .enumerate()
        .fold(basins, |mut basins, (row_idx, row)| {
            row.iter().enumerate().for_each(|(col, height)| {
                let adjacents = get_adjacents(&input, (row_idx, col));
                if is_local_minima(*height, adjacents) {
                    let basin_size = get_basin_size(input, (row_idx, col));
                    basins.push(basin_size);
                }
            });
            basins
        });

    basins.sort_by(|a, b| b.cmp(a));
    basins.iter().take(3).fold(1, |acc, val| acc * val)
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let raw_input = util::read_input("inputs/day9.txt").unwrap();
        b.iter(|| {
            let mut input = process(&raw_input);
            part_1(&mut input);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let raw_input = util::read_input("inputs/day9.txt").unwrap();
        b.iter(|| {
            let input = process(&raw_input);
            part_2(&input);
        });
    }
}
