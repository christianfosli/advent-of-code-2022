use std::error::Error;
use std::{fs, iter};

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    println!("Part 1: {:?}", part_1_find_first_start_of_packet(&input));
    Ok(())
}

fn part_1_find_first_start_of_packet(datastream: &str) -> (Option<usize>, Vec<char>) {
    datastream
        .chars()
        .enumerate()
        .fold_while((None, Vec::new()), |(_, mut marker), (i, el)| {
            if marker.iter().all(|&x| x != el) {
                marker.push(el);
                if marker.len() == 4 {
                    Done((Some(i + 1), marker))
                } else {
                    Continue((Some(i + 1), marker))
                }
            } else {
                let (_, rest) = marker.split_at(marker.iter().position(|&x| x == el).unwrap() + 1);
                let marker = rest
                    .into_iter()
                    .cloned()
                    .chain(iter::once(el))
                    .collect::<Vec<_>>();
                Continue((Some(i + 1), marker))
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
        assert_eq!(
            Some(7),
            part_1_find_first_start_of_packet(TEST_DATASTREAM).0
        );
    }
}
