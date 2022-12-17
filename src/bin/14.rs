use std::fmt::Display;

use aoc::grid::{bounds, Coordinate, Grid, Line, OutOfBounds};

mod parser {
    use aoc::grid::Coordinate;
    use nom::{
        bytes::complete::tag, combinator::map, multi::separated_list1, sequence::separated_pair,
        IResult,
    };

    fn coord(i: &str) -> IResult<&str, Coordinate> {
        use nom::character::complete::i32;
        map(separated_pair(i32, tag(","), i32), Coordinate::from)(i)
    }

    pub fn coords(i: &str) -> anyhow::Result<Vec<Coordinate>> {
        let (_, res) = separated_list1(tag(" -> "), coord)(i).map_err(|e| e.to_owned())?;

        Ok(res)
    }
}

fn lines(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|l| {
            parser::coords(l)
                .unwrap()
                .windows(2)
                .map(|w| Line::new(w[0].clone(), w[1].clone()))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

#[derive(Debug, Clone)]
enum Tile {
    Empty,
    Wall,
    Sand,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Wall => write!(f, "#"),
            Tile::Sand => write!(f, "o"),
        }
    }
}

trait SandyCoord {
    fn diag_left(self) -> Self;
    fn diag_right(self) -> Self;
    fn down(self) -> Self;
}

impl SandyCoord for Coordinate {
    fn diag_left(self) -> Self {
        self.offset(-1, 1)
    }

    fn diag_right(self) -> Self {
        self.offset(1, 1)
    }

    fn down(self) -> Self {
        self.offset(0, 1)
    }
}

trait SandyGrid {
    fn move_sand(&self, coord: &mut Coordinate) -> Result<bool, OutOfBounds>;
}

impl SandyGrid for Grid<Tile> {
    fn move_sand(&self, coord: &mut Coordinate) -> Result<bool, OutOfBounds> {
        if let Tile::Empty = self.get(coord.down())? {
            *coord = coord.down();
            Ok(true)
        } else if let Tile::Empty = self.get(coord.diag_left())? {
            *coord = coord.diag_left();
            Ok(true)
        } else if let Tile::Empty = self.get(coord.diag_right())? {
            *coord = coord.diag_right();
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

const START: Coordinate = Coordinate::new(500, 0);

pub enum SandError {
    FellOffEdge,
    NowhereToGo,
}
impl From<OutOfBounds> for SandError {
    fn from(_: OutOfBounds) -> Self {
        Self::FellOffEdge
    }
}

fn drop_sand(grid: &mut Grid<Tile>) -> Result<(), SandError> {
    let mut coord: Coordinate = START;
    while grid.move_sand(&mut coord)? {}
    if coord == START {
        return Err(SandError::NowhereToGo);
    }
    grid.set(coord, Tile::Sand).unwrap();

    Ok(())
}

pub fn part_one(input: &str) -> Option<u32> {
    let coords: Vec<_> = lines(input).iter().flat_map(|line| line.coords()).collect();
    let (min, max) = {
        let (mut min, mut max) = bounds(&coords).unwrap();
        min.x -= 1;
        min.y = 0;
        max.x += 1;
        max.y += 1;

        (min, max)
    };

    let mut grid = Grid::from_coords(min, max, Tile::Empty);
    for coord in coords {
        grid.set(coord, Tile::Wall).unwrap();
    }
    let mut count = 0;
    while let Ok(()) = drop_sand(&mut grid) {
        count += 1;
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let coords: Vec<_> = lines(input).iter().flat_map(|line| line.coords()).collect();
    let (min, max) = {
        let (mut min, mut max) = bounds(&coords).unwrap();
        min.x -= 1000;
        min.y = 0;
        max.x += 1000;
        max.y += 2;

        (min, max)
    };

    let mut grid = Grid::from_coords(min, max, Tile::Empty);
    for coord in coords {
        grid.set(coord, Tile::Wall).unwrap();
    }
    for coord in Line::horizontal(max.y, min.x, max.x).coords() {
        grid.set(coord, Tile::Wall).unwrap();
    }
    let mut count = 0;
    while let Ok(()) = drop_sand(&mut grid) {
        count += 1;
    }

    Some(count + 1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
