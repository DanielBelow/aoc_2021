use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day7)]
pub fn generate(inp: &str) -> Vec<i64> {
    inp.lines()
        .next()
        .unwrap()
        .split(',')
        .filter_map(|it| it.parse().ok())
        .collect()
}

fn fuel_for_target<T>(target: i64, inp: &[i64], fuel_cost: T) -> i64
where
    T: Fn(i64) -> i64,
{
    inp.iter().fold(0, |acc, it| {
        let dist = (*it - target).abs();
        acc + fuel_cost(dist)
    })
}

#[aoc(day7, part1)]
pub fn part1(inp: &[i64]) -> Option<i64> {
    let (&min_pos, &max_pos) = inp.iter().minmax().into_option()?;
    (min_pos..=max_pos)
        .map(|it| fuel_for_target(it, inp, |it| it))
        .min()
}

#[aoc(day7, part2)]
pub fn part2(inp: &[i64]) -> Option<i64> {
    let (&min_pos, &max_pos) = inp.iter().minmax().into_option()?;
    (min_pos..=max_pos)
        .map(|it| fuel_for_target(it, inp, |it| it * (it + 1) / 2))
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_sample_p1() {
        let gen = generate(TEST_DATA);
        let res = part1(&gen);
        assert_eq!(res, Some(37));
    }

    #[test]
    fn test_sample_p2() {
        let gen = generate(TEST_DATA);
        let res = part2(&gen);
        assert_eq!(res, Some(168));
    }
}
