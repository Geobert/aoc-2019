use anyhow::{bail, Result};
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn str_to_vec(input: &str) -> Vec<u64> {
    input
        .split(',')
        .map(|i| u64::from_str_radix(i, 10).expect("Invalid input, can't be converted to u64"))
        .collect()
}

fn exec_opcode(instruction: &[u64], mem: &mut [u64]) -> bool {
    let op = instruction[0];
    match op {
        1 => {
            let v1 = mem[instruction[1] as usize];
            let v2 = mem[instruction[2] as usize];
            mem[instruction[3] as usize] = v1 + v2;
            true
        }
        2 => {
            let v1 = mem[instruction[1] as usize];
            let v2 = mem[instruction[2] as usize];
            mem[instruction[3] as usize] = v1 * v2;
            true
        }
        _ => false,
    }
}

fn run_program(mut memory: &mut [u64]) -> u64 {
    let mut ip = 0;
    let mut inst = memory[ip..=ip + 3].to_vec();
    while exec_opcode(&inst, &mut memory) {
        ip += 4;
        inst = memory[ip..=ip + 3].to_vec();
    }
    memory[0]
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[u64]) -> u64 {
    let mut memory = input.to_vec();
    memory[1] = 12;
    memory[2] = 2;
    run_program(&mut memory)
}

fn find_noun_verb(input: &[u64]) -> Result<(u64, u64)> {
    let expected_result = 19690720;
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut memory = input.to_vec();
            memory[1] = noun;
            memory[2] = verb;
            if run_program(&mut memory) == expected_result {
                return Ok((noun, verb));
            }
        }
    }
    bail!("Can't find noun and verb that verifies 19690720")
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[u64]) -> u64 {
    if let Ok((noun, verb)) = find_noun_verb(&input) {
        100 * noun + verb
    } else {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn exec_opcode_test() {
        let mut memory = [1u64, 4, 5, 3, 3, 5, 99];
        let expected_memory = [1u64, 4, 5, 8, 3, 5, 99];
        let mut instruction = [0; 4];
        instruction.clone_from_slice(&memory[..=3]);
        exec_opcode(&instruction, &mut memory);
        assert_eq!(expected_memory, memory)
    }
}
