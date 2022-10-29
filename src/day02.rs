use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Debug, Copy, Clone)]
pub enum Direction {
    #[display("forward {0}")]
    Forward(i64),

    #[display("down {0}")]
    Down(i64),

    #[display("up {0}")]
    Up(i64),
}

#[derive(Default)]
struct State {
    horizontal_position: i64,
    depth: i64,
    aim: i64,
}

#[aoc_generator(day2)]
pub fn generate(inp: &str) -> Vec<Direction> {
    inp.lines().filter_map(|it| it.parse().ok()).collect()
}

#[aoc(day2, part1)]
pub fn part1(inp: &[Direction]) -> i64 {
    let res = inp.iter().fold(State::default(), |acc, &it| match it {
        Direction::Forward(x) => State {
            horizontal_position: acc.horizontal_position + x,
            ..acc
        },
        Direction::Down(x) => State {
            depth: acc.depth + x,
            ..acc
        },
        Direction::Up(x) => State {
            depth: acc.depth - x,
            ..acc
        },
    });

    res.horizontal_position * res.depth
}

#[aoc(day2, part2)]
pub fn part2(inp: &[Direction]) -> i64 {
    let res = inp.iter().fold(State::default(), |acc, &it| match it {
        Direction::Forward(x) => State {
            horizontal_position: acc.horizontal_position + x,
            depth: acc.depth + acc.aim * x,
            ..acc
        },
        Direction::Down(x) => State {
            aim: acc.aim + x,
            ..acc
        },
        Direction::Up(x) => State {
            aim: acc.aim - x,
            ..acc
        },
    });

    res.horizontal_position * res.depth
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "forward 5\n\
                             down 5\n\
                             forward 8\n\
                             up 3\n\
                             down 8\n\
                             forward 2";

    #[test]
    fn test_sample_p1() {
        let gen = generate(TEST_DATA);
        let res = part1(&gen);

        assert_eq!(res, 150);
    }

    #[test]
    fn test_sample_p2() {
        let gen = generate(TEST_DATA);
        let res = part2(&gen);

        assert_eq!(res, 900);
    }
}
