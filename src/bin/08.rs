fn parse_grid(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn transpose(v: &[Vec<u8>], len: usize) -> Vec<Vec<u8>> {
    (0..len)
        .map(|i| v.iter().map(|inner| inner[i]).collect::<Vec<u8>>())
        .collect()
}

fn is_edge(x: usize, y: usize, edge: usize) -> bool {
    x == 0 || x == edge || y == 0 || y == edge
}

fn max(rem: &[u8]) -> u8 {
    *rem.iter().max().unwrap()
}

fn viewing_distance<'a, T: Iterator<Item = &'a u8>>(tree: u8, rem: T) -> u32 {
    let mut d = 0;
    for t in rem {
        d += 1;
        if t >= &tree {
            break;
        }
    }

    d
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    let len = grid.len();
    let edge = len - 1;
    let inverted = transpose(&grid, len);

    let mut num = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, tree) in row.iter().enumerate() {
            let tree = *tree;
            if is_edge(x, y, edge) {
                num += 1;
                continue;
            }

            let left = max(&row[..x]);
            let right = max(&row[(x + 1)..]);

            let col = &inverted[x];
            let up = max(&col[(y + 1)..]);
            let down = max(&col[..y]);

            if tree > right || tree > left || tree > up || tree > down {
                num += 1;
            }
        }
    }

    Some(num)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    let len = grid.len();
    let edge = len - 1;
    let inverted = transpose(&grid, len);

    let mut max = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, tree) in row.iter().enumerate() {
            let tree = *tree;
            if is_edge(x, y, edge) {
                continue;
            }

            let left = viewing_distance(tree, row[..x].iter().rev());
            let right = viewing_distance(tree, row[(x + 1)..].iter());

            let col = &inverted[x];
            let up = viewing_distance(tree, col[..y].iter().rev());
            let down = viewing_distance(tree, col[(y + 1)..].iter());

            let distance = left * right * up * down;
            assert!(distance != 0);
            if distance > max {
                println!("{distance} || {x},{y}: {tree}. {left}-{right}  {up}-{down}");
                max = distance;
            }
        }
    }

    Some(max)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
