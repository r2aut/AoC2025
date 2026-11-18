//! Day 5: Cafeteria
//!
//! <https://adventofcode.com/2025/day/5>

use anyhow::Result;
use std::{io::BufRead, ops::Sub};

type Ingredient = u128;

/// Interval of ingredients (T-ingredient type)
#[derive(Debug, Clone, Copy)]
struct Interval<T> {
    beg: T,
    end: T, // closed border
}

impl<T> Interval<T>
where
    T: Ord + Eq,
{
    fn new(beg: T, end: T) -> Self {
        Self { beg, end }
    }
    fn contains(&self, item: &T) -> bool {
        *item >= self.beg && *item <= self.end
    }
    fn is_combinable_with(&self, other: &Interval<T>) -> bool
    where
        T: std::ops::Add<Output = T> + From<u64> + Copy,
    {
        let res1 = self.beg <= other.beg && self.end + T::from(1) >= other.beg;
        let res2 = other.beg <= self.beg && other.end + T::from(1) >= self.beg;
        res1 || res2
    }
    fn combine_with(&mut self, other: Interval<T>) {
        // should be applied to combinable intervals (is_combinable_with)
        if other.beg <= self.beg {
            self.beg = other.beg
        }
        if other.end >= self.end {
            self.end = other.end
        }
    }

    fn length(&self) -> usize
    where
        T: Copy + Sub<Output = T> + TryInto<usize>,
        <T as TryInto<usize>>::Error: std::fmt::Debug,
    {
        let diff = (self.end - self.beg).try_into().unwrap();
        diff + 1
    }
}

/// All intervals
#[derive(Debug)]
struct Intervals<T>(Vec<Interval<T>>);
impl<T> Intervals<T>
where
    T: Ord + Eq + Copy,
{
    fn new() -> Self {
        Intervals(Vec::new())
    }
    fn combine_all_possible(&mut self)
    where
        T: std::ops::Add<Output = T> + From<u64> + Copy,
    {
        self.0.sort_by_key(|int| int.beg);
        let mut result: Vec<Interval<T>> = Vec::new();
        let mut i = 0;
        while i < self.0.len() {
            let mut cur_int = self.0[i];
            i += 1;
            while i < self.0.len() && cur_int.is_combinable_with(&self.0[i]) {
                cur_int.combine_with(self.0[i]);
                i += 1;
            }
            result.push(cur_int);
        }
        self.0 = result
    }
    fn add(&mut self, interval: Interval<T>) {
        self.0.push(interval);
    }
}

/// Read data from file
fn read_data<T>(reader: &mut impl BufRead) -> Result<(Intervals<T>, Vec<T>)>
where
    T: Ord + Eq + Copy + From<u64>,
{
    let mut intervals: Intervals<T> = Intervals::new();
    let mut ingredients: Vec<T> = Vec::new();

    let mut buf: String = String::new();
    // read intervals
    reader.read_line(&mut buf)?;
    while !buf.trim().is_empty() {
        let int_str: Vec<&str> = buf.trim().split('-').collect();
        let int: Interval<T> = Interval::new(
            int_str[0].parse::<u64>()?.into(),
            int_str[1].parse::<u64>()?.into(),
        );
        intervals.add(int);
        buf.clear();
        reader.read_line(&mut buf)?;
    }
    // read ingredients (separated from intervals by empty line)
    buf.clear();
    reader.read_line(&mut buf)?;
    while !buf.trim().is_empty() {
        let ingred = buf.trim().parse::<u64>()?.into();
        ingredients.push(ingred);
        buf.clear();
        reader.read_line(&mut buf)?;
    }

    Ok((intervals, ingredients))
}

/// Solve part one
fn part_one<T>(intervals: &Intervals<T>, ingredients: Vec<T>) -> usize
where
    T: Ord + Eq + Copy,
{
    let mut counter = 0;
    for i in ingredients {
        if intervals.0.iter().any(|int| int.contains(&i)) {
            counter += 1;
        }
    }
    counter
}

/// Solve part two
fn part_two<T>(intervals: &Intervals<T>) -> usize
where
    T: Ord + Eq + Copy + Sub<Output = T> + TryInto<usize>,
    <T as TryInto<usize>>::Error: std::fmt::Debug,
{
    intervals.0.iter().map(|i| i.length()).sum()
}

/// Solve day04
#[cfg(not(tarpaulin_include))]
pub fn solve(mut reader: impl BufRead) -> Result<(Option<String>, Option<String>)> {
    let (mut intervals, ingredients) = read_data::<Ingredient>(&mut reader)?;
    intervals.combine_all_possible();

    let res1 = part_one::<Ingredient>(&intervals, ingredients).to_string();
    let res2 = part_two(&intervals).to_string();

    Ok((Some(res1), Some(res2)))
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    type T = u128;

    #[test]
    fn test_part_one() {
        let mut reader = DATA.as_bytes();
        let (mut intervals, ingredients) = read_data(&mut reader).unwrap();
        intervals.combine_all_possible();
        let res1 = part_one::<T>(&intervals, ingredients);
        assert_eq!(res1, 3);
    }

    #[test]
    fn test_part_two() {
        let mut reader = DATA.as_bytes();
        let (mut intervals, _) = read_data(&mut reader).unwrap();
        intervals.combine_all_possible();
        let res1 = part_two::<T>(&intervals);
        assert_eq!(res1, 14);
    }
}
