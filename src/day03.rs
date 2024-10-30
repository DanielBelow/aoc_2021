use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn generate(inp: &str) -> Vec<String> {
    inp.lines().map(ToString::to_string).collect()
}

fn count_zero_bits_at_idx(inp: &[String], idx: usize) -> usize {
    inp.iter()
        .filter(|it| it.chars().nth(idx).expect("idx is in range") == '0')
        .count()
}

fn select_bit_at_idx<P>(inp: &[String], idx: usize, pred: P) -> char
where
    P: Fn(usize, usize) -> char,
{
    let zero_count = count_zero_bits_at_idx(inp, idx);
    let one_count = inp.len() - zero_count;

    pred(zero_count, one_count)
}

fn most_common_at_idx(inp: &[String], idx: usize) -> char {
    select_bit_at_idx(inp, idx, |z, o| if z > o { '0' } else { '1' })
}

fn least_common_at_idx(inp: &[String], idx: usize) -> char {
    select_bit_at_idx(inp, idx, |z, o| if z <= o { '0' } else { '1' })
}

fn from_binary(number: &str) -> i64 {
    i64::from_str_radix(number, 2).expect("number is definitely binary")
}

fn calculate_rate<P>(inp: &[String], pred: P) -> i64
where
    P: Fn(&[String], usize) -> char,
{
    let length = inp[0].len();
    let mut result = String::with_capacity(length);

    for idx in 0..length {
        let mc = pred(inp, idx);
        result.push(mc);
    }

    from_binary(&result)
}

#[aoc(day3, part1)]
pub fn part1(inp: &[String]) -> i64 {
    let gamma = calculate_rate(inp, most_common_at_idx);
    let epsilon = calculate_rate(inp, least_common_at_idx);

    gamma * epsilon
}

fn find_rating_value<P>(inp: &[String], pred: P) -> i64
where
    P: Fn(&[String], usize) -> char,
{
    let length = inp[0].len();

    let mut current_list = inp.to_vec();
    for idx in 0..length {
        if current_list.len() == 1 {
            break;
        }

        let bit_to_find = pred(&current_list, idx);
        current_list.retain(|it| it.chars().nth(idx).expect("idx is in range") == bit_to_find);
    }

    assert_eq!(current_list.len(), 1);

    let result = &current_list[0];
    from_binary(result)
}

#[aoc(day3, part2)]
pub fn part2(inp: &[String]) -> i64 {
    let oxygen = find_rating_value(inp, most_common_at_idx);
    let co2 = find_rating_value(inp, least_common_at_idx);

    oxygen * co2
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "00100\n\
                             11110\n\
                             10110\n\
                             10111\n\
                             10101\n\
                             01111\n\
                             00111\n\
                             11100\n\
                             10000\n\
                             11001\n\
                             00010\n\
                             01010";

    #[test]
    fn test_sample_p1() {
        let gen = generate(TEST_DATA);
        let res = part1(&gen);
        assert_eq!(res, 198);
    }

    #[test]
    fn test_sample_p2() {
        let gen = generate(TEST_DATA);
        let res = part2(&gen);
        assert_eq!(res, 230);
    }
}
