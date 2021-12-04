use crate::util;

type CalledNumbers = Vec<u64>;

#[derive(Debug)]
struct BingoCard {
    card: Vec<Vec<Cell>>,
    winning_turn: Option<usize>,
}

impl BingoCard {
    fn new(rows: &Vec<Vec<u64>>) -> Self {
        let card: Vec<Vec<Cell>> = rows
            .iter()
            .map(|line| {
                line
                    .iter()
                    .map(|val| Cell::new(*val) )
                    .collect()
            })
            .collect();
        
        BingoCard { 
            card, 
            winning_turn: None, 
        }
    }

    fn check_rows(&self) -> bool {
        for row in self.card.iter() {
            let winning_row = row
                .iter()
                .all(|cell| cell.state == State::Marked);
            if winning_row {
                return true;
            }
        }
        false
    }

    fn check_cols(&self) -> bool {
        for col in 0..5 {
            let winning_col = (0..5)
                .all(|row| self.card[row][col].state == State::Marked);
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
                return
            }
        }
    }

    fn mark(&mut self, num: u64) {
        self.card
            .iter_mut()
            .for_each(|row| {
                row
                    .iter_mut()
                    .for_each(|cell|{
                        if cell.val == num {
                            cell.state = State::Marked;
                        }
                    })
            })
    }

    fn score(&self) -> u64 {
        self.card
            .iter()
            .fold(0, |score, row| {
                let mut row_score = 0;
                row
                    .iter()
                    .for_each(|cell| {
                        if cell.state == State::Unmarked {
                            row_score += cell.val
                        }
                    });
                score + row_score
            })
    }
}

#[derive(Debug)]
struct Cell {
    val: u64,
    state: State,
}

impl Cell {
    fn new(val: u64) -> Self {
        Cell {
            val,
            state: State::Unmarked
        }
    }
}

#[derive(Debug,PartialEq)]
enum State {
    Marked,
    Unmarked,
}

pub fn run() {
    let raw_input = util::read_input("inputs/day4.txt").unwrap();
    let mut input = process(&raw_input);
    println!("Part 1: {:#?}", part_1(&mut input.1, input.0));
}

fn process(input: &str) -> (CalledNumbers, Vec<BingoCard>) {
    let mut chunks = input
        .split("\n\n");

    let called_numbers = process_header(chunks.next().unwrap());
    let cards: Vec<BingoCard> = chunks
        .map(|card| process_card(card))
        .collect();
    (called_numbers, cards)
}

fn process_header(header: &str) -> CalledNumbers {
    header
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect()
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

fn part_1(cards: &mut Vec<BingoCard>, nums: Vec<u64>) -> u64 {
    cards
        .iter_mut()
        .for_each(|card| card.play(&nums));
    let mut winners: Vec<&BingoCard> = cards
        .iter()
        .filter(|card| card.winning_turn != None)
        .collect();
    winners.sort_by(|&card_a, &card_b| {
        let card_a_win_turn = card_a.winning_turn.unwrap();
        let card_b_win_ture = card_b.winning_turn.unwrap();
        card_a_win_turn.cmp(&card_b_win_ture)
    });
    let winner_score = winners[0].score();
    let winning_turn = winners[0].winning_turn.unwrap();
    let winning_number = nums[winning_turn];
    winner_score * winning_number
}

#[cfg(test)]
mod tests{
    use super::*;
    extern crate test;
    use test::Bencher;

    const TEST_DATA: &'static str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7"#;

    #[test]
    fn test_part_1() {
        let input = process(TEST_DATA);
        assert_eq!(27, input.0.len());
        assert_eq!(3, input.1.len());
    }
}
