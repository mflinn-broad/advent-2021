use std::cmp::Ordering;

use crate::util;

type CalledNumbers = Vec<u64>;

#[derive(Debug, Clone)]
struct BingoCard {
    card: Vec<Vec<Cell>>,
    winning_turn: Option<usize>,
}

impl BingoCard {
    fn new(rows: &Vec<Vec<u64>>) -> Self {
        let card: Vec<Vec<Cell>> = rows
            .iter()
            .map(|line| line.iter().map(|val| Cell::new(*val)).collect())
            .collect();

        BingoCard {
            card,
            winning_turn: None,
        }
    }

    fn check_rows(&self) -> bool {
        for row in self.card.iter() {
            let winning_row = row.iter().all(|cell| cell.state == State::Marked);
            if winning_row {
                return true;
            }
        }
        false
    }

    fn check_cols(&self) -> bool {
        for col in 0..5 {
            let winning_col = (0..5).all(|row| self.card[row][col].state == State::Marked);
            if winning_col {
                return true;
            }
        }
        false
    }

    fn check_win(&self) -> bool {
        self.check_cols() || self.check_rows()
    }

    fn play(&mut self, nums: &CalledNumbers) {
        for (turn, num) in nums.iter().enumerate() {
            self.mark(*num);
            if self.check_win() {
                self.winning_turn = Some(turn);
                return;
            }
        }
    }

    fn mark(&mut self, num: u64) {
        self.card.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|cell| {
                if cell.val == num {
                    cell.state = State::Marked;
                }
            })
        })
    }

    fn score(&self) -> u64 {
        self.card.iter().fold(0, |score, row| {
            let mut row_score = 0;
            row.iter().for_each(|cell| {
                if cell.state == State::Unmarked {
                    row_score += cell.val
                }
            });
            score + row_score
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Cell {
    val: u64,
    state: State,
}

impl Cell {
    fn new(val: u64) -> Self {
        Cell {
            val,
            state: State::Unmarked,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum State {
    Marked,
    Unmarked,
}

pub fn run() {
    let raw_input = util::read_input("inputs/day4.txt").unwrap();
    let mut input = process(&raw_input);
    println!(
        "Part 1: {:#?}",
        part_1(&mut input.1.clone(), input.0.clone())
    );
    println!("Part 2: {:#?}", part_2(&mut input.1, input.0))
}

fn process(input: &str) -> (CalledNumbers, Vec<BingoCard>) {
    let mut chunks = input.split("\n\n");

    let called_numbers = process_header(chunks.next().unwrap());
    let cards: Vec<BingoCard> = chunks.map(|card| process_card(card)).collect();
    (called_numbers, cards)
}

fn process_header(header: &str) -> CalledNumbers {
    header.split(',').map(|num| num.parse().unwrap()).collect()
}

fn process_card(card: &str) -> BingoCard {
    let card_data: Vec<Vec<u64>> = card
        .lines()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();

    BingoCard::new(&card_data)
}

fn part_1(cards: &mut Vec<BingoCard>, nums: CalledNumbers) -> u64 {
    calculate_games(cards, nums, |a, b| {
        let a_winning_turn = a.winning_turn.unwrap();
        let b_winning_turn = b.winning_turn.unwrap();
        a_winning_turn.cmp(&b_winning_turn)
    })
}

fn part_2(cards: &mut Vec<BingoCard>, nums: CalledNumbers) -> u64 {
    calculate_games(cards, nums, |a, b| {
        let a_winning_turn = a.winning_turn.unwrap();
        let b_winning_turn = b.winning_turn.unwrap();
        b_winning_turn.cmp(&a_winning_turn)
    })
}

fn calculate_games<F>(cards: &mut Vec<BingoCard>, nums: CalledNumbers, sort_func: F) -> u64
where
    F: FnMut(&&BingoCard, &&BingoCard) -> Ordering,
{
    cards.iter_mut().for_each(|card| card.play(&nums));
    let mut winners: Vec<&BingoCard> = cards
        .iter()
        .filter(|card| card.winning_turn != None)
        .collect();
    winners.sort_by(sort_func);
    let winner_score = winners[0].score();
    let winning_turn = winners[0].winning_turn.unwrap();
    let winning_number = nums[winning_turn];
    winner_score * winning_number
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let raw_input = util::read_input("inputs/day4.txt").unwrap();
        b.iter(|| {
            let mut input = process(&raw_input);
            part_1(&mut input.1, input.0);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let raw_input = util::read_input("inputs/day4.txt").unwrap();
        b.iter(|| {
            let mut input = process(&raw_input);
            part_2(&mut input.1, input.0);
        });
    }
}
