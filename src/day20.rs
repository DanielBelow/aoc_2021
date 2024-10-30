use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct Input {
    img_enhancement: String,
    image: Vec<Vec<char>>,
}

#[aoc_generator(day20)]
pub fn generate(inp: &str) -> Option<Input> {
    let img_enhancement = inp.lines().next()?.to_string();

    let image = inp
        .lines()
        .skip(2)
        .map(|it| it.chars().collect_vec())
        .collect_vec();

    Some(Input {
        img_enhancement,
        image,
    })
}

fn cube_as_string(image: &[Vec<char>], x: usize, y: usize, outside: char) -> String {
    let height = image.len();
    let width = image[0].len();

    let mut as_str = String::new();

    let top_left = if x > 0 && y > 0 {
        image[x - 1][y - 1]
    } else {
        outside
    };

    as_str.push(top_left);

    let top = if x > 0 { image[x - 1][y] } else { outside };
    as_str.push(top);

    let top_right = if x > 0 && y + 1 < width {
        image[x - 1][y + 1]
    } else {
        outside
    };
    as_str.push(top_right);

    let left = if y > 0 { image[x][y - 1] } else { outside };
    as_str.push(left);

    let mid = image[x][y];
    as_str.push(mid);

    let right = if y + 1 < width {
        image[x][y + 1]
    } else {
        outside
    };
    as_str.push(right);

    let bot_left = if x + 1 < height && y > 0 {
        image[x + 1][y - 1]
    } else {
        outside
    };
    as_str.push(bot_left);

    let bot = if x + 1 < height {
        image[x + 1][y]
    } else {
        outside
    };
    as_str.push(bot);

    let bot_right = if x + 1 < height && y + 1 < width {
        image[x + 1][y + 1]
    } else {
        outside
    };
    as_str.push(bot_right);

    as_str
        .chars()
        .map(|it| match it {
            '#' => '1',
            '.' => '0',
            _ => unreachable!(),
        })
        .join("")
}

fn convert(image: &[Vec<char>], algo: &str, outside: char) -> Vec<Vec<char>> {
    let mut image = image.to_vec();
    image.insert(0, vec![outside; image[0].len()]);
    image.push(vec![outside; image[0].len()]);
    for row in &mut image {
        row.insert(0, outside);
        row.push(outside);
    }

    let mut output_image = Vec::new();
    output_image.resize(image.len(), vec![outside; image[0].len()]);

    for x in 0..image.len() {
        for y in 0..image[x].len() {
            let as_str = cube_as_string(&image, x, y, outside);
            let num = usize::from_str_radix(&as_str, 2).expect("Is binary number");

            let output_pixel = algo.chars().nth(num).expect("'num' is a valid index");
            output_image[x][y] = output_pixel;
        }
    }

    output_image
}

#[aoc(day20, part1)]
pub fn part1(inp: &Input) -> usize {
    let result = convert(&inp.image, &inp.img_enhancement, '.');
    let result = convert(&result, &inp.img_enhancement, '#');

    result.iter().flatten().filter(|it| **it == '#').count()
}

#[aoc(day20, part2)]
pub fn part2(inp: &Input) -> usize {
    let mut outside = '.';

    let mut result = convert(&inp.image, &inp.img_enhancement, outside);
    for _ in 0..49 {
        outside = if outside == '.' { '#' } else { '.' };
        result = convert(&result, &inp.img_enhancement, outside);
    }

    result.iter().flatten().filter(|it| **it == '#').count()
}
