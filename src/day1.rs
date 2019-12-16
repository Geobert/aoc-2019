use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn read_masses(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|l| usize::from_str_radix(l, 10).expect("Input can't be converted to integer"))
        .collect()
}

#[aoc(day1, part1)]
pub fn solve(input: &[usize]) -> u32 {
    input.iter().fold(0, |fuel, mass| {
        fuel + ((*mass as f32 / 3f32).floor() - 2f32) as u32
    })
}
