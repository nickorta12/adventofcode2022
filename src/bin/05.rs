#[derive(Debug, Default)]
struct Crates {
    stacks: Vec<Vec<char>>,
}

impl Crates {
    fn parse(input: &str) -> Self {
        let mut stacks: Vec<Vec<char>> = Vec::new();

        for line in input.lines().rev() {
            for (i, c) in line.char_indices() {
                if c == ' ' {
                    continue;
                }
                // This should be a character
                if ((i as i64 - 1) % 4) == 0 {
                    let col_idx = (i - 1) / 4;
                    if let Some(col) = stacks.get_mut(col_idx) {
                        col.push(c);
                    } else {
                        stacks.push(vec![c]);
                    }
                }
            }
        }

        Self { stacks }
    }

    fn move_char(&mut self, num: u32, start: usize, end: usize) {
        let mut num = num;
        while num > 0 {
            let item = self.stacks[start]
                .pop()
                .expect(&format!("No stack with {num}, {start}, {end}: {self:?}"));
            let dest_col = &mut self.stacks[end];

            dest_col.push(item);
            num -= 1;
        }
    }

    fn move_char_many(&mut self, num: u32, start: usize, end: usize) {
        let mut new = Vec::new();
        let mut num = num;
        while num > 0 {
            let item = self.stacks[start]
                .pop()
                .expect(&format!("No stack with {num}, {start}, {end}: {self:?}"));
            new.push(item);
            num -= 1;
        }
        new.reverse();
        let dest_col = &mut self.stacks[end];

        dest_col.append(&mut new);
    }

    fn end(&self) -> String {
        self.stacks.iter().map(|col| col.last().unwrap()).collect()
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let crates_idx = input.find(" 1").unwrap();
    let mut crates = Crates::parse(&input[..crates_idx]);

    input[crates_idx..]
        .lines()
        .skip(2)
        .map(|line| {
            let parts: Vec<_> = line.split_ascii_whitespace().collect();
            (
                parts[1].parse::<u32>().unwrap(),
                parts[3].parse::<usize>().unwrap(),
                parts[5].parse::<usize>().unwrap(),
            )
        })
        .for_each(|(num, start, end)| crates.move_char(num, start - 1, end - 1));

    Some(crates.end())
}

pub fn part_two(input: &str) -> Option<String> {
    let crates_idx = input.find(" 1").unwrap();
    let mut crates = Crates::parse(&input[..crates_idx]);

    input[crates_idx..]
        .lines()
        .skip(2)
        .map(|line| {
            let parts: Vec<_> = line.split_ascii_whitespace().collect();
            (
                parts[1].parse::<u32>().unwrap(),
                parts[3].parse::<usize>().unwrap(),
                parts[5].parse::<usize>().unwrap(),
            )
        })
        .for_each(|(num, start, end)| crates.move_char_many(num, start - 1, end - 1));

    Some(crates.end())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
