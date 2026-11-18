//! Day 6: Trash Compactor
//!
//! <https://adventofcode.com/2025/day/6>

use anyhow::Result;
use std::io::BufRead;

type Mem = Vec<Vec<char>>;

fn read_data(reader: impl BufRead) -> Result<Vec<Problem>> {
    let mut result: Vec<Problem> = Vec::new();

    // get data into memory table
    let mut mem: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let char_vec: Vec<char> = line?.chars().collect();
        mem.push(char_vec);
    }
    // get operation line position
    let op_line_index = mem.len() - 1;
    let digits_row_num = op_line_index;

    let intervals = get_intervals(&mem, op_line_index);

    for (start, length) in intervals {
        let mut problem = Problem::new();

        #[allow(clippy::needless_range_loop)]
        for row in 0..digits_row_num {
            problem.nums.push(mem[row][start..start + length].to_vec());
        }
        problem.sign = mem[op_line_index][start];
        result.push(problem);
    }

    Ok(result)
}

fn get_intervals(mem: &Mem, char_line_index: usize) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    let char_line = &mem[char_line_index];
    let mut start = 0;
    let mut pos = 1;
    while pos < char_line.len() {
        if "*+".contains(char_line[pos]) {
            result.push((start, pos - start - 1));
            start = pos;
        }
        pos += 1;
    }
    result.push((start, pos - start));
    result
}

/// Cephalopods' math problem
#[derive(Debug)]
struct Problem {
    nums: Vec<Vec<char>>,
    sign: char,
}
impl Problem {
    fn new() -> Self {
        Self {
            nums: Vec::new(),
            sign: ' ',
        }
    }
    /// Get numbers on rows
    fn get_numbers_on_rows(&self) -> Vec<i64> {
        let mut result: Vec<i64> = Vec::new();
        for row in &self.nums {
            let mut number_str: Vec<char> = Vec::new();
            for d in row {
                number_str.push(*d);
            }
            let d_str: String = number_str.iter().collect();
            let num = d_str.trim().parse().unwrap();
            result.push(num);
        }
        result
    }
    /// Get numbers on columns
    fn get_numbers_on_columns(&self) -> Vec<i64> {
        let mut result: Vec<i64> = Vec::new();
        let row_num = self.nums.len();
        let col_num = self.nums[0].len();
        for col in (0..col_num).rev() {
            let mut number_str: Vec<char> = Vec::new();
            for row in 0..row_num {
                number_str.push(self.nums[row][col]);
            }
            let d_str: String = number_str.iter().collect();
            let num = d_str.trim().parse().unwrap();
            result.push(num);
        }
        result
    }
    /// Calculate problem getting numbers on rows
    fn calc_value_by_rows(&self) -> i64 {
        match self.sign {
            '+' => self.get_numbers_on_rows().iter().sum(),
            '*' => self.get_numbers_on_rows().iter().product(),
            _ => unreachable!(),
        }
    }
    /// Calculate problem getting numbers on columns
    fn calc_value_by_columns(&self) -> i64 {
        match self.sign {
            '+' => self.get_numbers_on_columns().iter().sum(),
            '*' => self.get_numbers_on_columns().iter().product(),
            _ => unreachable!(),
        }
    }
}

fn part_one(data: &[Problem]) -> i64 {
    data.iter().map(|p| p.calc_value_by_rows()).sum::<i64>()
}

fn part_two(data: &[Problem]) -> i64 {
    data.iter().map(|p| p.calc_value_by_columns()).sum::<i64>()
}

/// Solve day06
#[cfg(not(tarpaulin_include))]
pub fn solve(reader: impl BufRead) -> Result<(Option<String>, Option<String>)> {
    let data = read_data(reader)?;

    let res1 = part_one(&data).to_string();
    let res2 = part_two(&data).to_string();

    Ok((Some(res1), Some(res2)))
}

#[cfg(test)]
mod test {

    use super::*;

    const DATA: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

    #[test]
    fn test_part_one() {
        let data = read_data(DATA.as_bytes()).unwrap();
        let res1 = part_one(&data);
        assert_eq!(res1, 4277556);
    }

    #[test]
    fn test_part_two() {
        let data = read_data(DATA.as_bytes()).unwrap();
        let res1 = part_two(&data);
        assert_eq!(res1, 3263827);
    }
}
