#![feature(iter_array_chunks)]
use std::{collections::HashSet, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    println!("Part 1: {}", part_1_sum_pri_common_cpt(&input));
    println!("Part 2: {}", part_2_sum_pri_badges(&input));
    Ok(())
}

trait Prioritizable {
    fn priority(&self) -> u8;
}

impl Prioritizable for char {
    fn priority(&self) -> u8 {
        if self.is_lowercase() {
            (*self as u8) - b'a' + 1u8 // a-z -> 1-26
        } else if self.is_uppercase() {
            (*self as u8) - b'A' + 27u8 // A-Z -> 27-52
        } else {
            unreachable!();
        }
    }
}

fn part_1_sum_pri_common_cpt(rucksacks: &str) -> usize {
    rucksacks
        .lines()
        .map(|rucksack| {
            let (cpt1, cpt2) = rucksack.split_at(rucksack.len() / 2);
            cpt1.chars()
                .collect::<HashSet<_>>()
                .intersection(&cpt2.chars().collect::<HashSet<_>>())
                .next()
                .expect("rucksack compartments had nothing in common")
                .priority() as usize
        })
        .sum()
}

fn part_2_sum_pri_badges(rucksacks: &str) -> usize {
    rucksacks
        .lines()
        .array_chunks()
        .map(|[first, second, third]| {
            first
                .chars()
                .collect::<HashSet<_>>()
                .intersection(&second.chars().collect::<HashSet<_>>())
                .copied()
                .collect::<HashSet<_>>()
                .intersection(&third.chars().collect::<HashSet<_>>())
                .next()
                .expect("group has no badge")
                .priority() as usize
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_RUCKSACKS: &'static str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn priority_works() {
        assert_eq!(1, 'a'.priority());
        assert_eq!(2, 'b'.priority());
        assert_eq!(27, 'A'.priority());
        assert_eq!(28, 'B'.priority());
    }

    #[test]
    fn it_works_with_example_1() {
        assert_eq!(157, part_1_sum_pri_common_cpt(TEST_RUCKSACKS));
    }

    #[test]
    fn it_works_with_example_2() {
        assert_eq!(70, part_2_sum_pri_badges(TEST_RUCKSACKS));
    }
}
