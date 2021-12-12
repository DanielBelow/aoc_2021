use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Eq, PartialEq, Clone, Debug)]
#[display("{from}-{to}")]
pub struct Path {
    from: String,
    to: String,
}

#[aoc_generator(day12)]
pub fn generate(inp: &str) -> Vec<Path> {
    let res = inp
        .lines()
        .filter_map(|it| it.parse::<Path>().ok())
        .collect_vec();

    let other_dir = res
        .iter()
        .filter(|it| it.from != "start" && it.to != "end")
        .map(|it| Path {
            from: it.to.clone(),
            to: it.from.clone(),
        })
        .collect_vec();
    res.iter().chain(other_dir.iter()).cloned().collect()
}

fn collect_paths_from(
    cur: &str,
    inp: &[Path],
    cur_path: &[String],
    result: &mut Vec<Vec<String>>,
    can_visit: fn(&str, &[String]) -> bool,
) {
    if cur == "end" {
        result.push(cur_path.to_owned());
        return;
    }

    for available_path in inp
        .iter()
        .filter(|it| it.from.eq(cur) && can_visit(&it.to, cur_path))
    {
        let mut cur_path = cur_path.to_owned();
        cur_path.push(available_path.to.clone());
        collect_paths_from(&available_path.to, inp, &cur_path, result, can_visit);
    }
}

fn find_paths(inp: &[Path], can_visit: fn(&str, &[String]) -> bool) -> Vec<Vec<String>> {
    let mut result = Vec::new();

    let start = inp
        .iter()
        .find(|it| it.from == "start")
        .expect("Start node exists");

    collect_paths_from(
        &start.from,
        inp,
        &[start.from.clone()],
        &mut result,
        can_visit,
    );

    result
}

#[aoc(day12, part1)]
pub fn part1(inp: &[Path]) -> usize {
    let can_visit_cave = |node: &str, paths: &[String]| {
        !node.chars().all(char::is_lowercase) || !paths.iter().any(|it| it.eq(node))
    };

    let paths = find_paths(inp, can_visit_cave);
    paths.len()
}

#[aoc(day12, part2)]
pub fn part2(inp: &[Path]) -> usize {
    let can_visit_cave = |node: &str, paths: &[String]| {
        if node == "start" {
            return false;
        }

        if !node.chars().all(char::is_lowercase) {
            return true;
        }

        let contains_node = paths.iter().any(|it| it.eq(node));
        if !contains_node {
            return true;
        }

        paths
            .iter()
            .filter(|&it| it.chars().all(char::is_lowercase))
            .all_unique()
    };

    let paths = find_paths(inp, can_visit_cave);
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
