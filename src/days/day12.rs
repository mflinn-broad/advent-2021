use crate::util;
use petgraph::dot::{Config, Dot};
use petgraph::graphmap::UnGraphMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;

pub fn run() {
    let raw_input = util::read_input("inputs/day12.txt").unwrap();
    let input = process(&raw_input);

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));

    make_diagram(&input).unwrap();
}

fn process(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|line| line.trim().split_once("-").unwrap())
        .collect()
}

fn part_1(input: &Vec<(&str, &str)>) -> usize {
    let cave_map = UnGraphMap::<&str, ()>::from_edges(input);
    let mut visit_tracker: HashSet<&str> = HashSet::new();
    visit_tracker.insert("start");

    count_simple_paths(&cave_map, "start", &mut visit_tracker)
}

fn part_2(input: &Vec<(&str, &str)>) -> usize {
    let cave_map = UnGraphMap::<&str, ()>::from_edges(input);
    let mut visit_tracker: HashSet<&str> = HashSet::new();
    visit_tracker.insert("start");

    count_paths_v2(&cave_map, "start", &mut visit_tracker, &mut None)
}

fn is_small_cave(cave: &str) -> bool {
    cave.chars().all(|c| c.is_lowercase())
}

fn is_start(cave: &str) -> bool {
    cave == "start"
}

fn is_end(cave: &str) -> bool {
    cave == "end"
}

fn count_simple_paths<'a>(
    cave_system: &UnGraphMap<&'a str, ()>,
    curr_node: &'a str,
    visited_small_caves: &mut HashSet<&'a str>,
) -> usize {
    if curr_node == "end" {
        visited_small_caves.remove(curr_node);
        return 1;
    }

    let mut count = 0;

    for connected_cave in cave_system.neighbors(curr_node) {
        if is_small_cave(connected_cave) {
            if visited_small_caves.contains(connected_cave) {
                continue;
            } else {
                visited_small_caves.insert(connected_cave);
            }
        }
        count += count_simple_paths(&cave_system, connected_cave, visited_small_caves);
        visited_small_caves.remove(connected_cave);
    }
    count
}

fn count_paths_v2<'a>(
    cave_system: &UnGraphMap<&'a str, ()>,
    curr_node: &'a str,
    visited_small_caves: &mut HashSet<&'a str>,
    twive_visited_cave: &mut Option<&'a str>,
) -> usize {
    if is_end(curr_node) {
        visited_small_caves.remove(curr_node);
        return 1;
    }

    let mut count = 0;
    for connected_cave in cave_system.neighbors(curr_node) {
        if is_small_cave(connected_cave) {
            if !visited_small_caves.contains(connected_cave) {
                visited_small_caves.insert(connected_cave);
            } else if twive_visited_cave.is_none()
                && !is_start(connected_cave)
                && !is_end(connected_cave)
            {
                *twive_visited_cave = Some(connected_cave);
            } else {
                continue;
            }
        }

        if let Some(_) = twive_visited_cave {
            count += count_simple_paths(cave_system, connected_cave, visited_small_caves);
        } else {
            count += count_paths_v2(
                cave_system,
                connected_cave,
                visited_small_caves,
                twive_visited_cave,
            );
        }

        if *twive_visited_cave == Some(connected_cave) {
            *twive_visited_cave = None;
        } else {
            visited_small_caves.remove(connected_cave);
        }
    }

    count
}

fn make_diagram(input: &Vec<(&str, &str)>) -> std::io::Result<()> {
    let cave_map = UnGraphMap::<&str, ()>::from_edges(input);

    let cave_diagram = Dot::with_config(&cave_map, &[Config::EdgeNoLabel]);

    let cave_diagram = format!("{:?}", cave_diagram);
    let mut file = File::create("outputs/cave.txt")?;
    file.write_all(cave_diagram.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let raw_input = util::read_input("inputs/day12.txt").unwrap();
        b.iter(|| {
            let input = process(&raw_input);
            part_1(&input);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let raw_input = util::read_input("inputs/day12.txt").unwrap();
        b.iter(|| {
            let input = process(&raw_input);
            part_2(&input);
        });
    }
}
