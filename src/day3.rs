use std::{fs::File, io::{Error, Read}};

const WIDTH: usize = 12;
const LENGTH: usize = 1000;

fn read_input(path: &str) -> Result<String, Error> {
    let mut file_handle = File::open(path)?;
    let mut content = String::new();
    file_handle.read_to_string(&mut content)?;
    Ok(content)
}

fn get_most_common_bits(nums: Vec<usize>) -> Vec<usize> {
    nums
        .iter()
        .fold(vec![0; WIDTH], |counts, bits| {
            counts
                .into_iter()
                .enumerate()
                .map(|(i, curr)| curr + ((bits & 1 << i) >> i))
                .collect()
        })
}

fn part_1(input: &str) -> usize {
    let nums:Vec<usize> = input.lines()
        .map(|line| usize::from_str_radix(line.trim(), 2).unwrap())
        .collect();
    let counts = get_most_common_bits(nums);
    let gamma: usize = counts
        .iter()
        .enumerate()
        .map(|(i, &count)| ((count >= LENGTH / 2) as usize) << i)
        .sum();
    let epsilon: usize = counts
        .iter()
        .enumerate()
        .map(|(i, &count)| ((count <= LENGTH / 2) as usize) << i)
        .sum();
    gamma * epsilon
}


fn part_2(input: &str) -> usize {
    let nums: Vec<usize> = input.lines()
        .map(|line| usize::from_str_radix(line.trim(), 2).unwrap())
        .collect();
    let oxy = (0..WIDTH)
    .rev()
    .scan(nums.clone(), |oxy, i| {
        let one = oxy.iter().filter(|n| *n & 1 << i > 0).count() >= (oxy.len() + 1) / 2;
        oxy.drain_filter(|n| (*n & 1 << i > 0) != one);
        oxy.first().copied()
    })
    .last()
    .unwrap();

    let co2 = (0..WIDTH)
        .rev()
        .scan(nums, |co2, i| {
            let one = co2.iter().filter(|n| *n & 1 << i > 0).count() >= (co2.len() + 1) / 2;
            co2.drain_filter(|n| (*n & 1 << i > 0) == one);
            co2.first().copied()
        })
        .last()
        .unwrap();

    oxy * co2
}


pub fn run() {
    let input = read_input("inputs/day3.txt").unwrap();
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

#[cfg(test)]
mod tests{
    use super::*;
    
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input = read_input("inputs/day3.txt").unwrap();
        b.iter(|| {
            part_1(&input)
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = read_input("inputs/day3.txt").unwrap();
        b.iter(||{
            part_2(&input)
        })
    }
}
