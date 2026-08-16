//! Day 3: Lobby
//!
//! <https://adventofcode.com/2025/day/3>

use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

/// Solve day03
pub fn solve(input_file: &Path) -> Result<(Option<String>, Option<String>)> {
    let file = File::open(input_file)?;
    let mut reader = BufReader::new(file);
    let batteries = read_batteries(&mut reader)?;
    let res_1 = get_max_sum_joltage(&batteries, 2).to_string();
    let res_2 = get_max_sum_joltage(&batteries, 12).to_string();
    Ok((Some(res_1), Some(res_2)))
}

/// Get index of maximal element in the battery slice
fn get_max_element_index(elements: &[u8]) -> Option<usize> {
    if let Some(max_item) = elements.iter().max() {
        elements.iter().position(|i| i == max_item)
    } else {
        None
    }
}

/// Get max possible joltage of the battery switching on on_num elements
fn get_max_joltage(battery: &str, on_num: usize) -> u128 {
    let mut result_value: Vec<u128> = Vec::new();
    let evements: Vec<u8> = battery.bytes().map(|ch| ch - b'0').collect();
    // make sliding slice
    let mut l_border = 0;
    let mut r_border = evements.len() - on_num + 1;
    for _ in 0..on_num {
        let slice = &evements[l_border..r_border];
        if let Some(index) = get_max_element_index(slice) {
            result_value.push(slice[index] as u128);
            l_border += index + 1; // index in slice
            r_border += 1;
        }
    }
    result_value
        .into_iter()
        .reduce(|a, b| a * 10 + b)
        .unwrap_or(0)
}

/// Calculate sum of max joltage of all batteries
fn get_max_sum_joltage(batteries: &[String], on_num: usize) -> u128 {
    batteries
        .iter()
        .map(|battery| get_max_joltage(battery, on_num))
        .sum()
}

/// Read bateries from file
fn read_batteries(reader: impl BufRead) -> Result<Vec<String>> {
    let mut result: Vec<String> = Vec::new();
    for line in reader.lines() {
        result.push(line?.trim().to_string());
    }
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
";

    #[test]
    fn test_part_one() {
        let batteries = read_batteries(DATA.as_bytes()).unwrap();
        let res_1 = get_max_sum_joltage(&batteries, 2);
        assert_eq!(res_1, 357);
    }

    #[test]
    fn test_part_two() {
        let batteries = read_batteries(DATA.as_bytes()).unwrap();
        let res_2 = get_max_sum_joltage(&batteries, 12);
        assert_eq!(res_2, 3121910778619);
    }
}
