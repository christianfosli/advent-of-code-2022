use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("input.txt")?;
    let calories_by_elf = input.split("\n\n").map(|elf| {
        elf.lines()
            .filter_map(|cal| cal.parse::<u64>().ok())
            .sum::<u64>()
    });

    println!(
        "Part 1 - elf with most calories: {:?}",
        calories_by_elf.clone().max()
    );

    let mut calories_by_elf = calories_by_elf.collect::<Vec<_>>();
    calories_by_elf.sort_unstable();
    println!(
        "Part 2 answer - sum of top 3 elve's calories: {:?}",
        calories_by_elf.iter().rev().take(3).sum::<u64>()
    );

    Ok(())
}
