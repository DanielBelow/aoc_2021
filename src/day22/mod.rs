use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};
use std::collections::HashSet;

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Eq, Copy, Clone, Debug)]
#[display("x={from_x}..{to_x},y={from_y}..{to_y},z={from_z}..{to_z}")]
pub struct Instruction {
    from_x: i64,
    to_x: i64,
    from_y: i64,
    to_y: i64,
    from_z: i64,
    to_z: i64,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Input {
    flag: bool,
    instr: Instruction,
}

#[aoc_generator(day22)]
pub fn generate(inp: &str) -> Vec<Input> {
    inp.lines()
        .filter_map(|it| {
            let (flag, instr) = it.split_once(' ')?;
            let flag = flag == "on";
            let instr = instr.trim().parse::<Instruction>().ok()?;
            Some(Input { flag, instr })
        })
        .collect_vec()
}

#[aoc(day22, part1)]
pub fn part1(inp: &[Input]) -> usize {
    let inp = inp
        .iter()
        .filter(|it| {
            it.instr.from_x >= -50
                && it.instr.to_x <= 50
                && it.instr.from_y >= -50
                && it.instr.to_y <= 50
                && it.instr.from_z >= -50
                && it.instr.to_z <= 50
        })
        .copied()
        .collect_vec();

    let mut lights = HashSet::new();

    for iter in inp {
        let Input { flag, instr } = iter;

        for (x, y, z) in iproduct!(
            instr.from_x..=instr.to_x,
            instr.from_y..=instr.to_y,
            instr.from_z..=instr.to_z
        ) {
            if flag {
                lights.insert((x, y, z));
            } else {
                lights.remove(&(x, y, z));
            }
        }
    }

    lights.len()
}

#[aoc(day22, part2)]
pub fn part2(inp: &[Input]) -> i64 {
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    let mut zs = Vec::new();
    for iter in inp.iter().map(|it| it.instr) {
        xs.push(iter.from_x);
        xs.push(iter.to_x + 1);

        ys.push(iter.from_y);
        ys.push(iter.to_y + 1);

        zs.push(iter.from_z);
        zs.push(iter.to_z + 1);
    }

    xs.sort_unstable();
    ys.sort_unstable();
    zs.sort_unstable();

    let mut res = vec![vec![vec![0; zs.len()]; ys.len()]; xs.len()];

    let get_position_of =
        |tgt: i64, v: &[i64]| v.iter().position(|it| *it >= tgt).unwrap_or_default();

    for iter in inp {
        let min_x = get_position_of(iter.instr.from_x, &xs);
        let max_x = get_position_of(iter.instr.to_x + 1, &xs);
        let min_y = get_position_of(iter.instr.from_y, &ys);
        let max_y = get_position_of(iter.instr.to_y + 1, &ys);
        let min_z = get_position_of(iter.instr.from_z, &zs);
        let max_z = get_position_of(iter.instr.to_z + 1, &zs);

        for (xi, yi, zi) in iproduct!(min_x..max_x, min_y..max_y, min_z..max_z) {
            res[xi][yi][zi] = i32::from(iter.flag);
        }
    }

    let mut result = 0;

    for (xi, yi, zi) in iproduct!(0..xs.len() - 1, 0..ys.len() - 1, 0..zs.len() - 1)
        .filter(|(x, y, z)| res[*x][*y][*z] == 1)
    {
        result += (xs[xi + 1] - xs[xi]) * (ys[yi + 1] - ys[yi]) * (zs[zi + 1] - zs[zi]);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "on x=-20..26,y=-36..17,z=-47..7\n\
                             on x=-20..33,y=-21..23,z=-26..28\n\
                             on x=-22..28,y=-29..23,z=-38..16\n\
                             on x=-46..7,y=-6..46,z=-50..-1\n\
                             on x=-49..1,y=-3..46,z=-24..28\n\
                             on x=2..47,y=-22..22,z=-23..27\n\
                             on x=-27..23,y=-28..26,z=-21..29\n\
                             on x=-39..5,y=-6..47,z=-3..44\n\
                             on x=-30..21,y=-8..43,z=-13..34\n\
                             on x=-22..26,y=-27..20,z=-29..19\n\
                             off x=-48..-32,y=26..41,z=-47..-37\n\
                             on x=-12..35,y=6..50,z=-50..-2\n\
                             off x=-48..-32,y=-32..-16,z=-15..-5\n\
                             on x=-18..26,y=-33..15,z=-7..46\n\
                             off x=-40..-22,y=-38..-28,z=23..41\n\
                             on x=-16..35,y=-41..10,z=-47..6\n\
                             off x=-32..-23,y=11..30,z=-14..3\n\
                             on x=-49..-5,y=-3..45,z=-29..18\n\
                             off x=18..30,y=-20..-8,z=-3..13\n\
                             on x=-41..9,y=-7..43,z=-33..15\n\
                             on x=-54112..-39298,y=-85059..-49293,z=-27449..7877\n\
                             on x=967..23432,y=45373..81175,z=27513..53682";

    #[test]
    fn test_sample_p1() {
        let gen = generate(TEST_DATA);
        let res = part1(&gen);
        assert_eq!(res, 590_784);
    }

    const TEST_DATA_2: &str = "on x=-5..47,y=-31..22,z=-19..33\n\
                               on x=-44..5,y=-27..21,z=-14..35\n\
                               on x=-49..-1,y=-11..42,z=-10..38\n\
                               on x=-20..34,y=-40..6,z=-44..1\n\
                               off x=26..39,y=40..50,z=-2..11\n\
                               on x=-41..5,y=-41..6,z=-36..8\n\
                               off x=-43..-33,y=-45..-28,z=7..25\n\
                               on x=-33..15,y=-32..19,z=-34..11\n\
                               off x=35..47,y=-46..-34,z=-11..5\n\
                               on x=-14..36,y=-6..44,z=-16..29\n\
                               on x=-57795..-6158,y=29564..72030,z=20435..90618\n\
                               on x=36731..105352,y=-21140..28532,z=16094..90401\n\
                               on x=30999..107136,y=-53464..15513,z=8553..71215\n\
                               on x=13528..83982,y=-99403..-27377,z=-24141..23996\n\
                               on x=-72682..-12347,y=18159..111354,z=7391..80950\n\
                               on x=-1060..80757,y=-65301..-20884,z=-103788..-16709\n\
                               on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856\n\
                               on x=-52752..22273,y=-49450..9096,z=54442..119054\n\
                               on x=-29982..40483,y=-108474..-28371,z=-24328..38471\n\
                               on x=-4958..62750,y=40422..118853,z=-7672..65583\n\
                               on x=55694..108686,y=-43367..46958,z=-26781..48729\n\
                               on x=-98497..-18186,y=-63569..3412,z=1232..88485\n\
                               on x=-726..56291,y=-62629..13224,z=18033..85226\n\
                               on x=-110886..-34664,y=-81338..-8658,z=8914..63723\n\
                               on x=-55829..24974,y=-16897..54165,z=-121762..-28058\n\
                               on x=-65152..-11147,y=22489..91432,z=-58782..1780\n\
                               on x=-120100..-32970,y=-46592..27473,z=-11695..61039\n\
                               on x=-18631..37533,y=-124565..-50804,z=-35667..28308\n\
                               on x=-57817..18248,y=49321..117703,z=5745..55881\n\
                               on x=14781..98692,y=-1341..70827,z=15753..70151\n\
                               on x=-34419..55919,y=-19626..40991,z=39015..114138\n\
                               on x=-60785..11593,y=-56135..2999,z=-95368..-26915\n\
                               on x=-32178..58085,y=17647..101866,z=-91405..-8878\n\
                               on x=-53655..12091,y=50097..105568,z=-75335..-4862\n\
                               on x=-111166..-40997,y=-71714..2688,z=5609..50954\n\
                               on x=-16602..70118,y=-98693..-44401,z=5197..76897\n\
                               on x=16383..101554,y=4615..83635,z=-44907..18747\n\
                               off x=-95822..-15171,y=-19987..48940,z=10804..104439\n\
                               on x=-89813..-14614,y=16069..88491,z=-3297..45228\n\
                               on x=41075..99376,y=-20427..49978,z=-52012..13762\n\
                               on x=-21330..50085,y=-17944..62733,z=-112280..-30197\n\
                               on x=-16478..35915,y=36008..118594,z=-7885..47086\n\
                               off x=-98156..-27851,y=-49952..43171,z=-99005..-8456\n\
                               off x=2032..69770,y=-71013..4824,z=7471..94418\n\
                               on x=43670..120875,y=-42068..12382,z=-24787..38892\n\
                               off x=37514..111226,y=-45862..25743,z=-16714..54663\n\
                               off x=25699..97951,y=-30668..59918,z=-15349..69697\n\
                               off x=-44271..17935,y=-9516..60759,z=49131..112598\n\
                               on x=-61695..-5813,y=40978..94975,z=8655..80240\n\
                               off x=-101086..-9439,y=-7088..67543,z=33935..83858\n\
                               off x=18020..114017,y=-48931..32606,z=21474..89843\n\
                               off x=-77139..10506,y=-89994..-18797,z=-80..59318\n\
                               off x=8476..79288,y=-75520..11602,z=-96624..-24783\n\
                               on x=-47488..-1262,y=24338..100707,z=16292..72967\n\
                               off x=-84341..13987,y=2429..92914,z=-90671..-1318\n\
                               off x=-37810..49457,y=-71013..-7894,z=-105357..-13188\n\
                               off x=-27365..46395,y=31009..98017,z=15428..76570\n\
                               off x=-70369..-16548,y=22648..78696,z=-1892..86821\n\
                               on x=-53470..21291,y=-120233..-33476,z=-44150..38147\n\
                               off x=-93533..-4276,y=-16170..68771,z=-104985..-24507";

    #[test]
    fn test_sample_p2() {
        let gen = generate(TEST_DATA_2);
        let res = part2(&gen);
        assert_eq!(res, 2_758_514_936_282_235);
    }
}
