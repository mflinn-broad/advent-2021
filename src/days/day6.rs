use std::thread::current;

use crate::util;

pub fn run() {
    let raw_input = util::read_input("inputs/day6.txt").unwrap();
    let starting_statte = process(&raw_input);
    println!("{:?}", starting_statte);
    println!("Part 1: {:?}", part_1(starting_statte));

}

fn process(input: &str) -> [u64; 9] {
    let mut tracker: [u64; 9] = [0; 9]; 
    input.trim().split(',')
        .map(|day_str| day_str.parse::<u64>().unwrap())
        .for_each(|day| tracker[day as usize] += 1);
    tracker
}

fn part_1(state: [u64; 9]) -> usize {
    let final_state: [u64; 9] = (1..=80)
        .fold(state, |mut curr_state, _| {
            let mut temp_state: [u64; 9] = [0; 9];
            temp_state.copy_from_slice(&curr_state);

            for bucket in 1..9 {
                curr_state[bucket-1] = temp_state[bucket];
            }

            curr_state[6] += temp_state[0];
            curr_state[8] = temp_state[0];
            println!("{:?}", curr_state);
            curr_state
        });
    final_state.iter().map(|count| *count as usize).sum()
}
