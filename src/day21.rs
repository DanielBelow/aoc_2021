use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct GameState {
    positions: [u64; 2],
    scores: [u64; 2],
    player_turn: usize,
}

const POSSIBLE_ROLL_RESULTS: [u64; 27] = [
    3, 4, 5, 4, 5, 6, 5, 6, 7, 4, 5, 6, 5, 6, 7, 6, 7, 8, 5, 6, 7, 6, 7, 8, 7, 8, 9,
];

#[aoc_generator(day21)]
pub fn generate(inp: &str) -> Option<[u64; 2]> {
    let mut lines = inp.lines();
    let mut parse_starting_pos = || {
        lines.next().and_then(|it| {
            let (_, end) = it.rsplit_once(' ')?;
            end.parse::<u64>().ok()
        })
    };
    let p1_pos = parse_starting_pos()?;
    let p2_pos = parse_starting_pos()?;

    Some([p1_pos, p2_pos])
}

fn play_game(mut state: GameState) -> Option<u64> {
    let mut num_rolls = 0;

    loop {
        let rolls = (1..=100).cycle().skip(num_rolls).take(3).sum::<u64>();
        num_rolls += 3;

        state.positions[state.player_turn] += rolls;
        while state.positions[state.player_turn] > 10 {
            state.positions[state.player_turn] -= 10;
        }
        state.scores[state.player_turn] += state.positions[state.player_turn];

        if state.scores[state.player_turn] >= 1000 {
            break;
        }

        state.player_turn = (state.player_turn + 1) % 2;
    }

    Some((num_rolls as u64) * state.scores.iter().min()?)
}

fn play_all_games_impl(state: GameState, cache: &mut HashMap<GameState, [u64; 2]>) -> [u64; 2] {
    if let Some(cached) = cache.get(&state) {
        return *cached;
    }

    let mut total_score = [0; 2];

    let current_player = state.player_turn;

    for roll in POSSIBLE_ROLL_RESULTS {
        let mut new_state = state;
        new_state.player_turn = (current_player + 1) % 2;

        new_state.positions[current_player] += roll;
        if new_state.positions[current_player] > 10 {
            new_state.positions[current_player] -= 10;
        }

        new_state.scores[current_player] += new_state.positions[current_player];

        if new_state.scores[current_player] >= 21 {
            total_score[current_player] += 1;
            continue;
        }

        let [p1, p2] = play_all_games_impl(new_state, cache);

        total_score[0] += p1;
        total_score[1] += p2;
    }

    cache.insert(state, total_score);

    total_score
}

fn play_all_games(state: GameState) -> u64 {
    let mut cache = HashMap::new();
    let [p1, p2] = play_all_games_impl(state, &mut cache);
    p1.max(p2)
}

#[aoc(day21, part1)]
pub fn part1(inp: &[u64; 2]) -> Option<u64> {
    let state = GameState {
        positions: *inp,
        scores: [0; 2],
        player_turn: 0,
    };

    play_game(state)
}

#[aoc(day21, part2)]
pub fn part2(inp: &[u64; 2]) -> u64 {
    let state = GameState {
        positions: *inp,
        scores: [0; 2],
        player_turn: 0,
    };

    play_all_games(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let inp = &[4, 8];
        let res = part1(inp);
        assert_eq!(res, Some(739_785));
    }

    #[test]
    fn test_sample_p2() {
        let inp = &[4, 8];
        let res = part2(inp);
        assert_eq!(res, 444_356_092_776_315);
    }
}
