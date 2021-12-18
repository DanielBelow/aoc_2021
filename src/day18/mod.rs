use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;
use std::ops::Add;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum SnailfishNumber {
    Regular(u64),
    Pair(Box<SnailfishNumber>, Box<SnailfishNumber>),
}

impl Add for SnailfishNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Pair(Box::new(self), Box::new(rhs))
    }
}

// Shamelessly stolen from https://old.reddit.com/r/adventofcode/comments/rizw2c/2021_day_18_solutions/hp0wqs2/
impl SnailfishNumber {
    fn reduce(self) -> Self {
        let mut number = self;
        loop {
            let (next_number, res) = number.explode(0);
            number = next_number;
            if res.is_some() {
                continue;
            };
            let (next_number, res) = number.split();
            number = next_number;
            if !res {
                break;
            }
        }
        number
    }

    #[allow(clippy::type_complexity)]
    fn explode(self, depth: usize) -> (Self, Option<(Option<u64>, Option<u64>)>) {
        match self {
            Self::Regular(_) => (self, None),
            Self::Pair(l, r) => match (*l, *r) {
                (Self::Regular(nl), Self::Regular(nr)) if depth >= 4 => {
                    (Self::Regular(0), Some((Some(nl), Some(nr))))
                }
                (l, r) => match l.explode(depth + 1) {
                    (l_reduced, Some((explode_left, explode_right))) => {
                        let r_added = if let Some(explode_right) = explode_right {
                            r.add_to_leftmost(explode_right)
                        } else {
                            r
                        };
                        (
                            Self::Pair(Box::new(l_reduced), Box::new(r_added)),
                            Some((explode_left, None)),
                        )
                    }
                    (l_reduced, None) => match r.explode(depth + 1) {
                        (r_reduced, Some((explode_left, explode_right))) => {
                            let l_added = if let Some(explode_left) = explode_left {
                                l_reduced.add_to_rightmost(explode_left)
                            } else {
                                l_reduced
                            };
                            (
                                Self::Pair(Box::new(l_added), Box::new(r_reduced)),
                                Some((None, explode_right)),
                            )
                        }
                        (r_reduced, None) => {
                            (Self::Pair(Box::new(l_reduced), Box::new(r_reduced)), None)
                        }
                    },
                },
            },
        }
    }

    fn split(self) -> (Self, bool) {
        match self {
            Self::Regular(n) if n >= 10 => (
                Self::Pair(
                    Box::new(Self::Regular(n / 2)),
                    Box::new(Self::Regular(if n % 2 == 0 { n / 2 } else { n / 2 + 1 })),
                ),
                true,
            ),
            Self::Regular(_) => (self, false),
            Self::Pair(l, r) => {
                let (l_split, l_was_split) = l.split();
                if l_was_split {
                    (Self::Pair(Box::new(l_split), r), true)
                } else {
                    let (r_split, r_was_split) = r.split();
                    (
                        Self::Pair(Box::new(l_split), Box::new(r_split)),
                        r_was_split,
                    )
                }
            }
        }
    }

    fn add_to_leftmost(self, val: u64) -> Self {
        match self {
            SnailfishNumber::Regular(n) => SnailfishNumber::Regular(n + val),
            SnailfishNumber::Pair(l, r) => {
                SnailfishNumber::Pair(Box::new(l.add_to_leftmost(val)), r)
            }
        }
    }

    fn add_to_rightmost(self, val: u64) -> Self {
        match self {
            SnailfishNumber::Regular(n) => SnailfishNumber::Regular(n + val),
            SnailfishNumber::Pair(l, r) => {
                SnailfishNumber::Pair(l, Box::new(r.add_to_rightmost(val)))
            }
        }
    }

