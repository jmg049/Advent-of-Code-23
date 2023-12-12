use std::io::BufRead;
use std::{fs::File, io, path::Path, str::FromStr};

use rayon::prelude::*;

const NUM_BUF_SIZE: usize = 4;
const INITIAL_OFFSET: usize = "Card ".len();

#[derive(Debug, Clone, PartialEq, Eq)]
struct Round {
    id: u16,
    winning_nums: Vec<u8>,
    ticket_nums: Vec<u8>,
}

impl Default for Round {
    fn default() -> Self {
        Self {
            id: 0,
            winning_nums: Vec::with_capacity(16),
            ticket_nums: Vec::with_capacity(32),
        }
    }
}

pub fn calculate_winnings<P: AsRef<Path>>(fp: P) -> io::Result<u32> {
    Ok(read_lines(fp)?
        .par_bridge()
        .map(|l| calculate_ticket_winnings(parse_line(l.unwrap().as_bytes())))
        .sum())
}

pub fn calculate_total_scratchers<P: AsRef<Path>>(fp: P) -> io::Result<u32> {
    let rounds = read_lines(fp)?
        .map(|line| parse_line(&line.unwrap().as_bytes()))
        .collect::<Vec<Round>>();

    let mut counter = vec![1u32; rounds.len()];

    for (idx, round) in rounds.iter().enumerate() {
        let n_matches = calculate_n_matches(round);
        if n_matches == 0 {
            continue;
        }

        let current_count = counter[idx];

        for count in &mut counter[(idx + 1)..=(idx + n_matches as usize)] {
            *count += current_count;
        }
    }
    Ok(counter.iter().sum())
}

#[inline(always)]
fn calculate_n_matches(round: &Round) -> u8 {
    let mut matches = 0;

    let winning_nums = &round.winning_nums;
    let ticket_nums = &round.ticket_nums;

    for num in winning_nums.iter() {
        if ticket_nums.contains(num) {
            matches += 1;
        }
    }

    matches
}

fn parse_line(line: &[u8]) -> Round {
    let mut round = Round::default();

    let mut idx = INITIAL_OFFSET;

    while line[idx] == ' ' as u8 {
        idx += 1;
    }

    let (card_num, jump) = parse_number::<u16>(idx, &line);

    round.id = card_num;
    idx = jump + 1; // +1 for the colon

    let mut winning_nums: Vec<u8> = Vec::with_capacity(16);
    let mut ticket_nums: Vec<u8> = Vec::with_capacity(32);

    let mut current_byte = line[idx];

    // Parse winning numbers
    while current_byte != '|' as u8 {
        if current_byte.is_ascii_digit() {
            let (num, jump) = parse_number::<u8>(idx, &line);
            winning_nums.push(num);

            idx = jump;
        } else {
            idx += 1;
        }
        current_byte = line[idx];
    }
    round.winning_nums = winning_nums;

    idx += 2; // Skip the pipe and the space

    // Parse ticket numbers
    while idx < line.len() {
        current_byte = line[idx];

        if current_byte.is_ascii_digit() {
            let (num, jump) = parse_number::<u8>(idx, &line);
            ticket_nums.push(num);
            idx = jump;
        } else {
            idx += 1;
        }
    }
    round.ticket_nums = ticket_nums;
    round
}

fn calculate_ticket_winnings(round: Round) -> u32 {
    let mut matches = 0;

    let winning_nums = &round.winning_nums;
    let ticket_nums = &round.ticket_nums;

    for num in winning_nums.iter() {
        if ticket_nums.contains(num) {
            matches += 1;
        }
    }
    let winnings = match matches == 0 {
        true => 0,
        false => 2u32.pow(matches - 1),
    };

    winnings
}

fn parse_number<F: FromStr>(idx: usize, chars: &[u8]) -> (F, usize)
where
    <F as FromStr>::Err: std::fmt::Debug,
{
    let mut num_buf: [u8; NUM_BUF_SIZE] = ['0' as u8; NUM_BUF_SIZE];

    let mut start_idx = idx;
    let mut end_idx = idx;

    while start_idx > 0 && chars[start_idx - 1].is_ascii_digit() {
        start_idx -= 1;
    }

    while end_idx < chars.len() && chars[end_idx].is_ascii_digit() {
        end_idx += 1;
    }

    let mut i = NUM_BUF_SIZE - 1;

    for c in chars[start_idx..end_idx].iter().rev() {
        num_buf[i] = *c;
        i -= 1;
    }

    let num = std::str::from_utf8(&num_buf).unwrap().parse::<F>().unwrap();

    (num, end_idx)
}

pub(crate) fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        let fp = "./resources/input.txt";

        let total = calculate_total_scratchers(fp).unwrap();
        println!("total: {}", total);
    }
}
