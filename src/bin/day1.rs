use std::num::ParseIntError;
use std::error::Error;

pub fn parse_input(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.lines().map(str::parse).collect()
}

fn fuel(mass: u32) -> u32 { (mass / 3).saturating_sub(2) }

fn fuel_rec(mass: u32) -> u32 {
    let fuel = fuel(mass);
    if fuel == 0 { fuel } else { fuel + fuel_rec(fuel) }
}

pub fn part1(input: &[u32]) -> u64 {
    input.iter().copied().map(fuel).map(u64::from).sum()
}

pub fn part2(input: &[u32]) -> u64 {
    input.iter().copied().map(fuel_rec).map(u64::from).sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../../input/day1.txt");
    let input = parse_input(input)?;

    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));

    Ok(())
}

#[test]
fn fuel_test() {
    assert_eq!(fuel(12), 2);
    assert_eq!(fuel(14), 2);
    assert_eq!(fuel(1969), 654);
    assert_eq!(fuel(100756), 33583);
}

#[test]
fn fuel_rec_test() {
    assert_eq!(fuel_rec(14), 2);
    assert_eq!(fuel_rec(1969), 966);
    assert_eq!(fuel_rec(100756), 50346);
}