    fn magnitude(&self) -> u64 {
        match self {
            Self::Regular(n) => *n,
            Self::Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }
}

fn try_parse_snailfish_number(inp: &str) -> Option<SnailfishNumber> {
    let re = Regex::new("^\\[([0-9]+),([0-9]+)]$").unwrap();
    if re.is_match(inp) {
        let caps = re.captures(inp).unwrap();
        let x = caps.get(1).unwrap();
        let y = caps.get(2).unwrap();

        let x = x.as_str().parse::<u64>().unwrap();
        let y = y.as_str().parse::<u64>().unwrap();

        let left = SnailfishNumber::Regular(x);
        let right = SnailfishNumber::Regular(y);

        return Some(SnailfishNumber::Pair(Box::new(left), Box::new(right)));
    }

    None
}

fn parse_snailfish_number(inp: &str) -> SnailfishNumber {
    if let Some(sfn) = try_parse_snailfish_number(inp) {
        return sfn;
    }

    let contained = inp
        .strip_prefix('[')
        .and_then(|it| it.strip_suffix(']'))
        .unwrap();

    let mut split_idx = None;
    if contained.starts_with('[') {
        let mut bracket_stack = 0;
        for (idx, c) in contained.chars().enumerate() {
            if c == '[' {
                bracket_stack += 1;
            }

            if c == ']' {
                bracket_stack -= 1;
                if bracket_stack == 0 {
                    split_idx = Some(idx + 1);
                    break;
                }
            }
        }
    }

    let (left, right) = if let Some(split_idx) = split_idx {
        assert_eq!(contained.chars().nth(split_idx), Some(','));
        let (l, r) = contained.split_at(split_idx);
        (l, r.strip_prefix(',').unwrap())
    } else {
        let comma_idx = contained.chars().position(|it| it == ',').unwrap();
        let (l, r) = contained.split_at(comma_idx);
        (l, r.strip_prefix(',').unwrap())
    };

    let try_parse_snailfish = |s: &str| {
        if let Ok(x) = s.parse::<u64>() {
            SnailfishNumber::Regular(x)
        } else {
            parse_snailfish_number(s)
        }
    };

    // try parsing as number
    let x = try_parse_snailfish(left);
    let y = try_parse_snailfish(right);
    SnailfishNumber::Pair(Box::new(x), Box::new(y))
}

#[aoc_generator(day18)]
pub fn generate(inp: &str) -> Vec<SnailfishNumber> {
    inp.lines().map(parse_snailfish_number).collect_vec()
}

#[aoc(day18, part1)]
pub fn part1(inp: &[SnailfishNumber]) -> u64 {
    inp.iter()
        .cloned()
        .reduce(|l, r| (l + r).reduce())
        .unwrap()
        .magnitude()
}

#[aoc(day18, part2)]
pub fn part2(inp: &[SnailfishNumber]) -> Option<u64> {
    inp.iter()
        .cloned()
        .permutations(2)
        .map(|it| {
            it.into_iter()
                .reduce(|l, r| (l + r).reduce())
                .unwrap()
                .magnitude()
        })
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_1() {
        let inp = "[1,2]";
        let sfn = parse_snailfish_number(inp);
        let expected = SnailfishNumber::Pair(
            Box::new(SnailfishNumber::Regular(1)),
            Box::new(SnailfishNumber::Regular(2)),
        );

        assert_eq!(sfn, expected);
    }

    #[test]
    fn test_parsing_2() {
        let inp = "[[1,2],3]";
        let sfn = parse_snailfish_number(inp);
        let expected = SnailfishNumber::Pair(
            Box::new(SnailfishNumber::Pair(
                Box::new(SnailfishNumber::Regular(1)),
                Box::new(SnailfishNumber::Regular(2)),
            )),
            Box::new(SnailfishNumber::Regular(3)),
        );
        assert_eq!(sfn, expected);
    }

    #[test]
    fn test_parsing_3() {
        let inp = "[9,[8,7]]";
        let sfn = parse_snailfish_number(inp);
        let expected = SnailfishNumber::Pair(
            Box::new(SnailfishNumber::Regular(9)),
            Box::new(SnailfishNumber::Pair(
                Box::new(SnailfishNumber::Regular(8)),
                Box::new(SnailfishNumber::Regular(7)),
            )),
        );

        assert_eq!(sfn, expected);
    }

    #[test]
    fn test_parsing_4() {
        let inp = "[[1,9],[8,5]]";
        let sfn = parse_snailfish_number(inp);
        let expected = SnailfishNumber::Pair(
            Box::new(SnailfishNumber::Pair(
                Box::new(SnailfishNumber::Regular(1)),
                Box::new(SnailfishNumber::Regular(9)),
            )),
            Box::new(SnailfishNumber::Pair(
                Box::new(SnailfishNumber::Regular(8)),
                Box::new(SnailfishNumber::Regular(5)),
            )),
        );
        assert_eq!(sfn, expected);
    }
}
