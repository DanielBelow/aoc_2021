use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day10)]
pub fn generate(inp: &str) -> Vec<String> {
    inp.lines().map(ToString::to_string).collect()
}

#[derive(Default)]
struct ParsingState {
    score: usize,
    incomplete: bool,
    stack: Vec<char>,
}

fn score_error(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Unexpected character"),
    }
}

fn score_completion(c: char) -> usize {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("Unexpected character {c}!"),
    }
}

fn syntax_error_score(s: &str) -> ParsingState {
    let mut stack = Vec::new();

    for chr in s.chars() {
        match chr {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),

            ')' | ']' | '}' | '>' if stack.is_empty() => {
                return ParsingState::default();
            }

            ')' | ']' | '}' | '>' => {
                if let Some(expected) = stack.pop() {
                    if expected != chr {
                        return ParsingState {
                            score: score_error(chr),
                            incomplete: false,
                            stack,
                        };
                    }
                }
            }
            _ => panic!("Invalid character"),
        }
    }

    ParsingState {
        score: 0,
        incomplete: !stack.is_empty(),
        stack,
    }
}

#[aoc(day10, part1)]
pub fn part1(inp: &[String]) -> usize {
    inp.iter().fold(0, |acc, it| {
        let ParsingState { score, .. } = syntax_error_score(it);
        acc + score
    })
}

#[aoc(day10, part2)]
pub fn part2(inp: &[String]) -> usize {
    let scores = inp
        .iter()
        .filter_map(|it| {
            let ParsingState {
                incomplete, stack, ..
            } = syntax_error_score(it);
            if incomplete {
                let score = stack
                    .iter()
                    .rev()
                    .fold(0, |acc, it| 5 * acc + score_completion(*it));
                Some(score)
            } else {
                None
            }
        })
        .sorted_unstable()
        .collect_vec();

    assert_eq!(scores.len() % 2, 1);

    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "[({(<(())[]>[[{[]{<()<>>\n\
                             [(()[<>])]({[<{<<[]>>(\n\
                             {([(<{}[<>[]}>{[]{[(<()>\n\
                             (((({<>}<{<{<>}{[]{[]{}\n\
                             [[<[([]))<([[{}[[()]]]\n\
                             [{[{({}]{}}([{[{{{}}([]\n\
                             {<[[]]>}<{[{[{[]{()[[[]\n\
                             [<(<(<(<{}))><([]([]()\n\
                             <{([([[(<>()){}]>(<<{{\n\
                             <{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_sample_p1() {
        let gen = generate(TEST_DATA);
        let res = part1(&gen);
        assert_eq!(res, 26_397);
    }

    #[test]
    fn test_sample_p2() {
        let gen = generate(TEST_DATA);
        let res = part2(&gen);
        assert_eq!(res, 288_957);
    }
}
