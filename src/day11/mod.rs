use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};

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
    for (dx, dy) in iproduct!(-1i64..=1, -1i64..=1) {
        if dx == 0 && dy == 0 {
            continue;
        }

        let row = row as i64 + dx;
        let col = col as i64 + dy;

        if row < 0 || row >= inp.len() as i64 {
            continue;
        }

        let row = row as usize;
        if col < 0 || col >= inp[row].len() as i64 {
            continue;
        }

        let col = col as usize;

        if inp[row][col] != 0 {
            inp[row][col] += 1;
        }
    }
}

fn needs_to_flash(flashed: &[Vec<bool>], inp: &[Vec<usize>]) -> bool {
    for (r, row) in inp.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if *col > 9 && !flashed[r][c] {
                return true;
            }
        }
    }

    false
}

fn do_step(inp: &mut Vec<Vec<usize>>) -> usize {
    let mut flashes = 0;

    for row in inp.iter_mut() {
        for col in row {
            *col += 1;
        }
    }

    let mut did_flash = vec![vec![false; 10]; 10];

    while needs_to_flash(&did_flash, inp) {
        for row in 0..inp.len() {
            for col in 0..inp[row].len() {
                if inp[row][col] > 9 && !did_flash[row][col] {
                    inp[row][col] = 0;
                    did_flash[row][col] = true;

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
