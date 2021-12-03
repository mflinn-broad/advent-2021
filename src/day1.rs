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
    get_num_increasing_windows(2, data)
}

fn part_2 (data: &Vec<i64>) -> usize {
    get_num_increasing_windows(4, data)
}

fn get_num_increasing_windows(window_size: usize, data: &Vec<i64>) -> usize {
    data.windows(window_size)
        .filter(|window| window[0] < window[window_size - 1])
        .count()
}

pub fn run() {
    let input = read_input("inputs/day1.txt");
    let data = process_input(input.unwrap());

    println!("part 1: {}", part_1(&data));
    println!("part2: {}", part_2(&data));
}

#[cfg(test)]
mod tests{
    use super::*;
    
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input = process_input(read_input("inputs/day1.txt").unwrap());
        b.iter(|| {
            part_1(&input)
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = process_input(read_input("inputs/day1.txt").unwrap());
        b.iter(||{
            part_2(&input)
        })
    }
}
