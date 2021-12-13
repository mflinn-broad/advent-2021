use advent_2021::days::*;
use itertools::Itertools;
use std::time::{Duration, Instant};
use std::collections::HashMap;

fn main() {
    let start = Instant::now();
    let mut day_durations: HashMap<usize, Duration> = HashMap::new();
    let day_1_start = Instant::now();
    println!("Day 1 --------");
    day1::run();
    day_durations.insert(1, day_1_start.elapsed());

    let day_2_start = Instant::now();
    println!("Day 2 --------");
    day2::run();
    day_durations.insert(2, day_2_start.elapsed());

    let day_3_start = Instant::now();
    println!("Day 3 --------");
    day3::run();
    day_durations.insert(3, day_3_start.elapsed());

    let day_4_start = Instant::now();
    println!("Day 4 --------");
    day4::run();
    day_durations.insert(4, day_4_start.elapsed());

    let day_5_start = Instant::now();
    println!("Day 5 --------");
    day5::run();
    day_durations.insert(5, day_5_start.elapsed());

    let day_6_start = Instant::now();
    println!("Day 6 --------");
    day6::run();
    day_durations.insert(6, day_6_start.elapsed());

    let day_7_start = Instant::now();
    println!("Day 7 --------");
    day7::run();
    day_durations.insert(7, day_7_start.elapsed());

    let day_8_start = Instant::now();
    println!("Day 8 --------");
    day8::run();
    day_durations.insert(8, day_8_start.elapsed());

    let day_9_start = Instant::now();
    println!("Day 9 --------");
    day9::run();
    day_durations.insert(9, day_9_start.elapsed());

    let day_10_start = Instant::now();
    println!("Day 10 --------");
    day10::run();
    day_durations.insert(10, day_10_start.elapsed());

    let day_11_start = Instant::now();
    println!("Day 11 --------");
    day11::run();
    day_durations.insert(11, day_11_start.elapsed());

    let day_12_start = Instant::now();
    println!("Day 12 --------");
    day12::run();
    day_durations.insert(12, day_12_start.elapsed());

    let day_13_start = Instant::now();
    println!("Day 13 --------");
    day13::run();
    day_durations.insert(13, day_13_start.elapsed());

    let total_duration = start.elapsed();

    println!("Total time to run all solutions: {:?}", total_duration);

    day_durations.iter()
        .sorted_by(|(_, dur_1), (_, dur_2)| dur_2.cmp(dur_1))
        .for_each(|(day, dur)| {
            println!("Duration to run day {}: {:?}", day, dur);
        })
}
