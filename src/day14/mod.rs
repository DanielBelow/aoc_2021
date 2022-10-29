use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use parse_display::{Display, FromStr};
use std::collections::HashMap;

#[derive(Display, FromStr, Clone, Debug)]
#[display("{from} -> {to}")]
pub struct Mapping {
    from: String,
    to: char,
}

#[derive(Clone, Debug)]
pub struct Input {
    template: String,
    mappings: Vec<Mapping>,
}

#[aoc_generator(day14)]
pub fn generate(inp: &str) -> Input {
    let template = inp.lines().next().expect("Expected template");
    let mappings = inp
        .lines()
        .skip(2)
        .filter_map(|it| it.parse().ok())
        .collect_vec();
    Input {
        template: template.to_string(),
        mappings,
    }
}

fn run_steps(n: usize, inp: &Input) -> usize {
    let mut pairs = inp
        .template
        .chars()
        .tuple_windows::<(_, _)>()
        .map(|(l, r)| format!("{l}{r}"))
        .counts();

    let mappings = inp
        .mappings
        .iter()
        .map(|it| (&it.from, it.to))
        .collect::<HashMap<_, _>>();

    let mut next_step_pairs = pairs.clone();
    for _ in 0..n {
        for (k, v) in pairs.iter().filter(|(_, &v)| v > 0) {
            if let Some(m) = mappings.get(k) {
                if let [l, r, ..] = k.chars().collect_vec().as_slice() {
                    *next_step_pairs.entry(format!("{l}{m}")).or_insert(0) += *v;
                    *next_step_pairs.entry(format!("{m}{r}")).or_insert(0) += *v;
                    if let Some(val) = next_step_pairs.get_mut(k) {
                        *val -= v;
                    }
                }
            }
        }

        pairs = next_step_pairs.clone();
    }

    let mut char_freqs = pairs.iter().fold(HashMap::new(), |mut acc, (k, v)| {
        for c in k.chars() {
            *acc.entry(c).or_insert(0) += *v;
        }
        acc
    });

    char_freqs.values_mut().for_each(|it| *it /= 2);
    *char_freqs
        .entry(inp.template.chars().next().unwrap())
        .or_insert(0) += 1;
    *char_freqs
        .entry(inp.template.chars().last().unwrap())
        .or_insert(0) += 1;

    match char_freqs.values().minmax() {
        MinMax(min, max) => max - min,
        _ => 0,
    }
}

#[aoc(day14, part1)]
pub fn part1(inp: &Input) -> usize {
    run_steps(10, inp)
}

#[aoc(day14, part2)]
pub fn part2(inp: &Input) -> usize {
    run_steps(40, inp)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "NNCB\n\
                             \n\
                             CH -> B\n\
                             HH -> N\n\
                             CB -> H\n\
                             NH -> C\n\
                             HB -> C\n\
                             HC -> B\n\
                             HN -> C\n\
                             NN -> C\n\
                             BH -> H\n\
                             NC -> B\n\
                             NB -> B\n\
                             BN -> B\n\
                             BB -> N\n\
                             BC -> B\n\
                             CC -> N\n\
                             CN -> C";

    #[test]
    fn test_sample_p1() {
        let gen = generate(TEST_DATA);
        let res = part1(&gen);
        assert_eq!(res, 1_588);
    }

    #[test]
    fn test_sample_p2() {
        let gen = generate(TEST_DATA);
        let res = part2(&gen);
        assert_eq!(res, 2_188_189_693_529);
    }
}
