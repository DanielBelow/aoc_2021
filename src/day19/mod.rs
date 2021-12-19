// https://github.com/AxlLind/AdventOfCode2021/blob/main/src/bin/19.rs

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::FromStr;
use std::collections::HashSet;

#[derive(FromStr, Hash, PartialEq, Eq, Copy, Clone)]
#[display("{x},{y},{z}")]
pub struct Coords {
    x: i64,
    y: i64,
    z: i64,
}

impl std::ops::Add for Coords {
    type Output = Coords;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub for Coords {
    type Output = Coords;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Coords {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn distance(&self, other: &Coords) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    fn get_rotation(&self, num: usize) -> Self {
        let (x, y, z) = (self.x, self.y, self.z);

        match num {
            0 => Self::new(x, y, z),
            1 => Self::new(x, z, -y),
            2 => Self::new(x, -y, -z),
            3 => Self::new(x, -z, y),
            4 => Self::new(y, x, -z),
            5 => Self::new(y, z, x),
            6 => Self::new(y, -x, z),
            7 => Self::new(y, -z, -x),
            8 => Self::new(z, x, y),
            9 => Self::new(z, y, -x),
            10 => Self::new(z, -x, -y),
            11 => Self::new(z, -y, x),
            12 => Self::new(-x, y, -z),
            13 => Self::new(-x, z, y),
            14 => Self::new(-x, -y, z),
            15 => Self::new(-x, -z, -y),
            16 => Self::new(-y, x, z),
            17 => Self::new(-y, z, -x),
            18 => Self::new(-y, -x, -z),
            19 => Self::new(-y, -z, x),
            20 => Self::new(-z, x, -y),
            21 => Self::new(-z, y, x),
            22 => Self::new(-z, -x, y),
            23 => Self::new(-z, -y, -x),
            _ => unreachable!(),
        }
    }
}

#[aoc_generator(day19)]
pub fn generate(inp: &str) -> Vec<Vec<Coords>> {
    inp.split("\n\n")
        .map(|s| {
            s.lines()
                .filter_map(|it| it.parse::<Coords>().ok())
                .collect_vec()
        })
        .collect()
}

fn try_rotation(total_scan: &mut HashSet<Coords>, scan: &[Coords], n: usize) -> Option<Coords> {
    let nth_rotation = scan.iter().map(|&v| v.get_rotation(n)).collect_vec();
    let distances = total_scan
        .iter()
        .cartesian_product(&nth_rotation)
        .map(|(l, r)| *l - *r);

    for dist in distances {
        let translated = nth_rotation.iter().map(|it| *it + dist);
        if translated
            .clone()
            .filter(|v| total_scan.contains(v))
            .count()
            >= 12
        {
            total_scan.extend(translated);
            return Some(dist);
        }
    }

    None
}

fn merge_scan(total_scan: &mut HashSet<Coords>, scan: &[Coords]) -> Option<Coords> {
    (0..24).find_map(|r| try_rotation(total_scan, scan, r))
}

fn split_first_scan(inp: &mut Vec<Vec<Coords>>) -> HashSet<Coords> {
    inp.remove(0).into_iter().collect()
}

#[aoc(day19, part1)]
pub fn part1(inp: &[Vec<Coords>]) -> usize {
    let mut inp = inp.to_vec();

    let mut scans = split_first_scan(&mut inp);
    while !inp.is_empty() {
        for idx in (0..inp.len()).rev() {
            let elem = &inp[idx];
            if merge_scan(&mut scans, elem).is_some() {
                inp.swap_remove(idx);
            }
        }
    }

    scans.len()
}

#[aoc(day19, part2)]
pub fn part2(inp: &[Vec<Coords>]) -> Option<i64> {
    let mut inp = inp.to_vec();
    let mut scans = split_first_scan(&mut inp);
    let mut dists = Vec::new();

    while !inp.is_empty() {
        for idx in (0..inp.len()).rev() {
            let elem = &inp[idx];
            if let Some(d) = merge_scan(&mut scans, elem) {
                inp.swap_remove(idx);
                dists.push(d);
            }
        }
    }

    dists
        .iter()
        .tuple_combinations()
        .map(|(lhs, rhs)| lhs.distance(rhs))
        .max()
}
