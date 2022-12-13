use std::{num::ParseIntError, str::FromStr};

fn main() {
    println!("Hello, world!");
}

#[derive(Clone, Debug)]
enum PacketData {
    List(Vec<PacketData>),
    Int(usize),
}

impl FromStr for PacketData {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 1 && s.chars().next().unwrap().is_digit(10) {
            Ok(PacketData::Int(s.parse::<usize>()?))
        } else if s.starts_with('[') && s.ends_with(']') {
            let content = s.strip_prefix('[').unwrap().strip_suffix(']').unwrap();
            Ok(match content {
                "" => PacketData::List(Vec::new()),
                _ => {
                    // Aaargh, we can't just split by comma and recursively process
                    // items because the nested lists also contain commas.
                    // TODO (WIP): Split by only commas which are not inside brackets,
                    // process each of those items and combine the results.
                    let mut parsed = Vec::new();
                    let mut inside_bracket = 0;
                    let mut nested_list = "".to_string();
                    for item in content.split(',') {
                        if inside_bracket == 0 && item.len() == 1 {
                            parsed.push(item.parse::<PacketData>()?);
                        } else if inside_bracket > 0 && item.len() == 1 {
                            nested_list += &format!(",{}", item);
                        }
                    }
                    PacketData::List(parsed)
                }
            })
        } else {
            dbg!(&s);
            unimplemented!();
        }
    }
}

fn part_1_sum_correct_indices(packets: &str) -> usize {
    let packets = packets.split("\n\n").map(|x| {
        if let [left, right] = x.lines().collect::<Vec<_>>()[..] {
        } else {
            unreachable!()
        }
    });
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_PACKETS: &'static str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn it_parses_simple_packets() {
        let parsed = "[1,1,3]".parse::<PacketData>();
        let parsed2 = "[]".parse::<PacketData>();
        println!("{:?}\n{:?}", parsed, parsed2);
    }

    #[test]
    fn it_parses_nested_packets() {
        let parsed = "[[1],[2,3,4]]".parse::<PacketData>();
        println!("{:?}", parsed);
    }

    // #[test]
    // fn it_works_with_example_1() {
    //     assert_eq!(13, part_1_sum_correct_indices(TEST_PACKETS));
    // }
}
