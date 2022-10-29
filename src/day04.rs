use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Clone)]
struct Board {
    numbers: Vec<Vec<(i64, bool)>>,
    winning_number: Option<i64>,
}

#[derive(Clone)]
pub struct Bingo {
    drawn_numbers: Vec<i64>,
    boards: Vec<Board>,
}

#[aoc_generator(day4)]
pub fn generate(inp: &str) -> Bingo {
    let mut lines = inp.lines();
    let drawn_numbers = lines
        .next()
        .map(|it| {
            it.split(',')
                .filter_map(|it| it.parse::<i64>().ok())
                .collect_vec()
        })
        .unwrap_or_default();

    let mut boards = Vec::new();

    let all_board_lines = lines.skip_while(|it| it.is_empty()).join("\n");
    for s in all_board_lines.split("\n\n") {
        let board_numbers = s.split('\n').collect_vec();

        let board = board_numbers.iter().fold(Vec::new(), |mut acc, it| {
            let nums = it
                .split(' ')
                .filter_map(|it| it.parse::<i64>().ok())
                .zip(std::iter::repeat(false))
                .collect_vec();
            acc.push(nums);
            acc
        });

        boards.push(Board {
            numbers: board,
            winning_number: None,
        });
    }

    Bingo {
        drawn_numbers,
        boards,
    }
}

fn has_winning_row(board: &Board) -> bool {
    board
        .numbers
        .iter()
        .any(|it| it.iter().all(|(_, is_set)| *is_set))
}

fn has_winning_col(board: &Board) -> bool {
    (0..5).any(|col| board.numbers.iter().map(|it| it[col]).all(|(_, it)| it))
}

fn is_winning(board: &Board) -> bool {
    has_winning_row(board) || has_winning_col(board)
}

fn sum_unmarked(board: &Board) -> i64 {
    let mut unmarked = 0;
    for row in &board.numbers {
        for (num, it) in row {
            if !it {
                unmarked += num;
            }
        }
    }

    unmarked
}

#[aoc(day4, part1)]
pub fn part1(inp: &Bingo) -> Option<i64> {
    let mut boards = inp.boards.clone();

    for &drawn_number in &inp.drawn_numbers {
        for board in &mut boards {
            for row in &mut board.numbers {
                for (num, flag) in row {
                    if *num == drawn_number {
                        *flag = true;
                    }
                }
            }

            if is_winning(board) {
                let unmarked = sum_unmarked(board);
                return Some(drawn_number * unmarked);
            }
        }
    }

    None
}

#[aoc(day4, part2)]
pub fn part2(inp: &Bingo) -> Option<i64> {
    let mut boards = inp.boards.clone();

    let mut winning_boards = Vec::new();

    for &drawn_number in &inp.drawn_numbers {
        for board in boards.iter_mut().filter(|it| it.winning_number.is_none()) {
            for row in &mut board.numbers {
                for (num, flag) in row {
                    if *num == drawn_number {
                        *flag = true;
                    }
                }
            }

            if is_winning(board) {
                board.winning_number = Some(drawn_number);
                winning_boards.push(board.clone());
            }
        }
    }

    winning_boards
        .last()
        .and_then(|wb| wb.winning_number.map(|wn| wn * sum_unmarked(wb)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\
\n\
                       22 13 17 11  0\n\
                        8  2 23  4 24\n\
                       21  9 14 16  7\n\
                        6 10  3 18  5\n\
                        1 12 20 15 19\n\
\n\
                        3 15  0  2 22\n\
                        9 18 13 17  5\n\
                       19  8  7 25 23\n\
                       20 11 10 24  4\n\
                       14 21 16 12  6\n\
\n\
                       14 21 17 24  4\n\
                       10 16 15  9 19\n\
                       18  8 23 26 20\n\
                       22 11 13  6  5\n\
                        2  0 12  3  7";

    #[test]
    fn test_sample_p1() {
        let gen = generate(TEST_DATA);
        let res = part1(&gen);
        assert_eq!(res, Some(4512));
    }

    #[test]
    fn test_sample_p2() {
        let gen = generate(TEST_DATA);
        let res = part2(&gen);
        assert_eq!(res, Some(1924));
    }
}
