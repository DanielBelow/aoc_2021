use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
pub fn generate(inp: &str) -> Option<Vec<usize>> {
    Some(
        inp.lines()
            .next()?
            .split(',')
            .filter_map(|it| it.parse().ok())
            .collect(),
    )
}

fn simulate_fish(num_days: u16, fish: &[usize]) -> usize {
    let mut result = [0; 9];

    for f in fish {
        result[*f] += 1;
    }

    for _ in 0..num_days {
        let new_fish = result[0];
        result.rotate_left(1);
        result[6] += new_fish;
    }

    result.iter().sum::<usize>()
}

#[aoc(day6, part1)]
pub fn part1(inp: &[usize]) -> usize {
    simulate_fish(80, inp)
}

#[aoc(day6, part2)]
pub fn part2(inp: &[usize]) -> usize {
    simulate_fish(256, inp)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "3,4,3,1,2";

    #[test]
    fn test_sample_p1() {
        let Some(gen) = generate(TEST_DATA) else {
            panic!("Could not parse test input")
        };

        let res = part1(&gen);
        assert_eq!(res, 5934);
    }

    #[test]
    fn test_sample_p2() {
        let Some(gen) = generate(TEST_DATA) else {
            panic!("Could not parse test input")
        };

        let res = part2(&gen);
        assert_eq!(res, 26_984_457_539);
    }
}
