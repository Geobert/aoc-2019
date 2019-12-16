use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn read_masses(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|l| u64::from_str_radix(l, 10).expect("Input can't be converted to integer"))
        .collect()
}

fn fuel_for_mass(mass: u64) -> u64 {
    ((mass as f64 / 3f64).floor() as u64)
        .checked_sub(2u64)
        .unwrap_or(0)
}

fn fuel_for_module(module_mass: u64) -> u64 {
    let mut module_fuel = fuel_for_mass(module_mass);
    let mut new_mass = module_fuel;
    loop {
        let f = fuel_for_mass(new_mass);
        if f > 0 {
            module_fuel += f;
            new_mass = f;
        } else {
            break;
        };
    }
    module_fuel
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u64]) -> u64 {
    input
        .iter()
        .fold(0, |fuel, mass| fuel + fuel_for_mass(*mass))
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u64]) -> u64 {
    input
        .iter()
        .fold(0, |fuel, mass| fuel + fuel_for_module(*mass))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fuel_for_module_test() {
        assert_eq!(966, fuel_for_module(1969))
    }
}
