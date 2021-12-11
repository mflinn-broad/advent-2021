use crate::util;

pub fn run() {
    let raw_input = util::read_input("inputs/day11.txt").unwrap();
    let input = process(&raw_input);
    println!("Part 1: {}", part_1(input));
}

#[derive(Debug)]
struct Octopus {
    e_level: u32,
    flashed_on_step: bool,
}

struct Position {
    row: usize,
    col: usize
}

impl Octopus {
    fn new(e_level: u32) -> Self {
        Octopus {
            e_level,
            flashed_on_step: false,
        }
    }

    fn increase(&mut self) {
        self.e_level += 1;
    }
}

fn get_adjacent_octopi(pos: Position, grid: &Vec<Vec<Octopus>>) -> Vec<Position> {
    let mut adjacents: Vec<Position> = Vec::new();
    if pos.row != 0 {
        let position = Position {
            row: pos.row - 1,
            col: pos.col
        };
        adjacents.push(position);
    }
    if pos.col != 0 {
        let position = Position {
            row: pos.row,
            col: pos.col - 1
        };
        adjacents.push(position);
    }
    if pos.row != (grid.len() - 1) {
        let position = Position {
            row: pos.row + 1,
            col: pos.col
        };
        adjacents.push(position);
    }
    if pos.col != (grid[0].len() - 1) {
        let position = Position {
            row: pos.row,
            col: pos.col + 1
        };
        adjacents.push(position);
    }
    if pos.row != 0 && pos.col != 0 {
        let position = Position {
            row: pos.row - 1,
            col: pos.col - 1,
        };
        adjacents.push(position);
    }

    if pos.row != 0 && pos.col != (grid[0].len() - 1) {
        let position = Position {
            row: pos.row - 1,
            col: pos.col + 1,
        };
        adjacents.push(position);
    }

    if pos.row != (grid.len() - 1) && pos.col != 0 {
        let position = Position {
            row: pos.row + 1,
            col: pos.col - 1,
        };
        adjacents.push(position);
    }

    if pos.row != (grid.len() - 1) && pos.col != (grid[0].len() - 1) {
        let position = Position {
            row: pos.row + 1,
            col: pos.col + 1,
        };
        adjacents.push(position);
    }

    adjacents
}

fn propagate_flash(pos: Position, grid: &mut Vec<Vec<Octopus>>) {
    let mut flash_stack: Vec<Position> = vec![pos];
    while let Some(flash_pos) = flash_stack.pop() {
        let adjacents = get_adjacent_octopi(flash_pos, &grid);
        adjacents.iter()
            .for_each(|adjacent| {
                if !grid[adjacent.row][adjacent.col].flashed_on_step {
                    grid[adjacent.row][adjacent.col].increase();
                    if grid[adjacent.row][adjacent.col].e_level > 9 {
                        grid[adjacent.row][adjacent.col].flashed_on_step = true;
                        flash_stack.push(Position {
                            row: adjacent.row,
                            col: adjacent.col,
                        });
                    }
                }
            })
    }
}

fn part_1(input: Vec<Vec<Octopus>>) -> usize {
    let mut input = input;
    let mut flash_counter: usize = 0;
    for step in 0..100 {
        for row in 0..input.len() {
            for col in 0..input[0].len() {
                input[row][col].increase();
            }
        }
        for row in 0..input.len() {
            for col in 0..input[0].len() {
                if input[row][col].e_level > 9 && !input[row][col].flashed_on_step {
                    let pos = Position { row, col };
                    input[row][col].flashed_on_step = true;
                    propagate_flash(pos, &mut input);
                }
            }
        }
        for row in 0..input.len() {
            for col in 0..input[0].len() {
                if input[row][col].flashed_on_step {
                    flash_counter += 1;
                    input[row][col].e_level = 0;
                    input[row][col].flashed_on_step = false;
                }
            }
        }
    }
    flash_counter
}

fn process(input: &str) -> Vec<Vec<Octopus>> {
    
    input.lines()
        .map(|line| {
            line.chars()
                .map(|ch| {
                    let e_level = ch.encode_utf8(&mut [0; 1]).parse().unwrap();
                    Octopus::new(e_level)
                })
                .collect()
        })
        .collect()
}
