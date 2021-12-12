use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Eq, PartialEq, Clone, Debug)]
#[display("{from}-{to}")]
pub struct Path {
    from: String,
    to: String,
}

#[aoc_generator(day12)]
pub fn generate(inp: &str) -> Vec<Path> {
    inp.lines()
        .filter_map(|it| it.parse::<Path>().ok())
        .fold(Vec::new(), |mut acc, it| {
            if it.from != "start" && it.to != "end" {
                let inverted = Path {
                    from: it.to.clone(),
                    to: it.from.clone(),
                };
                acc.push(inverted);
            }

            acc.push(it);

            acc
        })
}

fn collect_paths_from<'a>(
    cur: &str,
    inp: &'a [Path],
    cur_path: &[&'a String],
    can_double: bool,
    result: &mut Vec<Vec<&'a String>>,
    can_visit: fn(&str, &[&String], bool) -> bool,
) {
    if cur == "end" {
        result.push(Vec::from(cur_path));
        return;
    }

    let can_double = can_double
        && cur_path
            .iter()
            .filter(|&&it| it == cur && it.chars().all(char::is_lowercase))
            .count()
            < 2;

    for available_path in inp
        .iter()
        .filter(|it| it.from == cur && can_visit(&it.to, cur_path, can_double))
    {
        let mut cur_path = cur_path.to_owned();
        cur_path.push(&available_path.to);

        collect_paths_from(
            &available_path.to,
            inp,
            &cur_path,
            can_double,
            result,
            can_visit,
        );
    }
}

fn find_paths(
    inp: &[Path],
    can_double: bool,
    can_visit: fn(&str, &[&String], bool) -> bool,
) -> Vec<Vec<&String>> {
    let mut result = Vec::new();

    let start = inp
        .iter()
        .find(|it| it.from == "start")
        .expect("Start node exists");

    collect_paths_from(
        &start.from,
        inp,
        &[&start.from],
        can_double,
        &mut result,
        can_visit,
    );

    result
}

#[aoc(day12, part1)]
pub fn part1(inp: &[Path]) -> usize {
    let can_visit_cave = |node: &str, paths: &[&String], _: bool| {
        !node.chars().all(char::is_lowercase) || !paths.iter().any(|&it| it == node)
    };

    let paths = find_paths(inp, false, can_visit_cave);
    paths.len()
}

#[aoc(day12, part2)]
pub fn part2(inp: &[Path]) -> usize {
    let can_visit_cave = |node: &str, paths: &[&String], can_double: bool| {
        if node == "start" {
            return false;
        }

        if !node.chars().all(char::is_lowercase) {
            return true;
        }

        can_double || !paths.iter().any(|&it| it == node)
    };

    let paths = find_paths(inp, true, can_visit_cave);
    paths.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "start-A\n\
                             start-b\n\
                             A-c\n\
                             A-b\n\
                             b-d\n\
                             A-end\n\
                             b-end";

    const TEST_DATA_MED: &str = "dc-end\n\
                                 HN-start\n\
                                 start-kj\n\
                                 dc-start\n\
                                 dc-HN\n\
                                 LN-dc\n\
                                 HN-end\n\
                                 kj-sa\n\
                                 kj-HN\n\
                                 kj-dc";

    const TEST_DATA_LARGE: &str = "fs-end\n\
                                   he-DX\n\
                                   fs-he\n\
                                   start-DX\n\
                                   pj-DX\n\
                                   end-zg\n\
                                   zg-sl\n\
                                   zg-pj\n\
                                   pj-he\n\
                                   RW-he\n\
                                   fs-DX\n\
                                   pj-RW\n\
                                   zg-RW\n\
                                   start-pj\n\
                                   he-WI\n\
                                   zg-he\n\
                                   pj-fs\n\
                                   start-RW";

    #[test]
    fn test_sample_p1() {
        let gen = generate(TEST_DATA);
        let res = part1(&gen);
        assert_eq!(res, 10);

        let gen = generate(TEST_DATA_MED);
        let res = part1(&gen);
        assert_eq!(res, 19);

        let gen = generate(TEST_DATA_LARGE);
        let res = part1(&gen);
        assert_eq!(res, 226);
    }

    #[test]
    fn test_sample_p2() {
        let gen = generate(TEST_DATA);
        let res = part2(&gen);
        assert_eq!(res, 36);

        let gen = generate(TEST_DATA_MED);
        let res = part2(&gen);
        assert_eq!(res, 103);

        let gen = generate(TEST_DATA_LARGE);
        let res = part2(&gen);
        assert_eq!(res, 3509);
    }
}
