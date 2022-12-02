#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum RPS {
    Rock = 0,
    Paper,
    Scissors,
}

impl RPS {
    const fn from_i(i: u8) -> Self {
        match i {
            0 => Self::Rock,
            1 => Self::Paper,
            2 => Self::Scissors,
            _ => panic!("invalid RPS value"),
        }
    }
    const fn value(self) -> u8 {
        self as u8 + 1
    }

    const fn play_to(self, outcome: Outcome) -> Self {
        let result = self as u8;
        let result = match outcome {
            Outcome::Lose => match result.checked_sub(1) {
                Some(i) => i,
                None => 2,
            },
            Outcome::Draw => result,
            Outcome::Win => {
                if result == 2 {
                    0
                } else {
                    result + 1
                }
            }
        };
        Self::from_i(result)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

#[derive(Copy, Clone, Debug)]
pub struct Round(u8);

impl Round {
    pub fn from_abc_xyz(abc: u8, xyz: u8) -> Self {
        Self(((abc - b'A') << 4) | (xyz - b'X'))
    }

    pub fn left(self) -> RPS {
        match self.0 >> 4 {
            0 => RPS::Rock,
            1 => RPS::Paper,
            2 => RPS::Scissors,
            _ => panic!("Unexpected RPS value"),
        }
    }

    pub fn right(self) -> RPS {
        match self.0 & 0x0F {
            0 => RPS::Rock,
            1 => RPS::Paper,
            2 => RPS::Scissors,
            _ => panic!("Unexpected RPS value"),
        }
    }

    pub fn outcome(self) -> Outcome {
        match self.0 & 0x0F {
            0 => Outcome::Lose,
            1 => Outcome::Draw,
            2 => Outcome::Win,
            _ => panic!("Unexpected RPS value"),
        }
    }
}

pub fn generator(s: &str) -> Vec<Round> {
    assert_eq!(s.len() % 4, 0);
    s.as_bytes()
        .chunks_exact(4)
        .map(|chunk| Round::from_abc_xyz(chunk[0], chunk[2]))
        .collect()
}

fn score(mine: RPS, yours: RPS) -> u32 {
    let diff = mine as i8 - yours as i8;
    let outcome = match diff {
        0 => Outcome::Draw,
        -2 | 1 => Outcome::Win,
        -1 | 2 => Outcome::Lose,
        _ => unreachable!(),
    };
    let value = u32::from(mine.value());
    value + u32::from(outcome as u8)
}

pub fn part_1(input: &Vec<Round>) -> u32 {
    input
        .iter()
        .copied()
        .map(|round| score(round.right(), round.left()))
        .sum()
}

pub fn part_2(input: &Vec<Round>) -> u32 {
    input
        .iter()
        .copied()
        .map(|round| {
            let outcome = round.outcome();
            let mine = round.left().play_to(outcome);
            u32::from(mine.value()) + u32::from(outcome as u8)
        })
        .sum()
}
