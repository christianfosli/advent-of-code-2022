fn main() {
    println!("Hello, world!");
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
    let height = state.lines().count();
    let (final_state, _) = (0..10).fold((state, Direction::North), |(state, dir), _el| {
        // Step 1 - propose new positions
        let elves = state
            .chars()
            .enumerate()
            .filter(|&(_i, x)| x == '#')
            .map(|(i, _x)| {
                for dir in [dir, dir.next(), dir.next().next(), dir.next().next().next()] {
                    if let Some(next) = match dir {
                        Direction::North => {
                            if i >= width {
                                Some(i - width)
                            } else {
                                None
                            }
                        }
                        Direction::South => {
                            if i * width >= width * height - width {
                                Some(i + width)
                            } else {
                                None
                            }
                        }
                        Direction::West => {
                            if i % width != 0 {
                                Some(i - 1)
                            } else {
                                None
                            }
                        }
                        Direction::East => {
                            if i % width + 1 != width {
                                Some(i + 1)
                            } else {
                                None
                            }
                        }
                    } {
                        return (i, next);
                    }
                }
                return (i, i);
            });
        // Step 2 - move
        // TODO: Move and adjust state accordingly
        (state, dir.next())
    });

    final_state.chars().filter(|&x| x == '.').count()
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn it_works_with_example_1() {
        assert_eq!(
            110,
            part_1_count_empty_tiles_after_elf_diffusion(TEST_ELVES)
        )
    }
}
