pub fn part_one(input: &str) -> Option<u32> {
    let mut max = 0;
    let mut current_sum = 0;
    for line in input.lines() {
        if let Ok(current) = line.parse::<u32>() {
            current_sum += current;
        } else {
            if current_sum > max {
                max = current_sum;
            }
            current_sum = 0;
        }
    }

    if current_sum > max {
        max = current_sum;
    }

    Some(max)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut top_three = [0, 0, 0];
    let mut current_sum = 0;

    for line in input.lines() {
        if let Ok(current) = line.parse::<u32>() {
            current_sum += current;
        } else {
            let min = top_three.iter().min().unwrap();
            if current_sum > *min {
                top_three[top_three.iter().position(|x| x == min).unwrap()] = current_sum;
            }
            current_sum = 0;
        }
    }
    let min = top_three.iter().min().unwrap();
    if current_sum > *min {
        top_three[top_three.iter().position(|x| x == min).unwrap()] = current_sum;
    }

    Some(top_three.iter().sum())
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
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
