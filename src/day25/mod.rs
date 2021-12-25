use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day25)]
pub fn generate(inp: &str) -> Vec<Vec<char>> {
    inp.lines().map(|it| it.chars().collect_vec()).collect_vec()
}

fn simulate_step(inp: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut result = inp.to_vec();

    let height = inp.len();
    let width = inp[0].len();

    // move east
    for x in 0..height {
        for y in 0..width {
            let cur = inp[x][y];
            if cur == '>' {
                let yy = if y + 1 == width { 0 } else { y + 1 };
                if inp[x][yy] == '.' {
                    result[x][y] = '.';
                    result[x][yy] = cur;
                }
            }
        }
    }

    let inp = result.clone();

    // move south
    for x in 0..height {
        for y in 0..width {
            let cur = inp[x][y];
            if cur == 'v' {
                let xx = if x + 1 == height { 0 } else { x + 1 };
                if inp[xx][y] == '.' {
                    result[x][y] = '.';
                    result[xx][y] = cur;
                }
            }
        }
    }

    result
}

#[aoc(day25, part1)]
pub fn part1(inp: &[Vec<char>]) -> u64 {
    let mut inp = inp.to_vec();
    let mut moves = 0;
    loop {
        let res = simulate_step(&inp);
        moves += 1;
        if res == inp {
            break;
        }

        inp = res;
    }

    moves
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "v...>>.vv>\n\
                             .vv>>.vv..\n\
                             >>.>v>...v\n\
                             >>v>>.>.v.\n\
                             v>v.vv.v..\n\
                             >.>>..v...\n\
                             .vv..>.>v.\n\
                             v.v..>>v.v\n\
                             ....v..v.>";

    #[test]
    fn test_sample_p1() {
        let gen = generate(TEST_DATA);
        let res = part1(&gen);
        assert_eq!(res, 58);
    }
}
