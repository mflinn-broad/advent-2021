use std::{fs::File, io::{Error, Read}};

fn read_input(path: &str) -> Result<String, Error> {
    let mut file_handle = File::open(path)?;
    let mut content = String::new();
    file_handle.read_to_string(&mut content)?;
    Ok(content)
}

fn process_input(input: String) -> Vec<i64> {
    input.split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect()
}

fn part_1 (data: &Vec<i64>) -> usize {
    let mut count: usize = 0;
    let last_idx = data.len();
    for i in 1..last_idx {
        if data[i] > data[i-1] {
            count += 1;
        }
    }
    count
}

fn part_2 (data: &Vec<i64>) -> usize {
    data.windows(4)
      .fold(0, |acc, win| if win[0] < win[3] {acc + 1} else {acc})
}

pub fn run() {
    let input = read_input("inputs/day1.txt");
    let data = process_input(input.unwrap());

    println!("part 1: {}", part_1(&data));
    println!("part2: {}", part_2(&data));
}
