use aoc_runner_derive::{aoc, aoc_generator};
use itertools::iproduct;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Copy, Clone, Debug)]
#[display("target area: x={from_x}..{to_x}, y={from_y}..{to_y}")]
pub struct TargetArea {
    from_x: i64,
    from_y: i64,
    to_x: i64,
    to_y: i64,
}

impl TargetArea {
    const fn contains(&self, x: i64, y: i64) -> bool {
        self.from_x <= x && x <= self.to_x && self.from_y <= y && y <= self.to_y
    }

    const fn is_above(&self, y: i64) -> bool {
        y < self.from_y
    }

    fn get_valid_x_velocities(&self) -> Vec<i64> {
        let mut result = Vec::new();

        for x_vel in 0..=self.to_x {
            let mut probe = Probe::new(x_vel, 0);

            let mut hit_range = false;
            while probe.x_vel != 0 && !hit_range {
                do_step(&mut probe);
                hit_range = self.from_x <= probe.x_pos && probe.x_pos <= self.to_x;
            }

            if hit_range {
                result.push(x_vel);
            }
        }

        result
    }
}

pub struct Input {
    area: TargetArea,
    valid_x_velocities: Vec<i64>,
}

struct Probe {
    x_pos: i64,
    y_pos: i64,

    x_vel: i64,
    y_vel: i64,
}

impl Probe {
    const fn new(x_vel: i64, y_vel: i64) -> Self {
        Self {
            x_vel,
            y_vel,
            x_pos: 0,
            y_pos: 0,
        }
    }
}

#[aoc_generator(day17)]
pub fn generate(inp: &str) -> Option<Input> {
    let area = inp
        .lines()
        .next()
        .and_then(|it| it.parse::<TargetArea>().ok())?;
    let valid_x_velocities = area.get_valid_x_velocities();

    Some(Input {
        area,
        valid_x_velocities,
    })
}

fn do_step(probe: &mut Probe) {
    probe.x_pos += probe.x_vel;
    probe.y_pos += probe.y_vel;

    probe.x_vel -= probe.x_vel.signum();
    probe.y_vel -= 1;
}

#[aoc(day17, part1)]
pub fn part1(inp: &Input) -> Option<i64> {
    let Input {
        area,
        valid_x_velocities,
    } = inp;

    iproduct!(valid_x_velocities, -250..=250)
        .filter_map(|(&x_vel, y_vel)| {
            let mut probe = Probe::new(x_vel, y_vel);

            let mut cur_max_y = i64::MIN;
            loop {
                do_step(&mut probe);

                if area.is_above(probe.y_pos) {
                    return None;
                }

                cur_max_y = cur_max_y.max(probe.y_pos);
                if area.contains(probe.x_pos, probe.y_pos) {
                    return Some(cur_max_y);
                }
            }
        })
        .max()
}

#[aoc(day17, part2)]
pub fn part2(inp: &Input) -> u64 {
    let Input {
        area,
        valid_x_velocities,
    } = inp;

    iproduct!(valid_x_velocities, -250..=250)
        .filter_map(|(&x_vel, y_vel)| {
            let mut probe = Probe::new(x_vel, y_vel);
            loop {
                do_step(&mut probe);

                if area.is_above(probe.y_pos) {
                    return None;
                }

                if area.contains(probe.x_pos, probe.y_pos) {
                    return Some(1);
                }
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_sample_p1() {
        let Some(gen) = generate(TEST_DATA) else {
            panic!("Could not parse test input")
        };

        let res = part1(&gen);
        assert_eq!(res, Some(45));
    }

    #[test]
    fn test_sample_p2() {
        let Some(gen) = generate(TEST_DATA) else {
            panic!("Could not parse test input")
        };

        let res = part2(&gen);
        assert_eq!(res, 112);
    }
}
