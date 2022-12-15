use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

#[derive(Debug)]
struct Grid {
    points: Vec<u32>,
    width: usize,
    height: usize,
    start: Coordinate,
    end: Coordinate,
}

fn parse_grid(input: &str) -> Grid {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let chars: Vec<_> = input.lines().map(|l| l.chars()).flatten().collect();
    let start = Coordinate::from_index(
        chars
            .iter()
            .enumerate()
            .find_map(|(i, &c)| if c == 'S' { Some(i) } else { None })
            .unwrap(),
        width,
    );
    let end = Coordinate::from_index(
        chars
            .iter()
            .enumerate()
            .find_map(|(i, &c)| if c == 'E' { Some(i) } else { None })
            .unwrap(),
        width,
    );

    let points = chars
        .into_iter()
        .map(|c| {
            let c = match c {
                'S' => 'a',
                'E' => 'z',
                _ => c,
            };

            c as u32 - 'a' as u32
        })
        .collect();

    Grid {
        points,
        width,
        height,
        start,
        end,
    }
}

impl Grid {
    fn coord(&self, index: usize) -> Coordinate {
        Coordinate::from_index(index, self.width)
    }

    fn coord_in_direction(&self, coord: Coordinate, dir: Direction) -> Option<Coordinate> {
        match dir {
            Direction::Up => coord.move_y(-1),
            Direction::Down => coord.move_y(1).filter(|c| c.y < self.height),
            Direction::Left => coord.move_x(-1),
            Direction::Right => coord.move_x(1).filter(|c| c.x < self.width),
        }
    }

    /// Height of point
    /// b - a
    fn point_delta(&self, a: Coordinate, b: Coordinate) -> i32 {
        self.point(b) as i32 - self.point(a) as i32
    }

    fn point(&self, coord: Coordinate) -> u32 {
        self.points[coord.index()]
    }

    fn neighbors(&self, coord: Coordinate) -> Vec<Coordinate> {
        DIRECTIONS
            .iter()
            .filter_map(|dir| self.coord_in_direction(coord, *dir))
            .filter(|new_coord| self.point_delta(*new_coord, coord) <= 1)
            .collect()
    }

    fn find_shortest_path(&self, starts: &[Coordinate]) -> Option<u32> {
        let mut visited = vec![false; self.len()];
        let mut distances = vec![f32::INFINITY; self.len()];
        let mut current = self.end;

        distances[current.index()] = 0.0;

        loop {
            for nei in self.neighbors(current) {
                let nei_distance = distances[nei.index()];
                let current_distance = distances[current.index()];

                if !visited[nei.index()] && nei_distance > current_distance + 1.0 {
                    distances[nei.index()] = current_distance + 1.0;
                }
            }
            visited[current.index()] = true;

            if let Some(next) = distances
                .iter()
                .enumerate()
                .filter(|(index, _)| !visited[*index])
                .min_by(|(_, a), (_, b)| a.total_cmp(b))
                .map(|(index, _)| self.coord(index))
            {
                current = next;
            } else {
                break;
            }
        }

        starts.iter().map(|s| distances[s.index()] as u32).min()
    }
    fn len(&self) -> usize {
        self.points.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
    width: usize,
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Coordinate {
    fn from_index(index: usize, width: usize) -> Self {
        Self {
            x: index % width,
            y: index / width,
            width,
        }
    }

    fn index(self) -> usize {
        self.y * self.width + self.x
    }

    fn move_x(self, x: i32) -> Option<Self> {
        Self::checked_add(self.x, x).map(|x| Self {
            x,
            y: self.y,
            width: self.width,
        })
    }

    fn move_y(self, y: i32) -> Option<Self> {
        Self::checked_add(self.y, y).map(|y| Self {
            x: self.x,
            y,
            width: self.width,
        })
    }

    fn checked_add(a: usize, b: i32) -> Option<usize> {
        let c = a as i32 + b;
        if c < 0 {
            None
        } else {
            Some(c as usize)
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    grid.find_shortest_path(&[grid.start])
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    let starts: Vec<_> = grid
        .points
        .iter()
        .enumerate()
        .filter_map(|(i, c)| if *c == 0 { Some(grid.coord(i)) } else { None })
        .collect();

    grid.find_shortest_path(&starts)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
