//! Day 2: Gift Shop.
//!
//! <https://adventofcode.com/2025/day/2>

use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

/// Skeleton function for day solution
pub fn solve(input_file: &Path) -> Result<(Option<String>, Option<String>)> {
    let file = File::open(input_file)?;
    let mut reader = BufReader::new(file);
    let intervals = read_data(&mut reader)?;
    let res_1 = check_intervals(&intervals, is_pair_valid).to_string();
    let res_2 = check_intervals(&intervals, is_any_valid).to_string();
    Ok((Some(res_1), Some(res_2)))
}

/// Read intervals from file
pub fn read_data(reader: &mut impl BufRead) -> Result<Vec<(u128, u128)>> {
    let mut data: Vec<(u128, u128)> = Vec::new();
    let mut buf: String = String::new();
    reader.read_line(&mut buf)?;
    let intervals: Vec<&str> = buf.trim().split(',').collect();
    for int in intervals {
        let borders: Vec<&str> = int.split('-').collect();
        data.push((borders[0].parse()?, borders[1].parse()?));
    }
    Ok(data)
}

/// Checks if the id is valid according to rules in part one
pub fn is_pair_valid(id: u128) -> bool {
    let id_chars: Vec<u8> = id.to_string().bytes().collect();
    if !id_chars.len().is_multiple_of(2) {
        return true;
    }

    let shift = id_chars.len() / 2;
    if id_chars[0..shift] == id_chars[shift..] {
        return false;
    }
    true
}

/// Checks if the id is valid according to rules in part two
pub fn is_any_valid(id: u128) -> bool {
    let id_chars: Vec<u8> = id.to_string().bytes().collect();
    let mut valid = true;
    // shift the window on some sizes to check is it a copy
    for shift in 1..=id_chars.len() / 2 {
        if !id_chars.len().is_multiple_of(shift) {
            continue;
        }
        let times = id_chars.len() / shift;
        let first = &id_chars[0..shift]; // first window
        let mut copy_found = true;
        for i in 1..times {
            if first != &id_chars[i * shift..(i + 1) * shift] {
                copy_found = false; // at least one window is not a copy
                break;
            }
        }
        if copy_found {
            // invalid number
            valid = false;
            break;
        }
    }
    valid
}

/// Check vector of intervals and sum up all invalid ids
pub fn check_intervals(intervals: &Vec<(u128, u128)>, validity_func: fn(u128) -> bool) -> u128 {
    let mut cnt = 0;
    for int in intervals {
        for num in int.0..=int.1 {
            if !validity_func(num) {
                cnt += num;
            }
        }
    }
    cnt
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part_one() {
        let mut reader = DATA.as_bytes();
        let intervals = read_data(&mut reader).unwrap();
        let res_1 = check_intervals(&intervals, is_pair_valid);
        assert_eq!(res_1, 1227775554);
    }

    #[test]
    fn test_part_two() {
        let mut reader = DATA.as_bytes();
        let intervals = read_data(&mut reader).unwrap();
        let res_1 = check_intervals(&intervals, is_any_valid);
        assert_eq!(res_1, 4174379265);
    }
}
