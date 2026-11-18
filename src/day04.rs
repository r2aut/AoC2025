//! Day 4: Printing Department
//!
//! <https://adventofcode.com/2025/day/4>

use anyhow::Result;
use std::{
    cmp::{max, min},
    collections::HashSet,
    io::BufRead,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: i32, // row from top to bottom
    col: i32, // column from left to right
}

/// Rollgrid implementation (only positions of rolls)
struct RollGrid {
    set: HashSet<Position>,
    top: i32,
    bottom: i32, // not included
    left: i32,
    right: i32, // not included
}
impl RollGrid {
    fn new() -> Self {
        RollGrid {
            set: HashSet::new(),
            top: 0,
            bottom: 0,
            left: 0,
            right: 0,
        }
    }

    /// Add roll to the grid
    fn add(&mut self, p: &Position) {
        self.set.insert(*p);
    }

    /// Calculate used borders
    fn calc_borders(&mut self) {
        self.top = self.set.iter().map(|p| p.row).min().unwrap_or(0);
        self.bottom = self.set.iter().map(|p| p.row).max().unwrap_or(-1) + 1;
        self.left = self.set.iter().map(|p| p.col).min().unwrap_or(0);
        self.right = self.set.iter().map(|p| p.col).max().unwrap_or(-1) + 1;
    }

    /// Find all rolls that adjacent to provided
    fn adjacent_roll_positions(&self, p: &Position) -> Vec<Position> {
        let mut result: Vec<Position> = Vec::new();
        let top_row = max(p.row - 1, self.top);
        let bottom_row = min(p.row + 1, self.bottom - 1);
        let left_col = max(p.col - 1, self.left);
        let right_col = min(p.col + 1, self.right - 1);
        for row in top_row..=bottom_row {
            for col in left_col..=right_col {
                if !(row == p.row && col == p.col) {
                    let pos = Position { row, col };
                    if self.set.contains(&pos) {
                        result.push(pos);
                    }
                }
            }
        }
        result
    }

    /// Find all accessible rolls according to rules
    fn get_all_accessible_rolls(&self) -> Vec<Position> {
        let mut res: Vec<Position> = Vec::new();
        for p in &self.set {
            if self.adjacent_roll_positions(p).len() < 4 {
                res.push(*p);
            }
        }
        res
    }

    /// Remove rolls provided
    fn remove_rolls(&mut self, rolls: &[Position]) {
        self.set.retain(|p| !rolls.contains(p));
        // self.calc_borders();
    }

    /// Get off all accessible rolls , and again, and again
    fn get_off_rolls(&mut self) -> usize {
        let mut counter = 0;
        let mut accessible_rolls = self.get_all_accessible_rolls();
        while !accessible_rolls.is_empty() {
            self.remove_rolls(&accessible_rolls);
            counter += accessible_rolls.len();
            accessible_rolls = self.get_all_accessible_rolls();
        }
        counter
    }
}

/// Read Rollgate from the file
fn read_data(reader: impl BufRead) -> Result<RollGrid> {
    let mut rg = RollGrid::new();
    for (row, line) in reader.lines().enumerate() {
        for (col, ch) in line?.chars().enumerate() {
            if ch == '@' {
                rg.add(&Position {
                    row: row as i32,
                    col: col as i32,
                });
            }
        }
    }
    rg.calc_borders();
    Ok(rg)
}

/// Solve day04
#[cfg(not(tarpaulin_include))]
pub fn solve(reader: impl BufRead) -> Result<(Option<String>, Option<String>)> {
    let mut grid = read_data(reader)?;
    let res_1 = grid.get_all_accessible_rolls().len().to_string();
    let res_2 = grid.get_off_rolls().to_string();
    Ok((Some(res_1), Some(res_2)))
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn test_part_one() {
        let grid = read_data(DATA.as_bytes()).unwrap();
        let res_1 = grid.get_all_accessible_rolls().len();
        assert_eq!(res_1, 13);
    }

    #[test]
    fn test_part_two() {
        let mut grid = read_data(DATA.as_bytes()).unwrap();
        let res_2 = grid.get_off_rolls();
        assert_eq!(res_2, 43);
    }
}
