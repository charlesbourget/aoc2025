#![feature(test)]

use anyhow::Result;
use aoc2025::utils::read_input_str;

const INPUT: &str = include_str!("../../inputs/day01/input.txt");

fn main() -> Result<()> {
    let lines = read_input_str(INPUT);

    println!("Part 1 => {}", part_1(&lines)?);
    println!("Part 2 => {}", part_2(&lines)?);

    Ok(())
}

fn part_1(_input: &[String]) -> Result<i64> {
    Ok(0)
}

fn part_2(_input: &[String]) -> Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    extern crate test;
    use aoc2025::utils::{read_input, read_input_str};
    use test::Bencher;

    use super::*;

    #[test]
    fn part_1_test() {
        let test_input = read_input("inputs/day01/input.test.txt").unwrap();
        let expected_result = 0;
        let result = part_1(&test_input).unwrap();
        assert_eq!(expected_result, result);
    }

    #[test]
    fn part_2_test() {
        let test_input = read_input("inputs/day01/input.test.txt").unwrap();
        let expected_result = 0;
        let result = part_2(&test_input).unwrap();
        assert_eq!(expected_result, result);
    }

    #[bench]
    fn part_1_bench(b: &mut Bencher) {
        let lines = read_input_str(INPUT);

        b.iter(|| part_1(&lines));
    }

    #[bench]
    fn part_2_bench(b: &mut Bencher) {
        let lines = read_input_str(INPUT);

        b.iter(|| part_2(&lines));
    }
}
