use std::cmp::Ordering;

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
    // let elf_tallies = input
    //     .split_terminator("\n\n")
    //     .map(|list| {
    //         list.lines()
    //             .filter_map(|s| -> Option<u32> { s.parse().ok() })
    //             .sum()
    //     })
    //     // .collect::<Vec<u32>>();
    //     .into_iter();

    // let max_position = elf_tallies
    //     .enumerate()
    //     .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
    //     .map(|(index, _)| index);

    // if let Some(position) = max_position {
    //     //
    // }

    // let mut first = 0;
    // let mut second = 0;
    // let mut third = 0;

    // for tally in elf_tallies {
    //     dbg!(tally);
    //     if tally >= first {
    //         first = tally;
    //     } else if tally >= second {
    //         second = tally;
    //     } else if tally >= third {
    //         third = tally;
    //     }
    // }

    // println!("First: {first}, Second: {second}, Third: {third}");

    // Some(first + second + third)

    None
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
        assert_eq!(part_two(&input), None);
        // assert_eq!(part_two(&input), Some(45_000));
    }
}
