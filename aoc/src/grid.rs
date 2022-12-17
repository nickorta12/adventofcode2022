use std::{
    fmt::{Debug, Display},
    ops::{Add, Sub},
};

pub trait Integer:
    num::Integer + num::Signed + num::ToPrimitive + Clone + Copy + Display + Debug
{
}
impl Integer for i32 {}
impl Integer for i64 {}

/// Abstract coordinate in a two dimensional plane
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Coordinate<I: Integer> {
    pub x: I,
    pub y: I,
}

impl<I: Integer> Coordinate<I> {
    pub const fn new(x: I, y: I) -> Self {
        Self { x, y }
    }

    pub fn closest(self, other: Coordinate<I>) -> Coordinate<I> {
        let diff = other - self;
        Coordinate {
            x: self.x + diff.x.signum(),
            y: self.y + diff.y.signum(),
        }
    }

    pub fn offset(self, xd: I, yd: I) -> Coordinate<I> {
        Coordinate {
            x: self.x + xd,
            y: self.y + yd,
        }
    }
}

impl<I: Integer> Display for Coordinate<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<I: Integer> From<(I, I)> for Coordinate<I> {
    fn from((x, y): (I, I)) -> Self {
        Self { x, y }
    }
}

impl<I: Integer> Sub for Coordinate<I> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<I: Integer> Add for Coordinate<I> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub fn bounds<I: Integer>(coords: &[Coordinate<I>]) -> Option<(Coordinate<I>, Coordinate<I>)> {
    let min_x = coords.iter().map(|c| c.x).min()?;
    let min_y = coords.iter().map(|c| c.y).min()?;
    let max_x = coords.iter().map(|c| c.x).max()?;
    let max_y = coords.iter().map(|c| c.y).max()?;

    Some(((min_x, min_y).into(), (max_x, max_y).into()))
}

#[derive(Debug)]
pub struct Line<I: Integer> {
    pub start: Coordinate<I>,
    pub end: Coordinate<I>,
}

impl<I: Integer> Line<I> {
    pub fn new(start: Coordinate<I>, end: Coordinate<I>) -> Self {
        Self { start, end }
    }

    pub fn horizontal(y: I, x_start: I, x_end: I) -> Self {
        Self {
            start: Coordinate::new(x_start, y),
            end: Coordinate::new(x_end, y),
        }
    }

    pub fn vertical(x: I, y_start: I, y_end: I) -> Self {
        Self {
            start: Coordinate::new(x, y_start),
            end: Coordinate::new(x, y_end),
        }
    }

    pub fn coords(&self) -> Vec<Coordinate<I>> {
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

impl<I: Integer + Display> Display for Line<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.start, self.end)
    }
}

#[derive(Debug)]
pub struct OutOfBounds<I: Integer> {
    coord: Coordinate<I>,
    min: Coordinate<I>,
    max: Coordinate<I>,
}

impl<I: Integer> std::error::Error for OutOfBounds<I> {}

impl<I: Integer> OutOfBounds<I> {
    pub fn new(coord: Coordinate<I>, min: Coordinate<I>, max: Coordinate<I>) -> Self {
        Self { coord, min, max }
    }
}

impl<I: Integer> Display for OutOfBounds<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Out of bounds: {}. Min: {}, Max: {}",
            self.coord, self.min, self.max
        )
    }
}

#[derive(Debug)]
pub struct Grid<I: Integer, T: Debug> {
    points: Vec<T>,
    start: Coordinate<I>,
    end: Coordinate<I>,
    width: usize,
    height: usize,
}

impl<I: Integer, T: Debug> Grid<I, T> {
    pub fn from_coords(start: Coordinate<I>, end: Coordinate<I>, val: T) -> Grid<I, T>
    where
        T: Clone,
    {
        let diff = end - start;
        let width = diff.x.abs().to_usize().unwrap() + 1;
        let height = diff.y.abs().to_usize().unwrap() + 1;
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

    pub fn get(&self, coord: Coordinate<I>) -> Result<&T, OutOfBounds<I>> {
        let index = self.index(coord)?;
        Ok(&self.points[index])
    }

    pub fn set(&mut self, coord: Coordinate<I>, val: T) -> Result<(), OutOfBounds<I>> {
        let index = self.index(coord)?;
        self.points[index] = val;

        Ok(())
    }

    fn index(&self, coord: Coordinate<I>) -> Result<usize, OutOfBounds<I>> {
        if coord.x < self.start.x
            || coord.y < self.start.y
            || coord.x > self.end.x
            || coord.y > self.end.y
        {
            return Err(OutOfBounds::new(coord, self.start, self.end));
        }

        let diff = coord - self.start;
        Ok(diff.y.to_usize().unwrap() * self.width + diff.x.to_usize().unwrap())
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
