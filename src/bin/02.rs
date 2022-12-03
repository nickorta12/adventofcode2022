use std::str::FromStr;

#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn val(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn wins(&self, other: &Self) -> WinState {
        use Shape::*;
        use WinState::*;

        match (self, other) {
            (Rock, Scissors) => Win,
            (Rock, Paper) => Loss,
            (Scissors, Paper) => Win,
            (Scissors, Rock) => Loss,
            (Paper, Rock) => Win,
            (Paper, Scissors) => Loss,
            _ => WinState::Draw,
        }
    }

    fn wins_from(&self, state: WinState) -> Self {
        use Shape::*;
        use WinState::*;

        match (self, state) {
            (Rock, Win) => Paper,
            (Rock, Loss) => Scissors,
            (Rock, Draw) => Rock,
            (Paper, Win) => Scissors,
            (Paper, Loss) => Rock,
            (Paper, Draw) => Paper,
            (Scissors, Win) => Rock,
            (Scissors, Loss) => Paper,
            (Scissors, Draw) => Scissors,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum WinState {
    Win,
    Loss,
    Draw,
}

impl WinState {
    fn points(&self) -> u32 {
        match self {
            WinState::Win => 6,
            WinState::Loss => 0,
            WinState::Draw => 3,
        }
    }
}

#[derive(Debug)]
struct ParseError;

macro_rules! strat_guide {
    ($name:ident, $rock:ident, $paper:ident, $scissors:ident) => {
        #[derive(Debug, Clone, Copy)]
        enum $name {
            $rock,
            $paper,
            $scissors,
        }

        impl From<$name> for Shape {
            fn from(other: $name) -> Self {
                match other {
                    $name::$rock => Shape::Rock,
                    $name::$paper => Shape::Paper,
                    $name::$scissors => Shape::Scissors,
                }
            }
        }

        impl FromStr for $name {
            type Err = ParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    stringify!($rock) => Ok(Self::$rock),
                    stringify!($paper) => Ok(Self::$paper),
                    stringify!($scissors) => Ok(Self::$scissors),
                    _ => Err(ParseError),
                }
            }
        }
    };
}

strat_guide!(Player, X, Y, Z);
strat_guide!(Opponent, A, B, C);

struct Game {
    player: Player,
    opponent: Opponent,
}

impl Game {
    fn game(&self) -> u32 {
        let player: Shape = self.player.into();

        player.val() + player.wins(&self.opponent.into()).points()
    }

    fn rigged(&self) -> u32 {
        let game_result = match self.player {
            Player::X => WinState::Loss,
            Player::Y => WinState::Draw,
            Player::Z => WinState::Win,
        };

        let opponent: Shape = self.opponent.into();

        opponent.wins_from(game_result).val() + game_result.points()
    }
}

fn parse_line(input: &str) -> Game {
    let mut parts = input.split(' ').into_iter();
    let opponent = parts.next().unwrap().parse().unwrap();
    let player = parts.next().unwrap().parse().unwrap();

    Game { player, opponent }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    for line in input.lines() {
        total += parse_line(line).game();
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total = 0;
    for line in input.lines() {
        total += parse_line(line).rigged();
    }

    Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
