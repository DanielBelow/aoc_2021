use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day1)]
pub fn generate(inp: &str) -> Vec<usize> {
    inp.lines().filter_map(|it| it.parse().ok()).collect()
}

#[aoc(day1, part1)]
pub fn part1(inp: &[usize]) -> usize {
    inp.iter().tuple_windows().filter(|(a, b)| b > a).count()
}

#[aoc(day1, part2)]
pub fn part2(inp: &[usize]) -> usize {
    inp.iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let inp = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let res = part1(&inp);
        assert_eq!(res, 7);
    }

    #[test]
    fn test_sample_p2() {
        let inp = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let res = part2(&inp);
        assert_eq!(res, 5);
    }
}
