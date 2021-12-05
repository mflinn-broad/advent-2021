use crate::util;
use std::cmp;
use std::collections::HashMap;



type Point = (u64, u64);
type Line = (Point, Point);
type LineList = Vec<(Point, Point)>;

enum LineType {
    Vertical,
    Horizontal,
    Diagonal,
}

pub fn run() {
    let raw_input = util::read_input("inputs/day5.txt").unwrap();
    let input = process(&raw_input);
    println!("Part 1: {:?}", part_1(input.clone()));
    println!("Part 2: {:?}", part_2(input));
}

// this is ugly but it works
fn process(input: &str) -> LineList {
    input
        .lines()
        .map(|line| {
            let mut points_iter = line.trim().split(" -> ").map(|point_str| {
                let mut point_iter = point_str.split(',').map(|val| val.parse::<u64>().unwrap());
                (point_iter.next().unwrap(), point_iter.next().unwrap())
            });
            (points_iter.next().unwrap(), points_iter.next().unwrap())
        })
        .collect()
}

fn part_1(input: LineList) -> usize {
    let lines = get_verticals_and_horizontals(input);
    let mut point_tracker: HashMap<Point, usize> = HashMap::new();
    lines.iter().for_each(|line| {
        match line_type(line) {
            LineType::Vertical => update_vertical(&line, &mut point_tracker),
            LineType::Horizontal => update_horizontal(&line, &mut point_tracker),
            _ => panic!("unreachable"),
        }
    });
    count_overlaps(&point_tracker)
}

fn part_2(lines: LineList) -> usize {
    let mut point_tracker: HashMap<Point, usize> = HashMap::new();
    lines.iter().for_each(|line| {
        match line_type(line) {
            LineType::Vertical => update_vertical(&line, &mut point_tracker),
            LineType::Horizontal => update_horizontal(&line, &mut point_tracker),
            _ => update_diagonal(&line, &mut point_tracker),
        }
    });
    count_overlaps(&point_tracker)
}

fn count_overlaps(counter: &HashMap<Point, usize>) -> usize {
    counter.iter()
        .filter(|(_, count)| **count >= 2)
        .count()
}

fn get_verticals_and_horizontals(lines: LineList) -> LineList {
    lines
        .iter()
        .filter(|(start, end)| start.0 == end.0 || start.1 == end.1)
        .copied()
        .collect()
}

fn line_type(line: &Line) -> LineType {
    let (start, end) = line;
    if start.0 == end.0 {
        return LineType::Vertical;
    } else if start.1 == end.1 {
        return LineType::Horizontal;
    }
    LineType::Diagonal
}

fn update_diagonal(line: &Line, counter: &mut HashMap<Point, usize>) {
    let (a, b) = line;
    let min_x = cmp::min_by(a, b, |a, b| a.0.cmp(&b.0));
    let max_x = cmp::max_by(a, b, |a, b| a.0.cmp(&b.0));

    let mut x = min_x.0;
    let mut y = min_x.1;

    if min_x.1 < max_x.1 {
        while x <= max_x.0 {
            let point = counter.entry((x, y)).or_insert(0);
            *point += 1;
            x += 1;
            y += 1;
        }
    } else {
        while x <= max_x.0 {
            let point = counter.entry((x, y)).or_insert(0);
            *point += 1;
            x += 1;
            y -= 1;
        }
    }
}

fn update_vertical(line: &Line, counter: &mut HashMap<Point, usize>) {
    let (a, b) = line;
    let start = std::cmp::min(a.1, b.1);
    let end = std::cmp::max(a.1, b.1);
    for y in start..=end {
        let point = counter.entry((a.0, y)).or_insert(0);
        *point += 1;
    }
}

fn update_horizontal(line: &Line, counter: &mut HashMap<Point, usize>) {
    let (a, b) = line;
    let start = cmp::min(a.0, b.0);
    let end = cmp::max(a.0, b.0);
    for x in start..=end {
        let point = counter.entry((x, a.1)).or_insert(0);
        *point += 1;
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let raw_input = util::read_input("inputs/day5.txt").unwrap();
        b.iter(|| {
            let input = process(&raw_input);
            part_1(input);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let raw_input = util::read_input("inputs/day5.txt").unwrap();
        b.iter(|| {
            let input = process(&raw_input);
            part_2(input);
        });
    }

}
