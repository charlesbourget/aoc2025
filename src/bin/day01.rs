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

fn part_1(input: &[String]) -> Result<u32> {
    let mut pos: i16 = 50;
    let mut result = 0;
    for line in input.iter() {
        let (dir, count) = parse_instruction(line)?;

        match dir {
            'R' => pos += count,
            'L' => pos -= count,
            _ => panic!(),
        }

        pos = pos.rem_euclid(100);

        if pos == 0 {
            result += 1;
        }
    }

    Ok(result)
}

fn part_2(input: &[String]) -> Result<u32> {
    let mut pos: i16 = 50;
    let mut result = 0;
    for line in input.iter() {
        let pres_pos = pos;
        let (dir, mut count) = parse_instruction(line)?;

        if count > 100 {
            result += (count / 100) as u32;
            count = count.rem_euclid(100);
        }

        match dir {
            'R' => pos += count,
            'L' => pos -= count,
            _ => panic!(),
        }

        if pos > 99 || (pos <= 0 && pres_pos != 0) {
            result += 1;
        }

        pos = pos.rem_euclid(100);
    }

    Ok(result)
}

fn parse_instruction(line: &str) -> Result<(char, i16)> {
    let inst = line.as_bytes();
    let dir = inst[0] as char;
    let count: i16 = str::from_utf8(&inst[1..])?.parse()?;

    Ok((dir, count))
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
        let expected_result = 3;
        let result = part_1(&test_input).unwrap();
        assert_eq!(expected_result, result);
    }

    #[test]
    fn part_2_test() {
        let test_input = read_input("inputs/day01/input.test.txt").unwrap();
        let expected_result = 6;
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
