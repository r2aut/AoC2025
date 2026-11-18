//! Day 1: Secret Entrance
//!
//! <https://adventofcode.com/2025/day/1>

use anyhow::{Result, anyhow};
use std::io::BufRead;

/// Вращение диска в кликах: right - positive, left - negative
type Rotation = i32;

/// Диск от сейфа
#[derive(Debug)]
struct DialDisk {
    size: i32,            // число позиций на диске
    pos: i32,             // текущая позиция диска
    ended_zeros_cnt: i32, // число остановок диска на нуле
    passed_zero_cnt: i32, // число прохождений диска через ноль
}

impl DialDisk {
    fn new() -> Self {
        Self {
            size: 100,
            pos: 50,
            ended_zeros_cnt: 0,
            passed_zero_cnt: 0,
        }
    }

    /// Rotate disk
    fn rotate(&mut self, rotation: Rotation) {
        let new_pos: i32 = self.pos + rotation;
        if new_pos.rem_euclid(self.size) == 0 {
            self.ended_zeros_cnt += 1;
        }
        if new_pos >= self.size {
            self.passed_zero_cnt += new_pos / self.size;
        }
        if new_pos <= 0 && self.pos > 0 {
            self.passed_zero_cnt -= (new_pos - 1).div_euclid(self.size);
        } else if new_pos < 0 && self.pos == 0 {
            self.passed_zero_cnt -= (new_pos - 1).div_euclid(self.size) + 1;
        }
        self.pos = new_pos.rem_euclid(self.size);
    }
}

/// Read commands
fn read_rotations(reader: impl BufRead) -> Result<Vec<Rotation>> {
    let mut res: Vec<Rotation> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let dir_str = &line[0..1];
        let rot_str = &line[1..];
        let clicks: i32 = rot_str.parse()?;
        res.push(match dir_str {
            "L" => -clicks,
            "R" => clicks,
            _ => return Err(anyhow!(format!("Wrong direction {}", line))),
        });
    }
    Ok(res)
}

/// Solve day01
#[cfg(not(tarpaulin_include))]
pub fn solve(reader: impl BufRead) -> Result<(Option<String>, Option<String>)> {
    let data = read_rotations(reader)?;
    let mut disk = DialDisk::new();
    for rotation in data {
        disk.rotate(rotation);
    }
    let res_1 = Some(disk.ended_zeros_cnt.to_string());
    let res_2 = Some(disk.passed_zero_cnt.to_string());

    Ok((res_1, res_2))
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    fn test_rotation() {
        let data = read_rotations(DATA.as_bytes()).unwrap();
        let mut disk: DialDisk = DialDisk::new();
        for rotation in data {
            disk.rotate(rotation);
        }
        assert_eq!(disk.ended_zeros_cnt, 3);
        assert_eq!(disk.passed_zero_cnt, 6);
    }
}
