use std::error::Error;
use std::{fs, iter};

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    println!("Part 1: {:?}", part_1_find_start_of_packet(&input));
    println!("Part 2: {:?}", part_2_find_start_of_msg(&input));
    Ok(())
}

fn part_1_find_start_of_packet(datastream: &str) -> (Option<usize>, Vec<char>) {
    find_first_unique::<4>(datastream)
}

fn part_2_find_start_of_msg(datastream: &str) -> (Option<usize>, Vec<char>) {
    find_first_unique::<14>(datastream)
}

fn find_first_unique<const N: usize>(s: &str) -> (Option<usize>, Vec<char>) {
    s.chars()
        .enumerate()
        .fold_while((None, Vec::new()), |(_, mut marker), (i, el)| {
            if marker.iter().all(|&x| x != el) {
                marker.push(el);
                if marker.len() == N {
                    Done((Some(i + 1), marker))
                } else {
                    Continue((None, marker))
                }
            } else {
                let (_, rest) = marker.split_at(marker.iter().position(|&x| x == el).unwrap() + 1);
                let marker = rest
                    .iter()
                    .copied()
                    .chain(iter::once(el))
                    .collect::<Vec<_>>();
                Continue((None, marker))
            }
        })
        .into_inner()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATASTREAM: &'static str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn it_works_with_example_1() {
        assert_eq!(Some(7), part_1_find_start_of_packet(TEST_DATASTREAM).0);
    }

    #[test]
    fn it_works_with_example_2() {
        assert_eq!(Some(19), part_2_find_start_of_msg(TEST_DATASTREAM).0);
    }
}
