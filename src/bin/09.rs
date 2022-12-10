use nom::{
    bytes::complete::tag, character::complete::one_of, combinator::map, sequence::separated_pair,
    IResult,
};
use std::{
    collections::HashSet,
    fmt::Display,
    ops::{Add, Sub},
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn move_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
        }
    }

    fn move_closest(&mut self, other: Coord) {
        let diff = other - *self;
        self.x += diff.x.signum();
        self.y += diff.y.signum();
    }

    fn abs_diff(self, other: Coord) -> u32 {
        let coord_diff = (self - other).abs();
        let edge = coord_diff.x.min(coord_diff.y) as u32;
        let non_diag_diff = coord_diff.x.abs_diff(coord_diff.y);

        edge + non_diag_diff
    }

    fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl From<(i32, i32)> for Coord {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn parse_direction(i: &str) -> IResult<&str, Direction> {
    map(one_of("LRUD"), |char| match char {
        'L' => Direction::Left,
        'R' => Direction::Right,
        'U' => Direction::Up,
        'D' => Direction::Down,
        _ => unreachable!(),
    })(i)
}

#[derive(Debug, Clone, Copy)]
struct Delta {
    direction: Direction,
    num: u32,
}

fn parse_move(i: &str) -> IResult<&str, Delta> {
    map(
        separated_pair(parse_direction, tag(" "), nom::character::complete::u32),
        |(direction, num)| Delta { direction, num },
    )(i)
}

#[derive(Debug)]
struct Grid {
    snake: Vec<Coord>,
    visited: HashSet<Coord>,
}

impl Grid {
    fn new(len: usize) -> Self {
        let mut visited = HashSet::new();
        visited.insert((0, 0).into());

        let snake = vec![(0, 0).into(); len + 2];
        Self { snake, visited }
    }

    fn process_delta(&mut self, delta: Delta) {
        for _ in 0..delta.num {
            self.snake
                .first_mut()
                .unwrap()
                .move_direction(delta.direction);

            let mut prev = self.snake[0].clone();

            for coord in self.snake[1..].iter_mut() {
                if prev.abs_diff(*coord) > 1 {
                    coord.move_closest(prev);
                }
                prev = coord.clone();
            }

            self.visited.insert(*self.snake.last().unwrap());
        }
    }
}

fn snek(input: &str, size: usize) -> u32 {
    let mut grid: Grid = Grid::new(size);
    for delta in input.lines().map(|line| parse_move(line).unwrap().1) {
        grid.process_delta(delta);
    }

    grid.visited.len() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(snek(input, 0))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(snek(input, 8))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coord_diff() {
        assert_eq!(Coord::from((1, 1)).abs_diff(Coord::from((3, 3))), 2);
        assert_eq!(Coord::from((3, 3)).abs_diff(Coord::from((1, 1))), 2);
        assert_eq!(Coord::from((1, 1)).abs_diff(Coord::from((1, 1))), 0);
        assert_eq!(Coord::from((-1, -1)).abs_diff(Coord::from((1, 1))), 2);
        assert_eq!(Coord::from((1, 1)).abs_diff(Coord::from((3, 4))), 3);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }

    #[test]
    fn test_part_two_larger() {
        let input = "R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20"
            .replace("        ", "");
        assert_eq!(part_two(&input), Some(36));
    }
}
