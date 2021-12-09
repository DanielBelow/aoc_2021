use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};
use std::collections::HashMap;

#[aoc_generator(day9)]
pub fn generate(inp: &str) -> Vec<Vec<usize>> {
    inp.lines()
        .map(|it| {
            it.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

fn get_neighbours(x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    const OFFSETS: [(i64, i64); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    let mut result = Vec::new();

    for (x_off, y_off) in OFFSETS {
        let dx = x as i64 + x_off;
        let dy = y as i64 + y_off;
        if dx < 0 || dx >= width as i64 {
            continue;
        }
        let dx = dx as usize;

        if dy < 0 || dy >= height as i64 {
            continue;
        }
        let dy = dy as usize;

        result.push((dx, dy));
    }

    result
}

fn is_lowpoint(candidate: usize, x: usize, y: usize, inp: &[Vec<usize>]) -> bool {
    get_neighbours(x, y, inp.len(), inp[0].len())
        .into_iter()
        .all(|(nx, ny)| inp[nx][ny] > candidate)
}

#[aoc(day9, part1)]
pub fn part1(inp: &[Vec<usize>]) -> usize {
    let width = inp.len();
    let height = inp[0].len();

    let mut low_points = Vec::new();

    for (x, y) in iproduct!(0..width, 0..height) {
        let cur = inp[x][y];
        if is_lowpoint(cur, x, y, inp) {
            low_points.push(cur);
        }
    }

    low_points.iter().fold(0, |acc, it| acc + 1 + *it)
}

fn find_lowpoint(x: usize, y: usize, inp: &[Vec<usize>]) -> Option<(usize, usize)> {
    let current = inp[x][y];

    if is_lowpoint(current, x, y, inp) {
        return Some((x, y));
    }

    get_neighbours(x, y, inp.len(), inp[0].len())
        .into_iter()
        .filter(|&(nx, ny)| inp[nx][ny] < current)
        .map(|(nx, ny)| find_lowpoint(nx, ny, inp))
        .next()
        .flatten()
}

#[aoc(day9, part2)]
pub fn part2(inp: &[Vec<usize>]) -> usize {
    let mut basin_sizes = HashMap::new();

    let width = inp.len();
    let height = inp[0].len();

    for (x, y) in iproduct!(0..width, 0..height) {
        let cur = inp[x][y];
        if cur == 9 {
            continue;
        }

        if let Some((lx, ly)) = find_lowpoint(x, y, inp) {
            basin_sizes
                .entry((lx, ly))
                .and_modify(|it| *it += 1)
                .or_insert(1);
        }
    }

    basin_sizes.values().sorted().rev().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "2199943210\n\
                             3987894921\n\
                             9856789892\n\
                             8767896789\n\
                             9899965678";

    #[test]
    fn test_sample_p1() {
        let gen = generate(TEST_DATA);
        let res = part1(&gen);
        assert_eq!(res, 15);
    }

    #[test]
    fn test_sample_p2() {
        let gen = generate(TEST_DATA);
        let res = part2(&gen);
        assert_eq!(res, 1_134);
    }
}
