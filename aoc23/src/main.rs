use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    println!("{}", part_1_count_empty_tiles_after_elf_diffusion(&input));
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn next(&self) -> Self {
        match *self {
            Direction::North => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::East,
            Direction::East => Direction::North,
        }
    }
}

fn part_1_count_empty_tiles_after_elf_diffusion(state: &str) -> usize {
    let width = state.lines().next().unwrap().len();
    let elves = state
        .chars()
        .filter(|&c| !c.is_whitespace())
        .enumerate()
        .filter(|&(_i, x)| x == '#')
        .map(|(i, _x)| Position {
            x: (i % width) as isize,
            y: (i / width) as isize,
        })
        .collect::<Vec<_>>();

    let (final_elves, _) = (0..10).fold((elves, Direction::North), |(elves, dir), _el| {
        // Step 1 - propose new positions
        let mut proposed_moves = HashMap::new();
        for elf in elves.clone() {
            let north = Position {
                x: elf.x,
                y: elf.y - 1,
            };
            let north_east = Position {
                x: north.x + 1,
                y: north.y,
            };
            let north_west = Position {
                x: north.x - 1,
                y: north.y,
            };
            let south = Position {
                x: elf.x,
                y: elf.y + 1,
            };
            let south_east = Position {
                x: south.x + 1,
                y: south.y,
            };
            let south_west = Position {
                x: south.x - 1,
                y: south.y,
            };
            let west = Position {
                x: elf.x - 1,
                y: elf.y,
            };
            let east = Position {
                x: elf.x + 1,
                y: elf.y,
            };

            for dir in [dir, dir.next(), dir.next().next(), dir.next().next().next()] {
                if !elves.iter().any(|&e| {
                    e == north
                        || e == north_east
                        || e == north_west
                        || e == south
                        || e == south_east
                        || e == south_west
                        || e == west
                        || e == east
                }) {
                    // Nothing to do. Stay still!
                    break;
                }

                match dir {
                    Direction::North => {
                        if !elves
                            .iter()
                            .any(|&e| e == north || e == north_east || e == north_west)
                        {
                            proposed_moves.insert(elf, north);
                            break;
                        }
                    }
                    Direction::South => {
                        if !elves
                            .iter()
                            .any(|&e| e == south || e == south_west || e == south_east)
                        {
                            proposed_moves.insert(elf, south);
                            break;
                        }
                    }
                    Direction::West => {
                        if !elves
                            .iter()
                            .any(|&e| e == west || e == north_west || e == south_west)
                        {
                            proposed_moves.insert(elf, west);
                            break;
                        }
                    }
                    Direction::East => {
                        if !elves
                            .iter()
                            .any(|&e| e == east || e == north_east || e == south_east)
                        {
                            proposed_moves.insert(elf, east);
                            break;
                        }
                    }
                }
            }
        }

        // Step 2 - move to proposed positions
        let moved = elves
            .iter()
            .map(|e| {
                if let Some(move_to) = proposed_moves.get(e) {
                    if proposed_moves
                        .iter()
                        .filter(|&(_from, to)| to == move_to)
                        .count()
                        == 1
                    {
                        *move_to
                    } else {
                        *e
                    }
                } else {
                    *e
                }
            })
            .collect::<Vec<_>>();

        (moved, dir.next())
    });

    // Find empty spaces
    print_elves(&final_elves);
    let final_elves_map = final_elves.iter().copied().collect::<HashSet<_>>();
    let mut empty_ground_count = 0;
    for y in final_elves.iter().min_by_key(|&e| e.y).unwrap().y
        ..=final_elves.iter().max_by_key(|&e| e.y).unwrap().y
    {
        for x in final_elves.iter().min_by_key(|&e| e.x).unwrap().x
            ..=final_elves.iter().max_by_key(|&e| e.x).unwrap().x
        {
            if !final_elves_map.contains(&Position { x, y }) {
                empty_ground_count += 1;
            }
        }
        println!();
    }

    empty_ground_count
}

fn print_elves(elves: &[Position]) {
    println!();
    let map = elves.iter().copied().collect::<HashSet<_>>();
    for y in
        elves.iter().min_by_key(|&e| e.y).unwrap().y..=elves.iter().max_by_key(|&e| e.y).unwrap().y
    {
        for x in elves.iter().min_by_key(|&e| e.x).unwrap().x
            ..=elves.iter().max_by_key(|&e| e.x).unwrap().x
        {
            if map.contains(&Position { x, y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALLER_TEST_ELVES: &'static str = ".....
..##.
..#..
.....
..##.
.....";

    const TEST_ELVES: &'static str = "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............";

    #[test]
    fn it_works_position() {
        let _ = TEST_ELVES
            .chars()
            .filter(|&c| !c.is_whitespace())
            .enumerate()
            .filter(|&(_i, x)| x == '#')
            .map(|(i, _x)| Position {
                x: (i % TEST_ELVES.lines().next().unwrap().len()) as isize,
                y: (i / TEST_ELVES.lines().next().unwrap().len()) as isize,
            })
            .collect::<Vec<_>>();
    }

    #[test]
    fn it_works_with_example_1() {
        assert_eq!(
            110,
            part_1_count_empty_tiles_after_elf_diffusion(TEST_ELVES)
        )
    }
}
