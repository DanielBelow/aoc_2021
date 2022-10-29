use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Display, FromStr, Copy, Clone)]
#[display("{x},{y}")]
pub struct Point {
    x: i64,
    y: i64,
}

#[derive(Display, FromStr, Copy, Clone)]
#[display("{from} -> {to}")]
pub struct Line {
    from: Point,
    to: Point,
}

impl Line {
    fn points_on_line(&self) -> Vec<(i64, i64)> {
        let x_dir = match self.from.x.cmp(&self.to.x) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };

        let y_dir = match self.from.y.cmp(&self.to.y) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };

        let mut cur_x = self.from.x;
        let mut cur_y = self.from.y;

        let mut result = vec![(cur_x, cur_y)];

        while cur_x != self.to.x || cur_y != self.to.y {
            cur_x += x_dir;
            cur_y += y_dir;

            result.push((cur_x, cur_y));
        }

        result
    }
}

#[aoc_generator(day5)]
pub fn generate(inp: &str) -> Vec<Line> {
    inp.lines()
        .filter_map(|it| it.parse::<Line>().ok())
        .collect()
}

fn count_intersections(lines: &[Line]) -> usize {
    lines
        .iter()
        .flat_map(Line::points_on_line)
        .fold(HashMap::new(), |mut acc, (x, y)| {
            acc.entry((x, y)).and_modify(|it| *it += 1).or_insert(1);
            acc
        })
        .iter()
        .filter(|(_, &count)| count > 1)
        .count()
}

#[aoc(day5, part1)]
pub fn part1(inp: &[Line]) -> usize {
    let inp = inp
        .iter()
        .filter(|it| it.from.x == it.to.x || it.from.y == it.to.y)
        .copied()
        .collect_vec();

    count_intersections(&inp)
}

#[aoc(day5, part2)]
pub fn part2(inp: &[Line]) -> usize {
    count_intersections(inp)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "0,9 -> 5,9\n\
                             8,0 -> 0,8\n\
                             9,4 -> 3,4\n\
                             2,2 -> 2,1\n\
                             7,0 -> 7,4\n\
                             6,4 -> 2,0\n\
                             0,9 -> 2,9\n\
                             3,4 -> 1,4\n\
                             0,0 -> 8,8\n\
                             5,5 -> 8,2";

    #[test]
    fn test_sample_p1() {
        let gen = generate(TEST_DATA);
        let res = part1(&gen);
        assert_eq!(res, 5);
    }

    #[test]
    fn test_sample_p2() {
        let gen = generate(TEST_DATA);
        let res = part2(&gen);
        assert_eq!(res, 12);
    }
}
