use std::collections::HashSet;
use std::hash::Hash;

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn distinct(input: &str, num: usize) -> Option<u32> {
    let x: Vec<_> = input.char_indices().collect();
    x.windows(num).find_map(|x| {
        if has_unique_elements(x.iter().map(|(_, c)| c)) {
            x.last().map(|(u, _)| *u as u32 + 1)
        } else {
            None
        }
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    distinct(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    distinct(input, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        let mut lines = input.lines();
        assert_eq!(part_one(lines.next().unwrap()), Some(7));
        assert_eq!(part_one(lines.next().unwrap()), Some(5));
        assert_eq!(part_one(lines.next().unwrap()), Some(6));
        assert_eq!(part_one(lines.next().unwrap()), Some(10));
        assert_eq!(part_one(lines.next().unwrap()), Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        let mut lines = input.lines();
        assert_eq!(part_two(lines.next().unwrap()), Some(19));
        assert_eq!(part_two(lines.next().unwrap()), Some(23));
        assert_eq!(part_two(lines.next().unwrap()), Some(23));
        assert_eq!(part_two(lines.next().unwrap()), Some(29));
        assert_eq!(part_two(lines.next().unwrap()), Some(26));
    }
}
