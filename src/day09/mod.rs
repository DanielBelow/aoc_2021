use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};
use std::collections::HashSet;

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

fn get_neighbour_coords(x: usize, y: usize, inp: &[Vec<usize>]) -> Vec<(usize, usize)> {
    let mut neighbours = vec![(x + 1, y), (x, y + 1)];
    if let Some(it) = x.checked_sub(1) {
        neighbours.push((it, y));
    }
    if let Some(it) = y.checked_sub(1) {
        neighbours.push((x, it));
    }

    let width = inp.len();
    let height = inp[0].len();

    neighbours
        .iter()
        .copied()
        .filter(|&(dx, dy)| dx < width && dy < height)
        .collect()
}

fn is_lowpoint(candidate: usize, x: usize, y: usize, inp: &[Vec<usize>]) -> bool {
    get_neighbour_coords(x, y, inp)
        .into_iter()
        .all(|(nx, ny)| inp[nx][ny] > candidate)
}

fn get_lowpoints(inp: &[Vec<usize>]) -> Vec<(usize, usize)> {
    let width = inp.len();
    let height = inp[0].len();

    let mut low_points = Vec::new();

    for (x, y) in iproduct!(0..width, 0..height) {
        let cur = inp[x][y];
        if is_lowpoint(cur, x, y, inp) {
            low_points.push((x, y));
        }
    }

    low_points
}

#[aoc(day9, part1)]
pub fn part1(inp: &[Vec<usize>]) -> usize {
    get_lowpoints(inp)
        .into_iter()
        .fold(0, |acc, (x, y)| acc + 1 + inp[x][y])
}

fn get_basin_size(x: usize, y: usize, inp: &[Vec<usize>]) -> usize {
    let mut basin = HashSet::new();

    let mut queue = vec![(x, y)];

    while let Some((x, y)) = queue.pop() {
        let neighbours = get_neighbour_coords(x, y, inp)
            .into_iter()
            .filter(|&(nx, ny)| inp[nx][ny] != 9)
            .collect_vec();

        for (nx, ny) in neighbours {
            if basin.insert((nx, ny)) {
                queue.push((nx, ny));
            }
        }
    }

    basin.len()
}

#[aoc(day9, part2)]
pub fn part2(inp: &[Vec<usize>]) -> usize {
    get_lowpoints(inp)
        .into_iter()
        .map(|(x, y)| get_basin_size(x, y, inp))
        .sorted()
        .rev()
        .take(3)
        .product()
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
