use std::ops::RangeInclusive;

fn segment_range(segment: &str) -> RangeInclusive<u32> {
    let (start, finish) = segment.split_once('-').unwrap();
    let start = start.parse().unwrap();
    let finish = finish.parse().unwrap();

    start..=finish
}

fn parse_line(line: &str) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
    let (a, b) = line.split_once(',').unwrap();

    (segment_range(a), segment_range(b))
}

/// Whether a completely contains b
fn range_contains_completely(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    (a.start() <= b.start() && a.end() >= b.end()) || (b.start() <= a.start() && b.end() >= a.end())
}

fn range_contains_at_all(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    a.contains(&b.start()) || a.contains(&b.end()) || b.contains(&a.start()) || b.contains(&a.end())
}

pub fn common(input: &str, f: fn(&RangeInclusive<u32>, &RangeInclusive<u32>) -> bool) -> u32 {
    input.lines().fold(0, |acc, line| {
        let (a, b) = parse_line(line);
        if f(&a, &b) {
            acc + 1
        } else {
            acc
        }
    })
}
pub fn part_one(input: &str) -> Option<u32> {
    Some(common(input, range_contains_completely))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(common(input, range_contains_at_all))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
