use std::{
    collections::BTreeMap,
    fmt::{Debug, Display},
    ops::{Add, Sub},
};

/// Abstract coordinate in a two dimensional plane
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn closest(self, other: Coordinate) -> Coordinate {
        let diff = other - self;
        Coordinate {
            x: self.x + diff.x.signum(),
            y: self.y + diff.y.signum(),
        }
    }

    pub fn offset(self, xd: i32, yd: i32) -> Coordinate {
        Coordinate {
            x: self.x + xd,
            y: self.y + yd,
        }
    }

    pub fn with_x(self, x: i32) -> Coordinate {
        Coordinate { x, y: self.y }
    }

    pub fn with_y(self, y: i32) -> Coordinate {
        Coordinate { x: self.x, y }
    }

    pub fn offset_direction(self, direction: Direction, amount: u32) -> Coordinate {
        let amount = amount as i32;
        match direction {
            Direction::Up => self.offset(0, amount),
            Direction::Down => self.offset(0, -amount),
            Direction::Left => self.offset(-amount, 0),
            Direction::Right => self.offset(amount, 0),
        }
    }

    pub fn manhattan_distance(self, other: Coordinate) -> u32 {
        let xd = self.x.abs_diff(other.x);
        let yd = self.y.abs_diff(other.y);

        xd + yd
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(i32, i32)> for Coordinate {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl Sub for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub fn bounds(coords: &[Coordinate]) -> Option<(Coordinate, Coordinate)> {
    let min_x = coords.iter().map(|c| c.x).min()?;
    let min_y = coords.iter().map(|c| c.y).min()?;
    let max_x = coords.iter().map(|c| c.x).max()?;
    let max_y = coords.iter().map(|c| c.y).max()?;

    Some(((min_x, min_y).into(), (max_x, max_y).into()))
}

#[derive(Debug, Clone)]
pub struct Line {
    pub start: Coordinate,
    pub end: Coordinate,
}

impl Line {
    pub fn new(start: Coordinate, end: Coordinate) -> Self {
        Self { start, end }
    }

    pub fn horizontal(y: i32, x_start: i32, x_end: i32) -> Self {
        Self {
            start: Coordinate::new(x_start, y),
            end: Coordinate::new(x_end, y),
        }
    }

    pub fn vertical(x: i32, y_start: i32, y_end: i32) -> Self {
        Self {
            start: Coordinate::new(x, y_start),
            end: Coordinate::new(x, y_end),
        }
    }

    pub fn coords(self) -> LineIter {
        LineIter {
            first: true,
            current: self.start,
            end: self.end,
        }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.start, self.end)
    }
}

#[derive(Debug)]
pub struct LineIter {
    first: bool,
    current: Coordinate,
    end: Coordinate,
}

impl Iterator for LineIter {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            Some(self.current)
        } else if self.current == self.end {
            None
        } else {
            self.current = self.current.closest(self.end);
            Some(self.current)
        }
    }
}

#[derive(Debug)]
pub struct Square {
    min: Coordinate,
    max: Coordinate,
}

impl Square {
    pub fn new(min: Coordinate, max: Coordinate) -> Self {
        Self { min, max }
    }

    pub fn coords(self) -> impl Iterator<Item = Coordinate> {
        let col = Line::new(self.min, self.max.with_x(self.min.x));
        col.coords().into_iter().flat_map(move |y| {
            let row = Line::new(y, y.with_x(self.max.x));
            row.coords()
        })
    }
}

#[derive(Debug)]
pub enum OverflowType {
    None,
    Larger(i32),
    Smaller(i32),
}

#[derive(Debug)]
pub struct OutOfBounds {
    coord: Coordinate,
    x_overflow: OverflowType,
    y_overflow: OverflowType,
}

impl std::error::Error for OutOfBounds {}

impl OutOfBounds {
    pub fn new(coord: Coordinate, x_overflow: OverflowType, y_overflow: OverflowType) -> Self {
        Self {
            coord,
            x_overflow,
            y_overflow,
        }
    }
}

