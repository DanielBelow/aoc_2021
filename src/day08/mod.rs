use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashSet;

type UniqueSignal = Vec<String>;
type DigitOutput = Vec<String>;

#[aoc_generator(day8)]
pub fn generate(inp: &str) -> Vec<(UniqueSignal, DigitOutput)> {
    inp.lines()
        .map(|it| {
            let spl = it.split('|').collect_vec();
            let unique = spl[0]
                .split(' ')
                .filter(|it| !it.is_empty())
                .filter_map(|l| l.parse::<String>().ok())
                .collect_vec();
            let output = spl[1]
                .split(' ')
                .filter(|it| !it.is_empty())
                .filter_map(|l| l.parse::<String>().ok())
                .collect_vec();
            (unique, output)
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn part1(inp: &[(UniqueSignal, DigitOutput)]) -> usize {
    inp.iter().flat_map(|(_, out)| out).fold(0, |acc, it| {
        if matches!(it.len(), 2 | 3 | 4 | 7) {
            acc + 1
        } else {
            acc
        }
    })
}

fn find_digit_by_len(us: &UniqueSignal, len: usize) -> HashSet<char> {
    us.iter()
        .find(|it| it.len() == len)
        .unwrap()
        .chars()
        .collect::<HashSet<_>>()
}

#[aoc(day8, part2)]
pub fn part2(inp: &[(UniqueSignal, DigitOutput)]) -> usize {
    inp.iter()
        .map(|(lhs, rhs)| {
            let mut digits = vec![HashSet::new(); 10];

            digits[1] = find_digit_by_len(lhs, 2);
            digits[4] = find_digit_by_len(lhs, 4);
            digits[7] = find_digit_by_len(lhs, 3);
            digits[8] = find_digit_by_len(lhs, 7);

            // len 6 => 0, 6, 9
            lhs.iter()
                .filter(|it| it.len() == 6)
                .map(|it| it.chars().collect::<HashSet<_>>())
                .for_each(|it| {
                    if !it.is_superset(&digits[1]) {
                        digits[6] = it;
                    } else if !it.is_superset(&digits[4]) {
                        digits[0] = it;
                    } else {
                        digits[9] = it;
                    }
                });

            // len 5 => 2, 3, 5
            lhs.iter()
                .filter(|it| it.len() == 5)
                .map(|it| it.chars().collect::<HashSet<_>>())
                .for_each(|it| {
                    if it.is_subset(&digits[6]) {
                        digits[5] = it;
                    } else if it.is_subset(&digits[9]) {
                        digits[3] = it;
                    } else {
                        digits[2] = it;
                    }
                });

            rhs.iter().fold(0, |acc, it| {
                let sig = it.chars().collect::<HashSet<_>>();
                let as_num = digits.iter().position(|d| *d == sig).unwrap();
                acc * 10 + as_num
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\n\
                             edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\n\
                             fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\n\
                             fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\n\
                             aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\n\
                             fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\n\
                             dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\n\
                             bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\n\
                             egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\n\
                             gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_sample_p1() {
        let gen = generate(TEST_DATA);
        let res = part1(&gen);
        assert_eq!(res, 26);
    }

    #[test]
    fn test_sample_p2() {
        let gen = generate(TEST_DATA);
        let res = part2(&gen);
        assert_eq!(res, 61_229);
    }
}
