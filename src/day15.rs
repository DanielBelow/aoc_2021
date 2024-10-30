use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day15)]
pub fn generate(inp: &str) -> Vec<Vec<usize>> {
    inp.lines()
        .map(|it| {
            it.to_string()
                .chars()
                .filter_map(|it| it.to_digit(10).map(|it| it as usize))
                .collect_vec()
        })
        .collect()
}

fn neighbours(cur: (usize, usize), size: usize) -> Vec<(usize, usize)> {
    let (x, y) = cur;

    let mut positions_to_check = vec![(x + 1, y), (x, y + 1)];
    if let Some(xx) = x.checked_sub(1) {
        positions_to_check.push((xx, y));
    }
    if let Some(yy) = y.checked_sub(1) {
        positions_to_check.push((x, yy));
    }

    positions_to_check
        .into_iter()
        .filter(|&(x, y)| x < size && y < size)
        .collect()
}

fn find_path(inp: &[Vec<usize>]) -> usize {
    let size = inp.len();

    let (_, risk) = pathfinding::prelude::dijkstra(
        &(0usize, 0usize),
        |&(x, y)| {
            neighbours((x, y), size)
                .iter()
                .map(|&it| (it, inp[it.0][it.1]))
                .collect_vec()
        },
        |(x, y)| *x == size - 1 && *y == size - 1,
    )
    .expect("Expected available path");

    risk
}

#[aoc(day15, part1)]
pub fn part1(inp: &[Vec<usize>]) -> usize {
    find_path(inp)
}

const fn inc_by_step(step: usize, it: usize) -> usize {
    if it + step < 10 {
        it + step
    } else {
        it + step - 9
    }
}

#[aoc(day15, part2)]
pub fn part2(inp: &[Vec<usize>]) -> usize {
    let mut extended_map = inp
        .iter()
        .map(|it| {
            std::iter::repeat(it)
                .take(5)
                .enumerate()
                .flat_map(|(step, it)| it.iter().map(|p| inc_by_step(step, *p)).collect_vec())
                .collect_vec()
        })
        .collect_vec();

    let size = extended_map.len();
    for step in 1..5 {
        for row in 0..size {
            let new_row = extended_map[row]
                .iter()
                .map(|p| inc_by_step(step, *p))
                .collect_vec();
            extended_map.push(new_row);
        }
    }

    find_path(&extended_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "1163751742\n\
                             1381373672\n\
                             2136511328\n\
                             3694931569\n\
                             7463417111\n\
                             1319128137\n\
                             1359912421\n\
                             3125421639\n\
                             1293138521\n\
                             2311944581";

    #[test]
    fn test_sample_p1() {
        let gen = generate(TEST_DATA);
        let res = part1(&gen);
        assert_eq!(res, 40);
    }

    #[test]
    fn test_sample_p2() {
        let gen = generate(TEST_DATA);
        let res = part2(&gen);
        assert_eq!(res, 315);
    }
}
