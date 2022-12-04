use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    println!("{}", part_1_count_full_overlaps(&input));
    Ok(())
}

fn part_1_count_full_overlaps(assignment_pairs: &str) -> usize {
    let into_range = |r: &str| {
        let (from, to) = r.split_once('-').unwrap();
        from.parse::<usize>().unwrap()..=to.parse::<usize>().unwrap()
    };

    assignment_pairs
        .lines()
        .map(|pair| {
            let (p1, p2) = pair.split_once(',').unwrap();
            (into_range(p1), into_range(p2))
        })
        .filter(|(p1, p2)| {
            (p1.contains(&p2.clone().next().unwrap()) && p1.contains(&p2.clone().last().unwrap()))
                || (p2.contains(&p1.clone().next().unwrap())
                    && p2.contains(&p1.clone().last().unwrap()))
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
}
