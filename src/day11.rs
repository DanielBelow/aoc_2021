use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day11)]
pub fn generate(inp: &str) -> Vec<Vec<usize>> {
    inp.lines()
        .map(|it| {
            it.chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as usize))
                .collect_vec()
        })
        .collect()
}

fn apply_flash(row: usize, col: usize, inp: &mut Vec<Vec<usize>>) {
    let mut positions_to_check = vec![(row + 1, col), (row, col + 1), (row + 1, col + 1)];

    if row >= 1 && col >= 1 {
        positions_to_check.push((row - 1, col - 1));
    }
    if row >= 1 {
        positions_to_check.push((row - 1, col));
        positions_to_check.push((row - 1, col + 1));
    }
    if col >= 1 {
        positions_to_check.push((row, col - 1));
        positions_to_check.push((row + 1, col - 1));
    }

    let width = inp.len();
    let height = inp[0].len();

    for &(dx, dy) in positions_to_check
        .iter()
        .filter(|(dx, dy)| *dx < width && *dy < height)
    {
        if inp[dx][dy] != 0 {
            inp[dx][dy] += 1;
        }
    }
}

fn needs_to_flash(inp: &[Vec<usize>]) -> bool {
    inp.iter().any(|r| r.iter().any(|c| *c > 9))
}

fn do_step(inp: &mut Vec<Vec<usize>>) -> usize {
    let mut flashes = 0;

    for row in inp.iter_mut() {
        for col in row {
            *col += 1;
        }
    }

    while needs_to_flash(inp) {
        for row in 0..inp.len() {
            for col in 0..inp[row].len() {
                if inp[row][col] > 9 {
                    inp[row][col] = 0;
                    apply_flash(row, col, inp);
                    flashes += 1;
                }
            }
        }
    }

    flashes
}

fn run_for_n_steps(n: usize, inp: &[Vec<usize>]) -> usize {
    let mut next = inp.to_vec();
    (0..n).fold(0, |acc, _| acc + do_step(&mut next))
}

fn all_flashed(inp: &[Vec<usize>]) -> bool {
    inp.iter().all(|r| r.iter().all(|c| *c == 0))
}

fn run_until_all_flash(inp: &[Vec<usize>]) -> usize {
    let mut next = inp.to_vec();
    let mut step = 0;

    while !all_flashed(&next) {
        let _ = do_step(&mut next);
        step += 1;
    }

    step
}

#[aoc(day11, part1)]
pub fn part1(inp: &[Vec<usize>]) -> usize {
    run_for_n_steps(100, inp)
}

#[aoc(day11, part2)]
pub fn part2(inp: &[Vec<usize>]) -> usize {
    run_until_all_flash(inp)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "5483143223\n\
                             2745854711\n\
                             5264556173\n\
                             6141336146\n\
                             6357385478\n\
                             4167524645\n\
                             2176841721\n\
                             6882881134\n\
                             4846848554\n\
                             5283751526";

    #[test]
    fn test_sample_p1() {
        let gen = generate(TEST_DATA);
        let res = run_for_n_steps(100, &gen);
        assert_eq!(res, 1656);
    }

    #[test]
    fn test_sample_p2() {
        let gen = generate(TEST_DATA);
        let res = part2(&gen);
        assert_eq!(res, 195);
    }
}
