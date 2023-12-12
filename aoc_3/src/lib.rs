use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
const LINE_LEN: usize = 140; // Taken from the file itself, very much a magic number

const SPECIAL: [u8; 10] = [
    '*' as u8, '&' as u8, '+' as u8, '-' as u8, '=' as u8, '/' as u8, '%' as u8, '$' as u8,
    '@' as u8, '#' as u8,
];

const NUM_BUF_SIZE: usize = 4;
const ASCII_ZERO: u8 = '0' as u8;
const ASCII_NINE: u8 = '9' as u8;

fn parse_schematics<P: AsRef<Path>>(fp: P) -> u32 {
    let lines: Vec<String> = read_lines(fp).unwrap().map(|l| l.unwrap()).collect();
    let chars: Vec<&u8> = lines.iter().flat_map(|f| f.as_bytes()).collect_vec();

    let mut idx = 0;

    let mut data: Vec<u32> = Vec::with_capacity(16);

    while idx < chars.len() {
        let current = *chars[idx];

        if current == '.' as u8 || SPECIAL.contains(&current) {
            idx += 1;
            continue;
        }
        // println!("Current: {}", current as char);

        match current.is_ascii_digit() {
            true => {
                let start_idx = idx;
                let mut end_idx = idx + 1;

                while end_idx < chars.len() && chars[end_idx as usize].is_ascii_digit() {
                    end_idx += 1;
                }

                let mut found_special = false;

                for i in start_idx..end_idx {
                    let indices = generate_indices(i, chars.len());
                    match contains_special_char(indices, &chars) {
                        true => {
                            found_special = true;
                            break;
                        }
                        false => continue,
                    }
                }
                let (num, jump) = parse_num(&chars[start_idx..end_idx]);

                if found_special {
                    data.push(num);
                }

                idx += jump as usize;
            }
            false => {
                idx += 1;
                continue;
            }
        }
    }
    data.iter().sum()
}

fn parse_gear_ratio<P: AsRef<Path>>(fp: P) -> u32 {
    let lines: Vec<String> = read_lines(fp).unwrap().map(|l| l.unwrap()).collect();
    let chars: Vec<&u8> = lines.iter().flat_map(|f| f.as_bytes()).collect_vec();

    let mut idx = 0;

    let mut data: Vec<u32> = Vec::with_capacity(16);

    while idx < chars.len() {
        let current = *chars[idx];

        if current != '*' as u8 {
            idx += 1;
            continue;
        }

        let neighbours = generate_indices(idx, chars.len());
        let number_neighbours = flatten_continous_indices(&num_in_neighbours(neighbours, &chars));

        if number_neighbours.len() != 2 {
            idx += 1;
            continue;
        }

        let mut ratio = 1;
        for id in number_neighbours {
            let (num, _) = parse_number::<u32>(id, chars.as_slice());
            ratio *= num;
        }
        data.push(ratio);

        idx += 1;
    }
    data.iter().sum()
}

fn num_in_neighbours(indices: [isize; 8], chars: &[&u8]) -> Vec<usize> {
    let mut nums = Vec::with_capacity(8);
    for idx in indices.into_iter() {
        if idx < 0 || idx >= chars.len() as isize {
            continue;
        }

        if chars[idx as usize].is_ascii_digit() {
            nums.push(idx as usize);
        }
    }
    nums.sort();
    nums
}

fn flatten_continous_indices(indices: &[usize]) -> Vec<usize> {
    let mut prev = indices[0];
    let mut flattened = Vec::with_capacity(indices.len());
    flattened.push(prev);
    for idx in indices.into_iter().skip(1) {
        if *idx != prev + 1 {
            flattened.push(*idx);
        }
        prev = *idx;
    }
    flattened
}

fn parse_num<F: FromStr>(s: &[&u8]) -> (F, u8)
where
    <F as FromStr>::Err: std::fmt::Debug,
{
    let mut num_buf: [u8; NUM_BUF_SIZE] = ['0' as u8; NUM_BUF_SIZE];

    let _t = s.iter().map(|c| **c).collect_vec();


    let mut i = 0;

    for c in s {
        match c {
            ASCII_ZERO..=ASCII_NINE => {
                num_buf[i] = **c;
                i += 1;
            }
            _ => {
                break;
            }
        }
    }

    let num = std::str::from_utf8(&num_buf[0..i])
        .unwrap()
        .parse::<F>()
        .unwrap();
    let jump = i as u8 + 1; // +1 for the space

    (num, jump)
}

fn parse_number<F: FromStr>(idx: usize, chars: &[&u8]) -> (F, usize)
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
        num_buf[i] = **c;
        i -= 1;
    }

    let num = std::str::from_utf8(&num_buf).unwrap().parse::<F>().unwrap();

    (num, end_idx)
}

#[inline(always)]
fn contains_special_char(indices: [isize; 8], chars: &[&u8]) -> bool {
    for idx in indices.into_iter() {
        if idx < 0 || idx >= chars.len() as isize {
            continue;
        }

        if SPECIAL.contains(&chars[idx as usize]) {
            return true;
        }
    }
    false
}

#[inline(always)]
fn generate_indices(idx: usize, max: usize) -> [isize; 8] {
    let mut indices = [0; 8];
    let mut i = 0;
    for x in -1..=1 {
        for y in -1..=1 {
            if x == 0 && y == 0 {
                continue;
            }
            indices[i] = (idx as isize + x + y * LINE_LEN as isize).clamp(-1, max as isize - 1);
            i += 1;
        }
    }
    indices
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
    fn parse_scheme() {
        println!("Result: {}", parse_schematics("resources/input.txt"));
    }

    #[test]
    fn parse_gears() {
        println!("Result: {}", parse_gear_ratio("resources/input.txt"));
    }
}
