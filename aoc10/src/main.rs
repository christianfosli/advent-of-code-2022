use std::{collections::HashMap, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    println!("Part 1: {}", part_1_sum_signal_strengths(&input));
    println!("Part 2:");
    part_2_draw_crt(&input);
    Ok(())
}

fn part_1_sum_signal_strengths(program: &str) -> isize {
    let mut registry = HashMap::from([('x', 1isize)]);

    program
        .lines()
        .flat_map(|instr| do_instruction(&mut registry, instr)) // cycles
        .enumerate()
        .map(|(i, x)| (i as isize + 1) * x) //signal strength
        .skip(19)
        .step_by(40)
        .sum()
}

fn part_2_draw_crt(program: &str) {
    let mut registry = HashMap::from([('x', 1isize)]);

    for (i, x) in program
        .lines()
        .flat_map(|instr| do_instruction(&mut registry, instr))
        .enumerate()
    {
        let sprite = x - 1..=x + 1;
        let crt_pos = i % 40;
        if crt_pos == 0 {
            println!("");
        }

        if sprite.contains(&(crt_pos as isize)) {
            print!("#");
        } else {
            print!("-");
        }
    }

    println!("");
}

// Returns list of x reg value per CPU cycle used to perform the instruction
fn do_instruction(registry: &mut HashMap<char, isize>, instruction: &str) -> Vec<isize> {
    match instruction {
        "noop" => {
            vec![*registry.get(&'x').unwrap()]
        }
        _ if instruction.starts_with("addx") => {
            let (_, arg) = instruction.split_once(' ').unwrap();
            let arg = arg.parse::<isize>().unwrap();
            let initial = *registry.get(&'x').unwrap();
            registry.entry('x').and_modify(|x| *x += arg);

            vec![initial, initial]
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_simple_example() {
        let mut registry = HashMap::from([('x', 1isize)]);
        let program = vec!["noop", "addx 3", "addx -5"];
        let cycles = program
            .iter()
            .flat_map(|instr| do_instruction(&mut registry, instr))
            .collect::<Vec<_>>();

        assert_eq!(1, cycles[0]);
        assert_eq!(1, cycles[1]);
        assert_eq!(1, cycles[2]);
        assert_eq!(4, cycles[3]);
        assert_eq!(4, cycles[4]);
        assert_eq!(-1, *registry.get(&'x').unwrap())
    }

    const PROGRAM: &'static str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn it_works_with_example_1() {
        assert_eq!(13140, part_1_sum_signal_strengths(PROGRAM));
    }

    #[test]
    fn it_works_with_example_2() {
        part_2_draw_crt(PROGRAM);
    }
}
