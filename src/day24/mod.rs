use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{Either, Itertools};
use parse_display::{Display, FromStr};
use std::collections::HashMap;

#[derive(Display, FromStr, Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum Instruction {
    #[display("inp {0}")]
    Input(char),

    #[display("add {0} {1}")]
    AddImm(char, i64),

    #[display("add {0} {1}")]
    Add(char, char),

    #[display("mul {0} {1}")]
    MulImm(char, i64),

    #[display("mul {0} {1}")]
    Mul(char, char),

    #[display("div {0} {1}")]
    DivImm(char, i64),

    #[display("div {0} {1}")]
    Div(char, char),

    #[display("mod {0} {1}")]
    ModImm(char, i64),

    #[display("mod {0} {1}")]
    Mod(char, char),

    #[display("eql {0} {1}")]
    EqImm(char, i64),

    #[display("eql {0} {1}")]
    Eq(char, char),
}

#[aoc_generator(day24)]
pub fn generate(inp: &str) -> Vec<Instruction> {
    inp.lines().filter_map(|it| it.parse().ok()).collect_vec()
}

fn get_reg_index(dest: char) -> usize {
    match dest {
        'w' => 0,
        'x' => 1,
        'y' => 2,
        'z' => 3,
        _ => unreachable!(),
    }
}

#[derive(Default, PartialEq, Eq, Hash, Copy, Clone)]
struct CpuState {
    regs: [i64; 4],
    pc: usize,
}

fn execute_insts(
    state: &mut CpuState,
    insts: &[Instruction],
    cache: &mut HashMap<CpuState, Option<String>>,
    input: i64,
    part_one: bool,
) -> Option<String> {
    while let Some(inst) = insts.get(state.pc) {
        match inst {
            Instruction::Input(_) => {
                if let Some(s) = run_program(*state, insts, cache, part_one) {
                    let res = String::from_iter([input.to_string(), s]);
                    cache.insert(*state, Some(res.clone()));
                    return Some(res);
                }

                return None;
            }
            Instruction::AddImm(dst, val) => {
                let idx = get_reg_index(*dst);
                state.regs[idx] += *val;
            }
            Instruction::Add(dst, src) => {
                let idx = get_reg_index(*dst);
                let val = state.regs[get_reg_index(*src)];
                state.regs[idx] += val;
            }
            Instruction::MulImm(dst, val) => {
                let idx = get_reg_index(*dst);
                state.regs[idx] *= *val;
            }
            Instruction::Mul(dst, src) => {
                let idx = get_reg_index(*dst);
                let val = state.regs[get_reg_index(*src)];
                state.regs[idx] *= val;
            }
            Instruction::DivImm(dst, val) => {
                let idx = get_reg_index(*dst);
                if *val == 0 {
                    cache.insert(*state, None);
                    return None;
                }
                state.regs[idx] /= *val;
            }
            Instruction::Div(dst, src) => {
                let idx = get_reg_index(*dst);
                let val = state.regs[get_reg_index(*src)];
                if val == 0 {
                    cache.insert(*state, None);
                    return None;
                }
                state.regs[idx] /= val;
            }
            Instruction::ModImm(dst, val) => {
                let idx = get_reg_index(*dst);
                if state.regs[idx] < 0 || *val <= 0 {
                    cache.insert(*state, None);
                    return None;
                }
                state.regs[idx] %= *val;
            }
            Instruction::Mod(dst, src) => {
                let idx = get_reg_index(*dst);
                let val = state.regs[get_reg_index(*src)];
                if state.regs[idx] < 0 || val <= 0 {
                    cache.insert(*state, None);
                    return None;
                }
                state.regs[idx] %= val;
            }
            Instruction::EqImm(dst, val) => {
                let idx = get_reg_index(*dst);
                state.regs[idx] = if state.regs[idx] == *val { 1 } else { 0 };
            }
            Instruction::Eq(dst, src) => {
                let idx = get_reg_index(*dst);
                let val = state.regs[get_reg_index(*src)];
                state.regs[idx] = if state.regs[idx] == val { 1 } else { 0 };
            }
        }

        state.pc += 1;
    }

    if state.regs[3] == 0 {
        cache.insert(*state, Some(input.to_string()));
        return Some(input.to_string());
    }

    None
}

fn run_program(
    state: CpuState,
    insts: &[Instruction],
    cache: &mut HashMap<CpuState, Option<String>>,
    part_one: bool,
) -> Option<String> {
    if let Some(cached) = cache.get(&state) {
        return cached.clone();
    }

    let range = if part_one {
        Either::Left((1..=9).rev())
    } else {
        Either::Right(1..=9)
    };

    for w in range {
        let mut state = state;
        state.regs[0] = w;
        state.pc += 1;

        if let Some(res) = execute_insts(&mut state, insts, cache, w, part_one) {
            return Some(res);
        }
    }

    cache.insert(state, None);
    None
}

#[aoc(day24, part1)]
pub fn part1(inp: &[Instruction]) -> Option<String> {
    let result = run_program(CpuState::default(), inp, &mut HashMap::new(), true)?;
    Some(result)
}

#[aoc(day24, part2)]
pub fn part2(inp: &[Instruction]) -> Option<String> {
    let result = run_program(CpuState::default(), inp, &mut HashMap::new(), false)?;
    Some(result)
}
