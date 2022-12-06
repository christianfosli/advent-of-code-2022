use std::{array, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let (drawing, rearrangement) = input.split_once("\n\n").unwrap();
    let stacks = parse_stacks::<9>(drawing);
    let stacks1 = rearrange_1(&stacks, rearrangement);
    println!(
        "part 1: {:?}",
        stacks1
            .iter()
            .map(|x| x.last().unwrap())
            .collect::<Vec<_>>()
    );

    let stacks2 = rearrange_2(&stacks, rearrangement);
    println!(
        "part 2: {:?}",
        stacks2
            .iter()
            .map(|x| x.last().unwrap())
            .collect::<Vec<_>>()
    );

    Ok(())
}

fn parse_stacks<const N: usize>(drawing: &str) -> [Vec<char>; N] {
    let mut stacks = array::from_fn(|_| Vec::new());
    let offsets = drawing
        .lines()
        .last()
        .unwrap()
        .char_indices()
        .filter(|&(_, c)| c.is_digit(10))
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    for i in 0..N {
        let offset = offsets[i];

        for crt in drawing.lines().filter_map(|l| {
            if let Some(crt) = l.chars().nth(offset) {
                if crt.is_alphabetic() {
                    Some(crt)
                } else {
                    None
                }
            } else {
                None
            }
        }) {
            stacks[i].insert(0, crt);
        }
    }

    stacks
}

fn rearrange_1(stacks: &[Vec<char>], rearrangements: &str) -> Vec<Vec<char>> {
    let mut stacks = stacks.to_vec();
    for cmd in rearrangements.lines() {
        if let [count, from, to] = cmd
            .split_whitespace()
            .filter_map(|str| str.parse::<usize>().ok())
            .collect::<Vec<_>>()[..]
        {
            for _ in 0..count {
                if let Some(crt) = stacks[from - 1].pop() {
                    stacks[to - 1].push(crt);
                } else {
                    unreachable!();
                }
            }
        } else {
            unreachable!();
        }
    }
    stacks
}

fn rearrange_2(stacks: &[Vec<char>], rearrangements: &str) -> Vec<Vec<char>> {
    let mut stacks = stacks.to_vec();
    for cmd in rearrangements.lines() {
        if let [count, from, to] = cmd
            .split_whitespace()
            .filter_map(|str| str.parse::<usize>().ok())
            .collect::<Vec<_>>()[..]
        {
            let mut moving = vec![];
            for _ in 0..count {
                if let Some(crt) = stacks[from - 1].pop() {
                    moving.push(crt);
                    // stacks[to - 1].push(crt);
                } else {
                    unreachable!();
                }
            }
            for crt in moving.into_iter().rev() {
                stacks[to - 1].push(crt);
            }
        } else {
            unreachable!();
        }
    }
    stacks
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_REARRANGEMENT_DRAWING: &'static str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn it_works_with_example_1() {
        let (drawing, rearrangement) = TEST_REARRANGEMENT_DRAWING.split_once("\n\n").unwrap();
        let stacks = parse_stacks::<3>(drawing);
        println!(
            "{:?}",
            rearrange_1(&stacks, rearrangement)
                .iter()
                .map(|crt| format!("{:?}", crt.last()))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn it_works_with_example_2() {
        let (drawing, rearrangement) = TEST_REARRANGEMENT_DRAWING.split_once("\n\n").unwrap();
        let stacks = parse_stacks::<3>(drawing);
        println!(
            "{:?}",
            rearrange_2(&stacks, rearrangement)
                .iter()
                .map(|crt| format!("{:?}", crt.last()))
                .collect::<Vec<_>>()
        );
    }
}
