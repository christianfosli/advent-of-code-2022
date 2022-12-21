use std::{collections::HashMap, error::Error, fs, str::FromStr};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    println!("Part 1 root yells: {:?}", part_1_find_root_num(&input));
    Ok(())
}

#[derive(Clone, Debug, PartialEq)]
enum MonkeyJob {
    Number(usize),
    Operation(String),
}

impl FromStr for MonkeyJob {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(num) = s.parse::<usize>() {
            Ok(MonkeyJob::Number(num))
        } else {
            // We could do better validation, but we'll assume it's a valid operation
            Ok(MonkeyJob::Operation(s.to_string()))
        }
    }
}

fn part_1_find_root_num(s: &str) -> Result<usize, Box<dyn Error>> {
    fn monkey_yell(map: &HashMap<&str, MonkeyJob>, name: &str) -> Result<usize, String> {
        let job = map
            .get(name)
            .ok_or_else(|| format!("Monkey {name} not found"))?;

        match job.clone() {
            MonkeyJob::Number(num) => Ok(num),
            MonkeyJob::Operation(op) => {
                if let [monkey1, op, monkey2] = op.split_whitespace().collect::<Vec<_>>()[..] {
                    match op {
                        "+" => Ok(monkey_yell(map, monkey1)? + monkey_yell(map, monkey2)?),
                        "-" => Ok(monkey_yell(map, monkey1)? - monkey_yell(map, monkey2)?),
                        "*" => Ok(monkey_yell(map, monkey1)? * monkey_yell(map, monkey2)?),
                        "/" => Ok(monkey_yell(map, monkey1)? / monkey_yell(map, monkey2)?),
                        _ => Err(format!("Unexpected operation {op}")),
                    }
                } else {
                    unreachable!("Unexpected format for operation")
                }
            }
        }
    }

    let monkey_jobs = s
        .lines()
        .map(|l| {
            let (monkey, job) = l.split_once(": ").ok_or("Unexpected format")?;
            Ok((monkey, job.parse::<MonkeyJob>()?))
        })
        .collect::<Result<HashMap<&str, MonkeyJob>, Box<dyn Error>>>()?;

    Ok(monkey_yell(&monkey_jobs, "root")?)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_MONKEY_JOBS: &'static str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn it_works_example_1() {
        assert_eq!(152, part_1_find_root_num(TEST_MONKEY_JOBS).unwrap())
    }
}