impl Display for OutOfBounds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} out of bounds: ", self.coord)?;
        match self.x_overflow {
            OverflowType::None => {}
            OverflowType::Larger(b) => write!(f, "x={} greater then {}", self.coord.x, b)?,
            OverflowType::Smaller(b) => write!(f, "x={} smaller then {}", self.coord.x, b)?,
        }
        match self.y_overflow {
            OverflowType::None => {}
            OverflowType::Larger(b) => write!(f, "y={} greater then {}", self.coord.y, b)?,
            OverflowType::Smaller(b) => write!(f, "y={} smaller then {}", self.coord.y, b)?,
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Grid<T: Debug> {
    points: BTreeMap<Coordinate, T>,
    start: Coordinate,
    end: Coordinate,
    width: usize,
    height: usize,
    empty: T,
}

impl<T: Debug> Grid<T> {
    pub fn from_coords(start: Coordinate, end: Coordinate, empty: T) -> Grid<T>
    where
        T: Clone,
    {
        let diff = end - start;
        let width = diff.x.abs() as usize + 1;
        let height = diff.y.abs() as usize + 1;
        let mut points = BTreeMap::new();
        points.insert(start, empty.clone());
        points.insert(end, empty.clone());

        Self {
            points,
            width,
            start,
            end,
            height,
            empty,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, coord: Coordinate) -> &T {
        self.points.get(&coord).unwrap_or(&self.empty)
    }

    pub fn set(&mut self, coord: Coordinate, val: T) {
        self.points.insert(coord, val);
    }

    pub fn get_bounded(&self, coord: Coordinate) -> Result<&T, OutOfBounds> {
        self.check_bounds(coord)?;
        Ok(self.get(coord))
    }

    pub fn set_bounded(&mut self, coord: Coordinate, val: T) -> Result<(), OutOfBounds> {
        self.check_bounds(coord)?;
        self.set(coord, val);

        Ok(())
    }

    pub fn get_resize(&mut self, coord: Coordinate) -> &T {
        self.check_and_resize(coord);
        self.get(coord)
    }

    pub fn set_resize(&mut self, coord: Coordinate, val: T) {
        self.check_and_resize(coord);
        self.set(coord, val)
    }

    fn check_bounds(&self, coord: Coordinate) -> Result<(), OutOfBounds> {
        let mut x_overflow = OverflowType::None;
        let mut y_overflow = OverflowType::None;

        if coord.x < self.start.x {
            x_overflow = OverflowType::Smaller(self.start.x);
        } else if coord.x > self.end.x {
            x_overflow = OverflowType::Larger(self.end.x);
        }
        if coord.y < self.start.y {
            y_overflow = OverflowType::Smaller(self.start.y);
        } else if coord.y > self.end.y {
            y_overflow = OverflowType::Larger(self.end.y);
        }

        if let (&OverflowType::None, &OverflowType::None) = (&x_overflow, &y_overflow) {
            Ok(())
        } else {
            Err(OutOfBounds::new(coord, x_overflow, y_overflow))
        }
    }

    fn check_and_resize(&mut self, coord: Coordinate) {
        if let Err(e) = self.check_bounds(coord) {
            match e.x_overflow {
                OverflowType::None => {}
                OverflowType::Larger(_) => self.end.x = coord.x,
                OverflowType::Smaller(_) => self.start.x = coord.x,
            }
            match e.y_overflow {
                OverflowType::None => {}
                OverflowType::Larger(_) => self.end.y = coord.y,
                OverflowType::Smaller(_) => self.start.y = coord.y,
            }
        }
    }

    pub fn coords(&self) -> Vec<Coordinate> {
        (self.start.y..=self.end.y)
            .into_iter()
            .flat_map(|y| {
                (self.start.x..=self.end.x)
                    .into_iter()
                    .map(move |x| Coordinate::new(x, y))
            })
            .collect()
    }

    pub fn coords_at_x(&self, x: i32) -> Result<Vec<Coordinate>, OutOfBounds> {
        let start = self.start.with_x(x);
        let end = self.end.with_x(x);

        self.check_bounds(start)?;
        self.check_bounds(end)?;

        Ok(Line::new(start, end).coords().collect())
    }

    pub fn coords_at_y(&self, y: i32) -> Result<Vec<Coordinate>, OutOfBounds> {
        let start = self.start.with_y(y);
        let end = self.end.with_y(y);

        self.check_bounds(start)?;
        self.check_bounds(end)?;

        Ok(Line::new(start, end).coords().collect())
    }

    pub fn coords_in_area<F>(
        &self,
        coord: Coordinate,
        distance: u32,
        filter: F,
    ) -> impl Iterator<Item = Coordinate>
    where
        F: Fn(Coordinate) -> bool,
    {
        let min = coord.offset(-(distance as i32), distance as i32);
        let max = coord.offset(distance as i32, -(distance as i32));

        println!("Making a square {distance} from {coord}");

        let square = Square::new(min, max);
        println!("Made square");
        square
            .coords()
            .filter(move |sq| filter(*sq) && sq.manhattan_distance(coord) <= distance)
    }

    pub fn display(&self)
    where
        T: Display,
    {
        for (i, point) in self
            .coords()
            .iter()
            .map(|c| self.get_bounded(*c).unwrap())
            .enumerate()
        {
            print!("{}", point);
            if ((i + 1) % self.width) == 0 {
                println!();
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_iter() {
        let line = Line::new((1, 1).into(), (3, 4).into());
        let expected: Vec<Coordinate> =
            vec![(1, 1).into(), (2, 2).into(), (3, 3).into(), (3, 4).into()];

        assert_eq!(line.clone().coords().collect::<Vec<_>>(), expected);
    }
}
