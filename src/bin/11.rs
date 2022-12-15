mod monkey {
    use std::collections::VecDeque;

    use anyhow::anyhow;
    use nom::{
        bytes::complete::{tag, take, take_until},
        character::complete::{alphanumeric1, newline, one_of},
        combinator::{map, map_res},
        multi::separated_list1,
        sequence::{preceded, separated_pair, terminated},
        IResult,
    };

    #[derive(Debug, PartialEq, Clone)]
    pub struct Monkey {
        items: VecDeque<u64>,
        operation: Operation,
        test: Test,
        pub inspects: u64,
    }

    #[derive(Debug)]
    pub struct MonkeyToss {
        pub to: usize,
        pub item: u64,
    }

    impl Monkey {
        pub fn parse(input: &str) -> anyhow::Result<Self> {
            let (_, monkey) = parse_monkey(input).map_err(|e| e.to_owned())?;

            Ok(monkey)
        }

        pub fn inspect(&mut self, worry_dividend: u64) -> Option<MonkeyToss> {
            let mut item = self.items.pop_front()?;
            item = self.operation.operate(item) / worry_dividend;

            self.inspects += 1;
            Some(self.test.throw(item))
        }

        pub fn add_item(&mut self, item: u64) {
            self.items.push_back(item)
        }
    }

    fn parse_monkey(i: &str) -> IResult<&str, Monkey> {
        let (i, items) = parse_items(i)?;
        newline(i)?;
        let (i, operation) = parse_operation(i)?;
        newline(i)?;
        let (i, test) = parse_test(i)?;

        Ok((
            i,
            Monkey {
                items,
                operation,
                test,
                inspects: 0,
            },
        ))
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum Operation {
        Add(u64),
        Multiply(u64),
        Divide(u64),
        Square,
    }

    impl Operation {
        fn operate(&self, item: u64) -> u64 {
            match self {
                Operation::Add(x) => item + x,
                Operation::Multiply(x) => item * x,
                Operation::Divide(x) => item / x,
                Operation::Square => item * item,
            }
        }
    }

    impl TryFrom<(char, &str)> for Operation {
        type Error = anyhow::Error;

        fn try_from((c, num_or_old): (char, &str)) -> Result<Self, Self::Error> {
            if num_or_old == "old" {
                return Ok(Self::Square);
            }
            let num = num_or_old.parse()?;
            match c {
                '+' => Ok(Operation::Add(num)),
                '*' => Ok(Operation::Multiply(num)),
                '/' => Ok(Operation::Divide(num)),
                c => Err(anyhow!("Invalid character: {}", c)),
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Test {
        dividend: u64,
        throw_true: u64,
        throw_false: u64,
    }

    impl Test {
        fn throw(&self, item: u64) -> MonkeyToss {
            if item % self.dividend == 0 {
                MonkeyToss {
                    to: self.throw_true as usize,
                    item,
                }
            } else {
                MonkeyToss {
                    to: self.throw_false as usize,
                    item,
                }
            }
        }
    }

    fn take_until_and_consume<T, I, E>(tag: T) -> impl Fn(I) -> nom::IResult<I, I, E>
    where
        E: nom::error::ParseError<I>,
        I: nom::InputTake + nom::FindSubstring<T> + nom::InputIter + nom::InputLength,
        T: nom::InputLength + Clone,
    {
        move |input| terminated(take_until(tag.clone()), take(tag.input_len()))(input)
    }

    fn parse_items(i: &str) -> IResult<&str, VecDeque<u64>> {
        let items = separated_list1(tag(", "), nom::character::complete::u64);

        map(
            preceded(take_until_and_consume("items: "), items),
            VecDeque::from,
        )(i)
    }

    fn parse_operation(i: &str) -> IResult<&str, Operation> {
        let operation = separated_pair(one_of("+*/"), tag(" "), alphanumeric1);
        map_res(
            preceded(take_until_and_consume("old "), operation),
            Operation::try_from,
        )(i)
    }

    fn parse_test(i: &str) -> IResult<&str, Test> {
        let (i, dividend) = preceded(
            take_until_and_consume("divisible by "),
            nom::character::complete::u64,
        )(i)?;

        let mut monkey_num = preceded(
            take_until_and_consume("monkey "),
            nom::character::complete::u64,
        );

        let (i, throw_true) = monkey_num(i)?;
        let (i, throw_false) = monkey_num(i)?;

        Ok((
            i,
            Test {
                dividend,
                throw_true,
                throw_false,
            },
        ))
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_parse_items() {
            let input = "  Starting items: 54, 65, 75, 74";
            assert_eq!(parse_items(input), Ok(("", vec![54, 65, 75, 74].into())));
        }

        #[test]
        fn test_parse_operation() {
            let input = "  Operation: new = old + 8";
            assert_eq!(parse_operation(input), Ok(("", Operation::Add(8))));
        }

        #[test]
        fn test_parse_test() {
            let input = "  Test: divisible by 11
            If true: throw to monkey 5
            If false: throw to monkey 6";
            assert_eq!(
                parse_test(input),
                Ok((
                    "",
                    Test {
                        dividend: 11,
                        throw_true: 5,
                        throw_false: 6
                    }
                ))
            )
        }

        #[test]
        fn test_parse_monkey() {
            let input = "Monkey 3:
            Starting items: 76, 92
            Operation: new = old + 6
            Test: divisible by 5
              If true: throw to monkey 1
              If false: throw to monkey 6";

            let monkey = Monkey {
                items: vec![76, 92].into(),
                operation: Operation::Add(6),
                test: Test {
                    dividend: 5,
                    throw_true: 1,
                    throw_false: 6,
                },
                inspects: 0,
            };

            assert_eq!(parse_monkey(input), Ok(("", monkey)));
        }
    }
}

use std::cell::RefCell;

use monkey::Monkey;

pub fn part_one(input: &str) -> Option<u64> {
    let mut monkeys: Vec<_> = input
        .split("\n\n")
        .map(|s| RefCell::new(Monkey::parse(s).unwrap()))
        .collect();

    for _ in 0..20 {
        for monkey in monkeys.iter() {
            while let Some(toss) = monkey.borrow_mut().inspect(3) {
                monkeys[toss.to].borrow_mut().add_item(toss.item);
            }
        }
    }

    monkeys.sort_by(|a, b| b.borrow().inspects.cmp(&a.borrow().inspects));
    Some(
        monkeys
            .iter()
            .take(2)
            .fold(1, |acc, x| acc * x.borrow().inspects),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
        // assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), None);
        // assert_eq!(part_two(&input), Some(2713310158));
    }
}
