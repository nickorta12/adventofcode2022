use std::{collections::VecDeque, fmt::Display};

use itertools::Itertools;
use nom::{branch::alt, bytes::complete::tag, combinator::map, sequence::preceded, IResult};

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Addx(i32),
    Noop,
}

fn parse_addx(i: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("addx "), nom::character::complete::i32), |x| {
        Instruction::Addx(x)
    })(i)
}

fn parse_noop(i: &str) -> IResult<&str, Instruction> {
    map(tag("noop"), |_| Instruction::Noop)(i)
}

fn parse_instruction(i: &str) -> Instruction {
    let (_i, instruction) = alt((parse_addx, parse_noop))(i).unwrap();

    instruction
}

impl Instruction {
    fn cycles(&self) -> u32 {
        match self {
            Instruction::Addx(_) => 2,
            Instruction::Noop => 1,
        }
    }
}

fn instructions(input: &str) -> VecDeque<Instruction> {
    input.lines().map(parse_instruction).collect()
}

#[derive(Debug)]
struct Cpu {
    x: i32,
    current: u32,
    instructions: VecDeque<Instruction>,
    pending: Option<Instruction>,
    until_execute: u32,
    crt: Vec<bool>,
}

#[derive(Debug)]
struct CycleState {
    cycle: u32,
    x: i32,
}

impl Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CYCLE: {} | x: {} | pending: {:?} | until_execute {}",
            self.current, self.x, self.pending, self.until_execute
        )
    }
}

impl Cpu {
    const WIDTH: usize = 40;

    fn new(instructions: VecDeque<Instruction>) -> Self {
        Self {
            x: 1,
            current: 0,
            instructions,
            until_execute: 0,
            pending: None,
            crt: Vec::new(),
        }
    }

    fn execute(&mut self) {
        match self.pending {
            Some(Instruction::Addx(x)) => self.x += x,
            _ => {}
        }

        self.pending = None;
    }

    fn cycle(&mut self) -> Option<CycleState> {
        self.current += 1;
        let state = CycleState {
            cycle: self.current,
            x: self.x,
        };

        if self.pending.is_none() && self.until_execute == 0 {
            self.pending = Some(self.instructions.pop_front()?);
            self.until_execute = self.pending.unwrap().cycles();
        }

        let x_pos = (self.current as i32 % Self::WIDTH as i32) - 1;
        self.crt.push(self.x.abs_diff(x_pos) <= 1);

        // println!("START: {self}");

        // End of cycle
        self.until_execute -= 1;

        if self.until_execute == 0 {
            self.execute();
        }

        // println!("END:   {self}\n");

        Some(state)
    }

    fn print(&self) -> String {
        self.crt
            .iter()
            .map(|c| if *c { "#" } else { "." })
            .chunks(Self::WIDTH)
            .into_iter()
            .map(|mut c| c.join(""))
            .join("\n")
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut cpu = Cpu::new(instructions(input));
    let indexes = [20, 60, 100, 140, 180, 220];
    let mut sum = 0;
    while let Some(cycle) = cpu.cycle() {
        if indexes.contains(&cycle.cycle) {
            sum += cycle.cycle as i32 * cycle.x;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut cpu = Cpu::new(instructions(input));

    while cpu.cycle().is_some() {}
    println!("{}", cpu.print());

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
