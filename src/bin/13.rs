use std::cmp::Ordering;

use itertools::{EitherOrBoth, Itertools};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    multi::separated_list0,
    sequence::{separated_pair, tuple},
    IResult,
};

#[derive(Debug, Eq, Clone)]
enum Packet {
    List(Vec<Packet>),
    Int(u32),
}

fn parse_int(i: &str) -> IResult<&str, Packet> {
    map(nom::character::complete::u32, |i| Packet::Int(i))(i)
}

fn parse_list(i: &str) -> IResult<&str, Packet> {
    map(
        tuple((tag("["), separated_list0(tag(","), parse_packet), tag("]"))),
        |(_, packets, _)| Packet::List(packets),
    )(i)
}

fn parse_packet(i: &str) -> IResult<&str, Packet> {
    alt((parse_int, parse_list))(i)
}

fn parse_pair(i: &str) -> IResult<&str, (Packet, Packet)> {
    separated_pair(parse_list, tag("\n"), parse_list)(i)
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    // fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    //     match (self, other) {
    //         (Packet::List(la), Packet::List(lb)) => {
    //             for m in la.iter().zip_longest(lb) {
    //                 match m {
    //                     itertools::EitherOrBoth::Both(a, b) => {
    //                         println!("{a:?}, {b:?}");
    //                         match a.partial_cmp(b) {
    //                             Some(Ordering::Equal) => {}
    //                             other => return other,
    //                         }
    //                     }
    //                     itertools::EitherOrBoth::Left(_) => return Some(Ordering::Less),
    //                     itertools::EitherOrBoth::Right(_) => return Some(Ordering::Greater),
    //                 }
    //             }

    //             panic!("Hello")
    //         }
    //         (Packet::Int(a), Packet::Int(b)) => a.partial_cmp(b),
    //         (Packet::List(a), Packet::Int(b)) => a.partial_cmp(&vec![Packet::Int(*b)]),
    //         (Packet::Int(a), Packet::List(b)) => vec![Packet::Int(*a)].partial_cmp(b),
    //     }
    // }

    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::List(la), Packet::List(lb)) => {
                for m in la.iter().zip_longest(lb) {
                    match m {
                        EitherOrBoth::Both(a, b) => match a.cmp(b) {
                            Ordering::Equal => {}
                            other => return other,
                        },
                        EitherOrBoth::Left(_) => return Ordering::Greater,
                        EitherOrBoth::Right(_) => return Ordering::Less,
                    }
                }

                Ordering::Equal
            }
            (Packet::Int(a), Packet::Int(b)) => a.cmp(b),
            (Packet::List(a), Packet::Int(b)) => a.cmp(&vec![Packet::Int(*b)]),
            (Packet::Int(a), Packet::List(b)) => vec![Packet::Int(*a)].cmp(b),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n\n")
            .enumerate()
            .filter_map(|(i, lines)| {
                let (_, (a, b)) = parse_pair(lines).unwrap();
                if a < b {
                    Some(i as u32 + 1)
                } else {
                    None
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut packets: Vec<_> = input
        .lines()
        .filter_map(|line| parse_packet(line).ok().map(|(_, p)| p))
        .collect();

    let decoder_a = Packet::List(vec![Packet::Int(2)]);
    let decoder_b = Packet::List(vec![Packet::Int(6)]);

    packets.push(decoder_a.clone());
    packets.push(decoder_b.clone());

    packets.sort();

    Some(
        packets
            .iter()
            .enumerate()
            .filter_map(|(i, packet)| {
                if packet == &decoder_a || packet == &decoder_b {
                    Some(i as u32 + 1)
                } else {
                    None
                }
            })
            .product(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
