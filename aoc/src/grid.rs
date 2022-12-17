use std::{
    fmt::{Debug, Display},
    ops::{Add, Sub},
};

/// Abstract coordinate in a two dimensional plane
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
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

#[derive(Debug)]
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

    pub fn coords(&self) -> Vec<Coordinate> {
        let mut coords = Vec::new();
        let mut coord = self.start.clone();

        while coord != self.end {
            coords.push(coord.clone());
            coord = coord.closest(self.end);
        }
        coords.push(coord);

        coords
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.start, self.end)
    }
}

#[derive(Debug)]
pub struct OutOfBounds {
    coord: Coordinate,
    min: Coordinate,
    max: Coordinate,
}

impl std::error::Error for OutOfBounds {}

impl OutOfBounds {
    pub fn new(coord: Coordinate, min: Coordinate, max: Coordinate) -> Self {
        Self { coord, min, max }
    }
}

impl Display for OutOfBounds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Out of bounds: {}. Min: {}, Max: {}",
            self.coord, self.min, self.max
        )
    }
}

#[derive(Debug)]
pub struct Grid<T: Debug> {
    points: Vec<T>,
    start: Coordinate,
    end: Coordinate,
    width: usize,
    height: usize,
}

impl<T: Debug> Grid<T> {
    pub fn from_coords(start: Coordinate, end: Coordinate, val: T) -> Grid<T>
    where
        T: Clone,
    {
        let diff = end - start;
        let width = diff.x.abs() as usize + 1;
        let height = diff.y.abs() as usize + 1;
        let points = vec![val; width * height];

        Self {
            points,
            width,
            start,
            end,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, coord: Coordinate) -> Result<&T, OutOfBounds> {
        let index = self.index(coord)?;
        Ok(&self.points[index])
    }

    pub fn set(&mut self, coord: Coordinate, val: T) -> Result<(), OutOfBounds> {
        let index = self.index(coord)?;
        self.points[index] = val;

        Ok(())
    }

    fn index(&self, coord: Coordinate) -> Result<usize, OutOfBounds> {
        if coord.x < self.start.x
            || coord.y < self.start.y
            || coord.x > self.end.x
            || coord.y > self.end.y
        {
            return Err(OutOfBounds::new(coord, self.start, self.end));
        }

        let diff = coord - self.start;
        Ok(diff.y as usize * self.width + diff.x as usize)
    }

    pub fn display(&self)
    where
        T: Display,
    {
        for (i, point) in self.points.iter().enumerate() {
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
