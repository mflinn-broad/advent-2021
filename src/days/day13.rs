use itertools::Itertools;

use crate::util;

pub fn run() {
    let raw_input = util::read_input("inputs/day13.txt").unwrap();
    let input = process(&raw_input);
    println!("Part 1: {}", part_1((&input.0, &input.1)));
    println!("Part 2: \n{}\n", part_2((input.0, &input.1)));
}

fn part_1(input: (&Vec<Point>, &Vec<Fold>)) -> usize {
    let (points, folds) = input;
    compute_fold(points, folds[0]).iter().count()
}

fn part_2(input: (Vec<Point>, &Vec<Fold>)) -> String {
    let (points, folds) = input;
    let final_points: Vec<Point> = folds.iter()
        .fold(points, |acc, fold| {
            compute_fold(&acc, *fold)
        });

    let max_x = final_points.iter()
        .max_by(|a, b| a.x.cmp(&b.x)).unwrap().x;
    let max_y = final_points.iter()
    .max_by(|a, b| a.y.cmp(&b.y)).unwrap().y;
    
    let mut grid: Vec<Vec<char>> = vec![vec![' '; max_x + 1]; max_y + 1];
    final_points.iter().for_each(|point| grid[point.y][point.x] = '#');

    let grid: Vec<String> = grid.iter()
        .map(|line| line.iter().collect())
        .collect();
    
    grid.join("\n")

}

#[derive(Debug, Clone, Copy)]
struct Fold {
    axis: Axis,
    pos: usize,
}

#[derive(Debug, Clone, Copy)]
enum Axis {
    X,
    Y,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Point {
    x: usize,
    y: usize
}

fn compute_fold(points: &Vec<Point>, fold: Fold) -> Vec<Point> {
   let mapped_points: Vec<Point> = match fold.axis {
        Axis::X => {
            points.iter()
                .filter(|point| point.x != fold.pos)
                .map(|point| {
                    Point {
                        x: fold.pos - (fold.pos.abs_diff(point.x)),
                        y: point.y
                    }
                })
                .collect()
        }
        Axis::Y => {
            points.iter()
                .filter(|point| point.y != fold.pos)
                .map(|point| {
                    Point {
                        x: point.x,
                        y: fold.pos - (fold.pos.abs_diff(point.y))
                    }
                })
                .collect()
        }
    };

    mapped_points.into_iter()
        .unique()
        .collect()
}

fn process(input: &str) -> (Vec<Point>, Vec<Fold>) {
    let (dots_str, folds_str) = input.split_once("\n\n").unwrap();
    let dots: Vec<Point> = dots_str.lines()
        .map(|line| {
            let (x, y) = line.trim().split_once(',').unwrap();
            Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect();
    
    let folds: Vec<Fold> = folds_str.lines()
        .map(|line| {
            line.trim().split_whitespace().nth(2)
        })
        .map(|fold_str| {
            let fold_str = fold_str.unwrap();
            let (axis, pos) = fold_str.split_once('=').unwrap();
            if axis == "x" {
                Fold {
                    axis: Axis::X,
                    pos: pos.parse().unwrap(),
                }
            } else {
                Fold {
                    axis: Axis::Y,
                    pos: pos.parse().unwrap(),
                }
            }
        })
        .collect();
    
    (dots, folds)
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let raw_input = util::read_input("inputs/day13.txt").unwrap();
        b.iter(|| {
            let input = process(&raw_input);
            part_1((&input.0, &input.1));
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let raw_input = util::read_input("inputs/day13.txt").unwrap();
        b.iter(|| {
            let input = process(&raw_input);
            part_2((input.0, &input.1));
        });
    }
}
