use std::fmt::Display;

use aoc::grid::{bounds, Coordinate, Direction, Grid, Line};

mod parser {
    use aoc::grid::Coordinate;
    use nom::{
        bytes::complete::tag,
        combinator::map,
        error::VerboseError,
        sequence::{preceded, separated_pair},
    };

    type IResult<'a, O> = nom::IResult<&'a str, O, VerboseError<&'a str>>;

    fn coord(i: &str) -> IResult<Coordinate> {
        use nom::character::complete::i32;
        map(
            separated_pair(
                preceded(tag("x="), i32),
                tag(", "),
                preceded(tag("y="), i32),
            ),
            |(x, y)| Coordinate::new(x, y),
        )(i)
    }

    fn sensor(i: &str) -> IResult<Coordinate> {
        preceded(tag("Sensor at "), coord)(i)
    }

    fn beacon(i: &str) -> IResult<Coordinate> {
        preceded(tag("closest beacon is at "), coord)(i)
    }

    pub fn sensor_beacon(
        i: &str,
    ) -> Result<(Coordinate, Coordinate), nom::Err<VerboseError<&str>>> {
        let (_, (sensor, beacon)) = separated_pair(sensor, tag(": "), beacon)(i)?;

        Ok((sensor, beacon))
    }
}

fn sensors_beacons(input: &str) -> Vec<(Coordinate, Coordinate)> {
    input
        .lines()
        .map(|line| parser::sensor_beacon(line).unwrap())
        .collect()
}

#[derive(Debug, Clone, Copy)]
enum Point {
    Sensor,
    Beacon,
    Blocked,
    Empty,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Point::Sensor => write!(f, "S"),
            Point::Beacon => write!(f, "B"),
            Point::Blocked => write!(f, "#"),
            Point::Empty => write!(f, "."),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let sb = sensors_beacons(input);
    let coords: Vec<_> = sb.iter().cloned().flat_map(|(a, b)| vec![a, b]).collect();
    let (start, end) = {
        let (mut start, mut end) = bounds(&*coords).unwrap();
        start.x -= 10;
        start.y -= 10;
        end.x += 10;
        end.y += 10;

        (start, end)
    };
    println!("Need to make grid: {} - {}", start, end);
    let mut grid = Grid::from_coords(start, end, Point::Empty);
    println!("Made grid");

    const Y: i32 = 10;

    for (s, b) in sb {
        grid.set_bounded(s, Point::Sensor).unwrap();
        grid.set_bounded(b, Point::Beacon).unwrap();

        let diff = s.manhattan_distance(b);
        if !((s.y - diff as i32)..=(s.y + diff as i32)).contains(&Y) {
            println!("Skipping {s}, {b} cause not in range of 10");
            continue;
        }
        println!("Checking {s} - {b} with distance: {}", diff);

        let target_coord = s.with_y(Y);
        let y_diff = s.y.abs_diff(target_coord.y);
        let remaining = diff - y_diff;
        for coord in Line::new(
            target_coord.offset_direction(Direction::Left, remaining),
            target_coord.offset_direction(Direction::Right, remaining),
        )
        .coords()
        {
            if let Point::Empty = grid.get(coord) {
                grid.set_resize(coord, Point::Blocked);
            }
        }
    }

    Some(
        grid.coords_at_y(Y)
            .unwrap()
            .into_iter()
            .filter(|c| matches!(grid.get(*c), Point::Blocked))
            .count() as u32,
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), None);
    }
}
