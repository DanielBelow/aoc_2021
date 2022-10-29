use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::collections::HashSet;

#[derive(Display, FromStr, Eq, PartialEq, Copy, Clone, Debug)]
#[display("{x},{y}")]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Display, FromStr, Eq, PartialEq, Copy, Clone, Debug)]
pub enum Fold {
    #[display("fold along x={0}")]
    Left(usize),

    #[display("fold along y={0}")]
    Up(usize),
}

#[derive(Clone, Debug)]
pub struct Input {
    points: Vec<Point>,
    folds: Vec<Fold>,
}

#[aoc_generator(day13)]
pub fn generate(inp: &str) -> Option<Input> {
    let mut spl = inp.split("\n\n");
    let first = spl.next()?;
    let second = spl.next()?;

    let points = first
        .lines()
        .filter_map(|it| it.parse::<Point>().ok())
        .collect_vec();
    let folds = second
        .lines()
        .filter_map(|it| it.parse::<Fold>().ok())
        .collect_vec();

    Some(Input { points, folds })
}

fn fold(fld: Fold, paper: &mut HashSet<(usize, usize)>) {
    match fld {
        Fold::Left(y) => {
            let points_bottom = paper
                .iter()
                .filter(|(_, yy)| *yy > y)
                .copied()
                .collect_vec();
            for &(xx, yy) in &points_bottom {
                let diff = yy - y;
                let insert_y = y - diff;
                paper.insert((xx, insert_y));
            }

            for p in points_bottom {
                paper.remove(&p);
            }
        }
        Fold::Up(x) => {
            let points_right = paper
                .iter()
                .filter(|(xx, _)| *xx > x)
                .copied()
                .collect_vec();
            for &(xx, yy) in &points_right {
                let diff = xx - x;
                let insert_x = x - diff;
                paper.insert((insert_x, yy));
            }

            for p in points_right {
                paper.remove(&p);
            }
        }
    }
}

#[aoc(day13, part1)]
pub fn part1(inp: &Input) -> usize {
    let mut paper = inp
        .points
        .iter()
        .map(|it| (it.y, it.x))
        .collect::<HashSet<_>>();

    let first_fold = inp.folds.first().unwrap();
    fold(*first_fold, &mut paper);

    paper.len()
}

#[aoc(day13, part2)]
pub fn part2(inp: &Input) -> Option<String> {
    let mut paper = inp
        .points
        .iter()
        .map(|it| (it.y, it.x))
        .collect::<HashSet<_>>();

    for fld in &inp.folds {
        fold(*fld, &mut paper);
    }

    let (max_x, _) = paper.iter().max_by_key(|(xx, _)| *xx)?;
    let (_, max_y) = paper.iter().max_by_key(|(_, yy)| *yy)?;

    let mut result = String::from("\n");

    for x in 0..=*max_x {
        for y in 0..=*max_y {
            if paper.contains(&(x, y)) {
                result.push('#');
            } else {
                result.push(' ');
            }
        }

        result.push('\n');
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "6,10\n\
0,14\n\
9,10\n\
0,3\n\
10,4\n\
4,11\n\
6,0\n\
6,12\n\
4,1\n\
0,13\n\
10,12\n\
3,4\n\
3,0\n\
8,4\n\
1,10\n\
2,14\n\
8,10\n\
9,0\n\
\n\
fold along y=7\n\
fold along x=5";

    #[test]
    fn test_sample_p1() {
        let gen = generate(TEST_DATA);
        let res = part1(&gen.unwrap());
        assert_eq!(res, 17);
    }
}
