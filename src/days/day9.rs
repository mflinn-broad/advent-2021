use crate::util;

pub fn run() {
    let raw_input = util::read_input("inputs/day9.txt").unwrap();
    let input = process(&raw_input);
    println!("Part 1: {:?}", part_1(&input));
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
    adjacents.iter()
        .all(|val| target < *val)
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

fn part_1(input: &Vec<Vec<u8>>) -> u32 {
    input.iter()
        .enumerate()
        .fold(0, |risk_score, (row_idx, row)| {
            risk_score + row.iter()
                .enumerate()
                .fold(0, |row_risk, (col, height)| {
                    let adjacents = get_adjacents(input, (row_idx, col));
                    if is_local_minima(*height, adjacents) {
                        row_risk + (*height as u32) + 1
                    } else {
                        row_risk
                    }
                })
        })
}
