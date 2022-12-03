pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;

    for rucksack in input.lines() {
        let (first_half, second_half) = rucksack.split_at(rucksack.len() / 2);
        if let Some(item) = find_common_item(first_half, second_half) {
            let priority = get_priority(item);
            total += priority;
        }
    }

    Some(total)
}

fn find_common_item(first_half: &str, second_half: &str) -> Option<char> {
    first_half.chars().find(|c| second_half.contains(*c))
}

fn get_priority(item: char) -> u32 {
    let code = item as u32;
    match item {
        'a'..='z' => code - 96,
        'A'..='Z' => code - 38,
        _ => panic!(),
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
