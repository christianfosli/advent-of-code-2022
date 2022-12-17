use std::{cmp::Ordering, error::Error, fs, num::ParseIntError, str::FromStr};

use once_cell::sync::Lazy;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    println!("Part 1: {}", part_1_sum_correct_indices(&input));
    println!("Part 2: {}", part_2_find_decoder_key(&input));
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum PacketData {
    List(Vec<PacketData>),
    Int(usize),
}

impl FromStr for PacketData {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.parse::<usize>().is_ok() {
            Ok(PacketData::Int(s.parse::<usize>()?))
        } else if s.starts_with('[') && s.ends_with(']') {
            let content = s.strip_prefix('[').unwrap().strip_suffix(']').unwrap();
            Ok(match content {
                "" => PacketData::List(Vec::new()),
                _ => {
                    // We can't just .split(',').map(|item| item.parse()) because nested lists also contain commas.
                    // This tries to split by comma but "rejoin" nested lists and process recursively.

                    // After thought: D'uh we should have just used serde to parse as json :-D
                    let mut parsed = Vec::new();
                    let mut inside_bracket = 0;
                    let mut nested_list = "".to_string();
                    for item in content.split(',') {
                        let is_inside_bracket = inside_bracket > 0;

                        if !is_inside_bracket && item.parse::<usize>().is_ok() {
                            parsed.push(item.parse::<PacketData>()?);
                            continue;
                        }

                        if !is_inside_bracket {
                            let open_bracket_count = item.chars().filter(|&c| c == '[').count();
                            let close_bracket_count = item.chars().filter(|&c| c == ']').count();

                            if open_bracket_count == close_bracket_count {
                                parsed.push(item.parse::<PacketData>()?);
                            } else {
                                inside_bracket +=
                                    open_bracket_count as isize - close_bracket_count as isize;
                                nested_list += item;
                            }
                        }

                        if is_inside_bracket {
                            nested_list += &format!(",{}", item);
                            let open_bracket_count = item.chars().filter(|&c| c == '[').count();
                            let close_bracket_count = item.chars().filter(|&c| c == ']').count();
                            inside_bracket +=
                                open_bracket_count as isize - close_bracket_count as isize;

                            if inside_bracket == 0 {
                                parsed.push(nested_list.parse::<PacketData>()?);
                                nested_list = "".to_string();
                            }
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

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Int(left), Self::Int(right)) => left.partial_cmp(right),
            (Self::List(left), Self::List(right)) => Some(
                left.iter()
                    .zip(right.iter())
                    .map(|(l, r)| l.cmp(r))
                    .find(|&ord| ord != Ordering::Equal)
                    .unwrap_or_else(|| left.len().cmp(&right.len())),
            ),
            (Self::Int(_), Self::List(_)) => Self::List(vec![self.clone()]).partial_cmp(other),
            (Self::List(_), Self::Int(_)) => self.partial_cmp(&Self::List(vec![other.clone()])),
        }
    }
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn part_1_sum_correct_indices(packets: &str) -> usize {
    packets
        .split("\n\n")
        .map(|x| {
            if let [left, right] = &x
                .lines()
                .map(|packet| packet.parse::<PacketData>().unwrap())
                .collect::<Vec<_>>()[..]
            {
                (left.clone(), right.clone())
            } else {
                unreachable!()
            }
        })
        .enumerate()
        .filter(|(_, (left, right))| left < right)
        .map(|(i, _)| i + 1)
        .sum()
}

fn part_2_find_decoder_key(packets: &str) -> usize {
    static DIVIDER_PACKETS: Lazy<[PacketData; 2]> = Lazy::new(|| {
        [
            "[[2]]".parse::<PacketData>().unwrap(),
            "[[6]]".parse::<PacketData>().unwrap(),
        ]
    });

    let mut packets = packets
        .lines()
        .filter_map(|l| {
            if l.is_empty() {
                None
            } else {
                l.parse::<PacketData>().ok()
            }
        })
        .chain(DIVIDER_PACKETS.iter().cloned())
        .collect::<Vec<_>>();
    packets.sort();

    packets
        .into_iter()
        .enumerate()
        .filter(|(_i, x)| DIVIDER_PACKETS.contains(x))
        .map(|(i, _x)| i + 1)
        .product()
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

    #[test]
    fn it_compares_correctly() {
        assert_eq!(
            Ordering::Greater,
            "[1,2,3]"
                .parse::<PacketData>()
                .unwrap()
                .cmp(&"[1,2,2]".parse::<PacketData>().unwrap())
        );

        assert_eq!(
            Ordering::Greater,
            "[1,2,3,4]"
                .parse::<PacketData>()
                .unwrap()
                .cmp(&"[1,2,3]".parse::<PacketData>().unwrap())
        );

        assert_eq!(
            Ordering::Greater,
            "[2,2,3]"
                .parse::<PacketData>()
                .unwrap()
                .cmp(&"[1,2,3]".parse::<PacketData>().unwrap())
        );

        assert_eq!(
            Ordering::Greater,
            "2".parse::<PacketData>()
                .unwrap()
                .cmp(&"[1]".parse::<PacketData>().unwrap())
        );
    }

    #[test]
    fn it_works_with_example_1() {
        assert_eq!(13, part_1_sum_correct_indices(TEST_PACKETS));
    }

    #[test]
    fn it_works_with_example_2() {
        assert_eq!(140, part_2_find_decoder_key(TEST_PACKETS));
    }
}
