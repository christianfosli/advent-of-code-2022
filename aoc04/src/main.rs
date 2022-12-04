use std::{collections::HashSet, error::Error, fs, ops::RangeInclusive};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    println!("1 - full overlaps: {}", part_1_count_full_overlaps(&input));
    println!("2 - any overlaps: {}", part_2_count_any_overlaps(&input));
    Ok(())
}

fn to_range_pair(pair: &str) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
    let into_range = |r: &str| {
        let (from, to) = r.split_once('-').unwrap();
        from.parse::<usize>().unwrap()..=to.parse::<usize>().unwrap()
    };

    let (p1, p2) = pair.split_once(',').unwrap();
    (into_range(p1), into_range(p2))
}

fn part_1_count_full_overlaps(assignment_pairs: &str) -> usize {
    assignment_pairs
        .lines()
        .map(to_range_pair)
        .filter(|(p1, p2)| {
            (p1.contains(&p2.clone().next().unwrap()) && p1.contains(&p2.clone().last().unwrap()))
                || (p2.contains(&p1.clone().next().unwrap())
                    && p2.contains(&p1.clone().last().unwrap()))
        })
        .count()
}

fn part_2_count_any_overlaps(assignment_pairs: &str) -> usize {
    assignment_pairs
        .lines()
        .map(to_range_pair)
        .filter(|(p1, p2)| {
            p1.clone()
                .collect::<HashSet<_>>()
                .intersection(&p2.clone().collect::<HashSet<_>>())
                .next()
                .is_some()
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ASSIGNMENT_PAIRS: &'static str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn it_works_with_example_1() {
        assert_eq!(2, part_1_count_full_overlaps(TEST_ASSIGNMENT_PAIRS));
    }

    #[test]
    fn it_works_with_example_2() {
        assert_eq!(4, part_2_count_any_overlaps(TEST_ASSIGNMENT_PAIRS));
    }
}
