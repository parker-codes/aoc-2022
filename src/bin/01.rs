pub fn part_one(input: &str) -> Option<u32> {
    input
        .split_terminator("\n\n")
        .map(|list| {
            list.lines()
                .filter_map(|s| -> Option<u32> { s.parse().ok() })
                .sum()
        })
        .max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let elf_tallies = input
        .split_terminator("\n\n")
        .map(|list| {
            list.lines()
                .filter_map(|s| -> Option<u32> { s.parse().ok() })
                .sum()
        })
        .into_iter();

    // let first: u32 = elf_tallies.clone().max().take().unwrap();
    // let second = elf_tallies.clone().max().take().unwrap();
    // let third = elf_tallies.max().take().unwrap();

    // println!("First: {first}, Second: {second}, Third: {third}");

    // Some(first + second + third)

    Some(1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24_000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45_000));
    }
}
