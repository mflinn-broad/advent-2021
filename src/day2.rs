use std::{fs::File, io::{Error, Read}};

fn read_input(path: &str) -> Result<String, Error> {
    let mut file_handle = File::open(path)?;
    let mut content = String::new();
    file_handle.read_to_string(&mut content)?;
    Ok(content)
}

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
            _ => panic!()
        }
    }
}

fn process_input(input: String) -> Vec<(Direction, i64)> {
    input.lines()
        .map(|line| {
            let cell: Vec<&str> = line.trim().split_whitespace().collect();
            (Direction::from(cell[0]), cell[1].parse().unwrap())
        }) 
        .collect()
}

fn part_1(input: Vec<(Direction, i64)>) -> i64 {   
    let position = input.iter()
        .fold((0, 0), |acc, (dir, amt) | {
            match dir {
                Direction::Forward => (acc.0 + amt, acc.1),
                Direction::Down => (acc.0, acc.1 + amt),
                Direction::Up => (acc.0, acc.1 - amt),
            }
        });
    position.0 * position.1
}


pub fn run() {
    let input = read_input("inputs/day2.txt");
    let input = process_input(input.unwrap());
    println!("part 1: {}", part_1(input));
}
