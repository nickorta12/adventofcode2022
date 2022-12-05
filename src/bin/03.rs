const START_LOWER: u32 = 'a' as u32;
const START_LOWER_OFFSET: u32 = 1;
const START_UPPER: u32 = 'A' as u32;
const START_UPPER_OFFSET: u32 = 27;

fn priority(c: &char) -> u32 {
    if c.is_lowercase() {
        (*c as u32 - START_LOWER) + START_LOWER_OFFSET
    } else if c.is_uppercase() {
        (*c as u32 - START_UPPER) + START_UPPER_OFFSET
    } else {
        panic!("Not lower or upper")
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let res = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .filter_map(|chars| {
            let (a, b) = chars.split_at(chars.len() / 2);
            for c in a {
                if b.contains(c) {
                    return Some(priority(c));
                }
            }
            None
        })
        .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<_> = input.lines().collect();
    Some(
        lines
            .chunks(3)
            .map(|lines| {
                for c in lines[0].chars() {
                    if lines[1].contains(c) && lines[2].contains(c) {
                        return priority(&c);
                    }
                }
                panic!("No common character")
            })
            .sum(),
    )
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
        assert_eq!(part_two(&input), Some(70));
    }
}
