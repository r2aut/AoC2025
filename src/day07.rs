//! Day 7: Laboratories
//!
//! <https://adventofcode.com/2025/day/7>

use anyhow::Result;
use std::{
    fmt::{Display, Write},
    io::BufRead,
};

/// One place of the manifold
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Space(u64),
    Source,
    Splitter(u64),
}
impl Default for Tile {
    fn default() -> Self {
        Self::Space(0)
    }
}
impl Display for Tile {
    #[cfg(not(tarpaulin_include))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Space(x) => {
                if *x == 0 {
                    f.write_char('.')
                } else {
                    f.write_char('|')
                }
            }
            Self::Source => f.write_char('S'),
            Self::Splitter(_) => f.write_char('^'),
        }
    }
}
impl From<char> for Tile {
    fn from(ch: char) -> Self {
        match ch {
            'S' => Tile::Source,
            '.' => Tile::Space(0),
            '^' => Tile::Splitter(0),
            _ => panic!("Unknown char in sourse {}", ch),
        }
    }
}

/// Tachyon manifold
struct Manifold(Vec<Vec<Tile>>);
impl Manifold {
    fn new() -> Self {
        Self(Vec::new())
    }

    /// Calc the flow of tachions on the row
    fn propagate(&mut self, row_num: usize) {
        if row_num > 0 {
            // let prev_row = self.0[row - 1].clone();
            for col in 0..self.0[row_num].len() {
                match self.0[row_num][col] {
                    Tile::Space(x) => {
                        if let Tile::Space(prev_x) = self.0[row_num - 1][col] {
                            self.0[row_num][col] = Tile::Space(x + prev_x);
                        } else if self.0[row_num - 1][col] == Tile::Source {
                            self.0[row_num][col] = Tile::Space(x + 1);
                        };
                    }
                    Tile::Splitter(x) => {
                        if col > 0
                            && let Tile::Space(x_l) = self.0[row_num][col - 1]
                            && let Tile::Space(x_t) = self.0[row_num - 1][col]
                        {
                            self.0[row_num][col - 1] = Tile::Space(x_l + x_t);
                        }
                        if col < self.0[row_num].len() - 1
                            && let Tile::Space(x_r) = self.0[row_num][col + 1]
                            && let Tile::Space(x_t) = self.0[row_num - 1][col]
                        {
                            self.0[row_num][col + 1] = Tile::Space(x_r + x_t);
                        }
                        if let Tile::Space(x_t) = self.0[row_num - 1][col]
                            && x_t > 0
                        {
                            self.0[row_num][col] = Tile::Splitter(x + x_t);
                            // self.1.insert(Position { row: row_num, col });
                        }
                    }
                    Tile::Source => {}
                }
            }
        }
    }

    /// Calc the flow of tachions on all rows
    fn propagate_all(&mut self) {
        for row_num in 0..self.0.len() {
            self.propagate(row_num);
        }
    }

    /// Get number of tachion rays went through the row
    fn get_ways(&self, row_num: usize) -> u64 {
        self.0[row_num]
            .iter()
            .filter_map(|t| {
                if let Tile::Space(x) = t {
                    Some(x)
                } else {
                    None
                }
            })
            .sum()
    }

    /// Get number of splitters that were used
    fn get_used_splitters(&self) -> u64 {
        let mut cnt = 0;
        for row in &self.0 {
            cnt += row
                .iter()
                .filter_map(|t| {
                    if let Tile::Splitter(x) = t
                        && *x > 0
                    {
                        Some(1)
                    } else {
                        None
                    }
                })
                .sum::<u64>()
        }
        cnt
    }
}
impl Display for Manifold {
    #[cfg(not(tarpaulin_include))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for tile in row {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

/// Read data from file
fn read_data(reader: impl BufRead) -> Result<Manifold> {
    let mut result = Manifold::new();
    for line in reader.lines() {
        let mut tile_line = Vec::new();
        for ch in line?.chars() {
            tile_line.push(Tile::from(ch));
        }
        result.0.push(tile_line);
    }
    Ok(result)
}

/// Solve part one
fn part_one(manifold: &Manifold) -> u64 {
    manifold.get_used_splitters()
    // let res2 = data.get_ways(data.0.len() - 1).to_string();
}

/// Solve part two
fn part_two(manifold: &Manifold) -> u64 {
    manifold.get_ways(manifold.0.len() - 1)
}

/// Solve day07
#[cfg(not(tarpaulin_include))]
pub fn solve(reader: impl BufRead) -> Result<(Option<String>, Option<String>)> {
    let mut manifold = read_data(reader)?;
    manifold.propagate_all();
    let res1 = part_one(&manifold).to_string();
    let res2 = part_two(&manifold).to_string();

    Ok((Some(res1), Some(res2)))
}

#[cfg(test)]
mod test {

    use super::*;

    const DATA: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[test]
    fn test_part_one() {
        let mut data = read_data(DATA.as_bytes()).unwrap();
        data.propagate_all();
        let res = part_one(&data);
        assert_eq!(res, 21);
    }

    #[test]
    fn test_part_two() {
        let mut data = read_data(DATA.as_bytes()).unwrap();
        data.propagate_all();
        let res = part_two(&data);
        assert_eq!(res, 40);
    }
}
