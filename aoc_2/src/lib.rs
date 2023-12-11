use std::fs::File;
use std::io;
use std::path::Path;

use std::io::BufRead;

const VALID_ROUND: Round = Round {
    n_red: 12,
    n_green: 13,
    n_blue: 14,
};

pub fn power<P: AsRef<Path>>(fp: P) -> u32 {
    let lines = read_lines(fp).unwrap();

    let games: Vec<GameData> = lines
        .map(|line| parse_line(&mut line.unwrap()).unwrap())
        .collect();

    games.iter().map(|g| g.power()).sum::<u32>()
}

pub fn sum<P: AsRef<Path>>(fp: P) -> u16 {
    let lines = read_lines(fp).unwrap();

    let games: Vec<GameData> = lines
        .map(|line| parse_line(&mut line.unwrap()).unwrap())
        .collect();

    let results: Vec<(&GameData, RoundResult)> = games
        .iter()
        .map(|game| {
            let mut valid: RoundResult = RoundResult::Valid;
            for (_, round) in game.rounds.iter().enumerate() {
                valid = valid_round(round, &VALID_ROUND);
                match valid {
                    RoundResult::Valid => {
                        continue;
                    }
                    _ => {
                        break;
                    }
                }
            }
            (game, valid)
        })
        .collect();

    results
        .iter()
        .filter(|(_, v)| *v == RoundResult::Valid)
        .map(|(g, _)| g.id)
        .sum()
}

pub(crate) fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Round {
    n_red: u8,
    n_green: u8,
    n_blue: u8,
}

impl Default for Round {
    fn default() -> Self {
        Round {
            n_red: 0,
            n_green: 0,
            n_blue: 0,
        }
    }
}

impl Into<(u8, u8, u8)> for Round {
    fn into(self) -> (u8, u8, u8) {
        (self.n_red, self.n_green, self.n_blue)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum RoundResult {
    Valid,
    InvalidRed,
    InvalidGreen,
    InvalidBlue,
}

#[inline(always)]
fn valid_round(round: &Round, compare_against: &Round) -> RoundResult {
    let r = round.n_red <= compare_against.n_red;
    let g = round.n_green <= compare_against.n_green;
    let b = round.n_blue <= compare_against.n_blue;

    if r && g && b {
        RoundResult::Valid
    } else if !r {
        RoundResult::InvalidRed
    } else if !g {
        RoundResult::InvalidGreen
    } else {
        RoundResult::InvalidBlue
    }
}

#[derive(Debug, PartialEq)]
struct GameData {
    id: u16,
    rounds: Vec<Round>,
}

impl GameData {
    pub fn new(id: u16, rounds: Vec<Round>) -> Self {
        GameData { id, rounds }
    }

    pub fn max_red(&self) -> u8 {
        self.rounds.iter().map(|r| r.n_red).max().unwrap()
    }

    pub fn max_green(&self) -> u8 {
        self.rounds.iter().map(|r| r.n_green).max().unwrap()
    }

    pub fn max_blue(&self) -> u8 {
        self.rounds.iter().map(|r| r.n_blue).max().unwrap()
    }

    pub fn max(&self) -> (u8, u8, u8) {
        (self.max_red(), self.max_green(), self.max_blue())
    }

    pub fn power(&self) -> u32 {
        let (r, g, b) = self.max();
        (r as u32) * (g as u32) * (b as u32)
    }
}

impl Default for GameData {
    fn default() -> Self {
        GameData {
            id: 0,
            rounds: Vec::new(),
        }
    }
}

const NUM_BUF_SIZE: usize = 4;
const ASCII_ZERO: u8 = '0' as u8;
const ASCII_NINE: u8 = '9' as u8;

fn parse_num(s: &[u8]) -> (u8, u8) {
    let mut num_buf: [u8; NUM_BUF_SIZE] = ['0' as u8; NUM_BUF_SIZE];

    let mut i = 0;

    for c in s {
        match c {
            ASCII_ZERO..=ASCII_NINE => {
                num_buf[i] = *c;
                i += 1;
            }
            _ => {
                break;
            }
        }
    }

    let num = std::str::from_utf8(&num_buf[0..i])
        .unwrap()
        .parse::<u8>()
        .unwrap();
    let jump = i as u8 + 1; // +1 for the space

    (num, jump)
}

fn parse_line(line: &mut str) -> io::Result<GameData> {
    let chars: &[u8] = line.as_bytes();

    let mut i = "Game ".len();
    let game_id = parse_num(&chars[i..]);

    i += game_id.1 as usize + 1; // +1 for :

    let mut rounds: Vec<Round> = Vec::with_capacity(6);

    let mut round = Round::default();
    while i < chars.len() {
        let num = parse_num(&chars[i..]);

        i += num.1 as usize;

        match chars[i] {
            b'r' => {
                round.n_red += num.0;
                i += "red".len();
            }
            b'b' => {
                round.n_blue += num.0;
                i += "blue".len();
            }
            b'g' => {
                round.n_green += num.0;
                i += "green".len();
            }
            _ => {
                break;
            }
        };

        if i >= chars.len() {
            rounds.push(round);
            break;
        }

        match chars[i] {
            b',' => {
                i += 2;
            }
            b';' => {
                i += 2;

                rounds.push(round);
                round = Round::default();
            }
            _ => {
                rounds.push(round);

                break;
            }
        }
    }

    Ok(GameData::new(game_id.0 as u16, rounds))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_num_ok() {
        let (num, jump) = parse_num(b"123 ");
        assert_eq!(num, 123);
        assert_eq!(jump, 4);

        let (num, jump) = parse_num(b"1 DAWD");
        assert_eq!(num, 1);
        assert_eq!(jump, 2);
    }
}
