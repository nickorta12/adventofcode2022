use std::{fmt::Display, num::ParseIntError};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res, rest},
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
enum Cmd {
    Cd(String),
    Ls,
}

impl Cmd {
    fn parse(i: &str) -> IResult<&str, Self> {
        let cd = map(preceded(tag("cd "), rest), |x: &str| {
            Self::Cd(x.to_string())
        });
        let ls = map(tag("ls"), |_| Self::Ls);
        preceded(tag("$ "), alt((cd, ls)))(i)
    }
}

#[derive(Debug)]
enum DirContents {
    Dir(String),
    File(u64),
}

impl DirContents {
    fn parse(i: &str) -> IResult<&str, Self> {
        let dir = map(preceded(tag("dir "), rest), |x: &str| {
            Self::Dir(x.to_string())
        });
        let file = map_res(
            separated_pair(digit1, tag(" "), rest),
            |(size, _name): (&str, &str)| -> Result<DirContents, ParseIntError> {
                let size = size.parse::<u64>()?;
                Ok(Self::File(size))
            },
        );

        alt((dir, file))(i)
    }
}

#[derive(Debug)]
enum ParsedLine {
    Cmd(Cmd),
    DirContents(DirContents),
}

fn parse_line(line: &str) -> ParsedLine {
    if let Ok((_line, cmd)) = Cmd::parse(line) {
        ParsedLine::Cmd(cmd)
    } else if let Ok((_line, dir)) = DirContents::parse(line) {
        ParsedLine::DirContents(dir)
    } else {
        panic!("Unable to parse line: {line}");
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct DirectoryId(usize);

impl DirectoryId {
    fn increment(&mut self) {
        self.0 += 1
    }
}

#[derive(Debug)]
struct Directory {
    id: DirectoryId,
    name: String,
    size: u64,
    parent: Option<DirectoryId>,
    children: Vec<DirectoryId>,
}

impl Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Directory {
    fn new(id: DirectoryId, name: String, parent: Option<DirectoryId>) -> Self {
        Self {
            id,
            name,
            size: 0,
            parent,
            children: Vec::new(),
        }
    }
}

#[derive(Debug, Default)]
struct Nodes {
    dirs: Vec<Directory>,
    id: DirectoryId,
}

impl Nodes {
    fn root(&mut self) -> DirectoryId {
        self.add_new_dir("/", None)
    }

    fn dir(&mut self, name: String, parent: DirectoryId) -> DirectoryId {
        self.add_new_dir(name, Some(parent))
    }

    fn add_new_dir<S: Into<String>>(
        &mut self,
        name: S,
        parent: Option<DirectoryId>,
    ) -> DirectoryId {
        let dir = Directory::new(self.id, name.into(), parent);
        self.dirs.push(dir);
        let id = self.id;
        self.id.increment();

        id
    }

    fn find_by_name(&self, cwd: DirectoryId, name: &str) -> Option<DirectoryId> {
        let dir = self.get_dir_ref(cwd);
        dir.children
            .iter()
            .map(|id| self.get_dir_ref(*id))
            .find_map(|x| if x.name == name { Some(x.id) } else { None })
    }

    fn get_dir_ref(&self, id: DirectoryId) -> &Directory {
        &self.dirs[id.0]
    }

    fn get_dir_mut(&mut self, id: DirectoryId) -> &mut Directory {
        &mut self.dirs[id.0]
    }

    fn size(&self, id: DirectoryId) -> u64 {
        let dir = self.get_dir_ref(id);
        let mut size = dir.size;
        for child in dir.children.iter() {
            size += self.size(*child);
        }

        size
    }
}

fn traverse(input: &str) -> Nodes {
    let mut fs = Nodes::default();
    let root = fs.root();
    let mut cwd = root;
    for line in input.lines().map(parse_line) {
        match line {
            ParsedLine::Cmd(Cmd::Cd(dir)) => match &*dir {
                "/" => {
                    cwd = root;
                }
                ".." => {
                    cwd = fs
                        .get_dir_ref(cwd)
                        .parent
                        .expect(&format!("No parent for {cwd:?}"))
                }
                _ => cwd = fs.find_by_name(cwd, &dir).unwrap(),
            },
            ParsedLine::Cmd(Cmd::Ls) => {}
            ParsedLine::DirContents(DirContents::Dir(dir)) => {
                let new_dir = fs.dir(dir, cwd);
                fs.get_dir_mut(cwd).children.push(new_dir);
            }
            ParsedLine::DirContents(DirContents::File(file)) => fs.get_dir_mut(cwd).size += file,
        }
    }

    fs
}

pub fn part_one(input: &str) -> Option<u64> {
    let fs = traverse(input);
    let max = 100000;
    let mut total = 0;
    for dir in fs.dirs.iter() {
        let size = fs.size(dir.id);
        if size <= max {
            total += size;
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let fs = traverse(input);
    let total = 70000000;
    let free = 30000000;

    let root_size = fs.size(fs.dirs[0].id);
    let unused = total - root_size;

    let needed = free - unused;
    fs.dirs
        .iter()
        .filter_map(|dir| {
            let size = fs.size(dir.id);
            if size >= needed {
                println!("{size}");
                Some(size)
            } else {
                None
            }
        })
        .min()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
