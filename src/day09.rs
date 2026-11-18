//! Day 9: Movie Theater
//!
//! <https://adventofcode.com/2025/day/9>

//!

use std::{
    cmp::{max, min},
    collections::HashMap,
    fmt::{Debug, Display, Write},
    hash::Hash,
    io::BufRead,
    ops::Index,
};

use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    col: usize,
    row: usize,
}

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
enum Tile {
    #[default]
    Unknown, // unknown tile
    Corner, // corner tile
    Edge,   // edge tile
    Inner,  // inner tile
    Outer,  // outer tile
}

/// Theater
/// --------------------------------------------------------------------------
#[derive(Debug)]
struct Theater(Vec<Position>);
impl Theater {
    fn new() -> Self {
        Self(Vec::new())
    }
    fn add_tile(&mut self, pos: Position) {
        self.0.push(pos);
    }
    fn iter(&self) -> TheaterIterator<'_> {
        TheaterIterator {
            values: &self.0,
            index: 0,
        }
    }
    fn len(&self) -> usize {
        self.0.len()
    }
    fn first(&self) -> Option<&Position> {
        self.0.first()
    }
}

impl Index<usize> for Theater {
    type Output = Position;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

struct TheaterIterator<'a> {
    values: &'a [Position],
    index: usize,
}

impl<'a> Iterator for TheaterIterator<'a> {
    type Item = &'a Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.values.len() {
            let res = &self.values[self.index];
            self.index += 1;
            Some(res)
        } else {
            None
        }
    }
}

type TileMap = Vec<Vec<Tile>>;
/// Theater map in squized coordinates
/// --------------------------------------------------------------------------
#[derive(Debug)]
struct SquizedMap {
    tiles: TileMap,
    refl: HashMap<Position, Position>, // squized -> real positions
    refl_back: HashMap<Position, Position>, // real positions -> squized
}
impl SquizedMap {
    fn new() -> Self {
        Self {
            tiles: Vec::new(),
            refl: HashMap::new(),
            refl_back: HashMap::new(),
        }
    }

    fn get_tile(&self, pos: Position) -> &Tile {
        let Position { row, col } = pos;
        &self.tiles[row][col]
    }
    fn set_tile(&mut self, pos: Position, tile: Tile) {
        let Position { row, col } = pos;
        self.tiles[row][col] = tile;
    }

    fn from_real_map(theater: &Theater) -> Self {
        let mut squize_map = Self::new();

        // check if theater is empty
        if theater.len() == 0 {
            return squize_map; // empty squized map
        }

        // vector of squized rows (indexes of row numbers * 2)
        let mut rows: Vec<usize> = theater.iter().map(|p| p.row).collect();
        rows.sort();
        rows.dedup();
        let squized_scale_rows: Vec<usize> = (0..rows.len()).map(|i| i * 2).collect();

        // vector of squized colimns (indexes of col numbers * 2)
        let mut cols: Vec<usize> = theater.iter().map(|p| p.col).collect();
        cols.sort();
        cols.dedup();
        let squized_scale_cols: Vec<usize> = (0..cols.len()).map(|i| i * 2).collect();

        // create empty squized tile map and add 1 row or column on each side
        for _ in 0..squized_scale_rows.last().unwrap() + 3 {
            squize_map
                .tiles
                .push(vec![Tile::Unknown; squized_scale_cols.last().unwrap() + 3]);
        }

        // calculate squize tables for both directions
        for pos in theater.iter() {
            let old_row_ind = rows.binary_search(&pos.row).expect("Row should exist");
            let new_row = squized_scale_rows[old_row_ind] + 1; // mind 1 empty row

            let old_col_ind = cols.binary_search(&pos.col).expect("Column should exist");
            let new_col = squized_scale_cols[old_col_ind] + 1; // mind 1 empty column

            // put corners on the map
            squize_map.tiles[new_row][new_col] = Tile::Corner;

            // prepare reflection for squized position into real position
            squize_map.refl.insert(
                Position {
                    col: new_col,
                    row: new_row,
                },
                *pos,
            );

            // prepare reflection for real position into squized position
            squize_map.refl_back.insert(
                *pos,
                Position {
                    col: new_col,
                    row: new_row,
                },
            );
        }

        // draw edges between corners
        let mut prev_pos = theater[0];
        for i in 0..theater.len() {
            let curr_pos = theater[i];
            let prev_squized = squize_map.refl_back[&prev_pos];
            let curr_squized = squize_map.refl_back[&curr_pos];
            squize_map.draw_line(prev_squized, curr_squized, Tile::Edge);
            prev_pos = curr_pos;
        }
        // draw edge between last and first positions
        let prev_squized = squize_map.refl_back[&prev_pos];
        let first_squized = squize_map.refl_back[theater.first().expect("More than 0 items")];
        squize_map.draw_line(prev_squized, first_squized, Tile::Edge);
        squize_map
    }

    /// Draw line between corners
    fn draw_line(&mut self, p1: Position, p2: Position, colour: Tile) {
        let Position {
            row: p1_row,
            col: p1_col,
        } = p1;
        let Position {
            row: p2_row,
            col: p2_col,
        } = p2;
        if p1_row == p2_row {
            // horisontal line
            let min_col = min(p1_col, p2_col);
            let max_col = max(p1_col, p2_col);
            for i in min_col + 1..max_col {
                self.set_tile(
                    Position {
                        row: p1_row,
                        col: i,
                    },
                    colour,
                );
            }
        } else if p1_col == p2_col {
            // vertival line
            let min_row = min(p1_row, p2_row);
            let max_row = max(p1_row, p2_row);
            for i in min_row + 1..max_row {
                self.set_tile(
                    Position {
                        row: i,
                        col: p1_col,
                    },
                    colour,
                );
            }
        }
    }

    fn get_neighbours(&self, p: Position, neighbours: &mut Vec<Position>) {
        // top neighbour
        if p.row > 0 {
            neighbours.push(Position {
                row: p.row - 1,
                col: p.col,
            });
        }
        // botom neighbour
        if p.row < self.tiles.len() - 1 {
            neighbours.push(Position {
                row: p.row + 1,
                col: p.col,
            });
        }
        // left neighbour
        if p.col > 0 {
            neighbours.push(Position {
                row: p.row,
                col: p.col - 1,
            });
        }
        // right neighbour
        if p.col < self.tiles[0].len() - 1 {
            neighbours.push(Position {
                row: p.row,
                col: p.col + 1,
            });
        }
    }

    /// Paint aria based on DFS algorithm
    fn paint_aria(&mut self, pos: Position, colour: Tile) {
        let mut stack: Vec<Position> = Vec::new();
        self.set_tile(pos, colour);
        self.get_neighbours(pos, &mut stack);
        while let Some(p) = stack.pop() {
            if ![Tile::Corner, Tile::Edge, Tile::Outer].contains(self.get_tile(p)) {
                self.set_tile(p, colour);
                self.get_neighbours(p, &mut stack);
            }
        }
    }
}
impl Display for SquizedMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for tile in row {
                let ch = match tile {
                    Tile::Edge => 'X',
                    Tile::Corner => '#',
                    Tile::Unknown => '.',
                    Tile::Inner => '*',
                    Tile::Outer => '+',
                };
                f.write_char(ch)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

/// Read theater from file
fn read_theater(reader: impl BufRead) -> Result<Theater> {
    let mut theater = Theater::new();
    for line in reader.lines() {
        let v: Vec<usize> = line?
            .trim()
            .split(',')
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()?;
        if let &[col, row, ..] = v.as_slice() {
            let curr_pos: Position = Position { row, col };
            theater.add_tile(curr_pos);
        }
    }
    Ok(theater)
}

/// Solve part one
/// -----------------------------------------------------------------------------------------------
/// Решение:
/// 1. Перебрать все комбинации углов
/// 2. Для каждой комбинации вычистить площадь
/// 3. Найти максимальную площадь
/// -----------------------------------------------------------------------------------------------
fn part_one(theater: &Theater) -> Option<usize> {
    let mut arias: Vec<usize> = Vec::new();
    // find all combinations
    for pos in theater.iter().combinations(2) {
        if let &[c1, c2, ..] = pos.as_slice() {
            let rect = Rectangle::from_corners(c1, c2);
            arias.push(rect.area());
        }
    }
    // select max aria
    arias.iter().copied().max()
}

/// Solve part two
/// ----------------------------------------------------------------------------------------------
/// Решение
/// 1. нриготовить сжатое отображение карты с углами и ребрами
/// 2. найти свободный угол
/// 3. начиная с него залить все внешние клетки
/// 4. залить остальные незакрашенные клетки другим цветом, это будут внутренние клетки
/// 5. собрать все углы, порядок не важен
/// 6. построить все возможные прямоугольники на сжатой системе координат
/// 7. отфильтровать только валидные прямоугольники, для которых все точки окрашены как внутренние
/// 8. пересчитать прощади прямоугольников на реальную систему коордитат
/// 10. выбрать максимальную площадь
/// -----------------------------------------------------------------------------------------------
fn part_two(theater: &Theater) -> Option<usize> {
    // 1 prepare Squized Map
    let mut sm = SquizedMap::from_real_map(theater);

    // 2. find the corner with unknown tile near the border - (0,0) is alwais unknown
    let start_position = Position { row: 0, col: 0 };

    // 3. paint all outer tiles
    sm.paint_aria(start_position, Tile::Outer);

    // 4. paint all internal tiles
    for row in &mut sm.tiles {
        for tile in row {
            if tile == &Tile::Unknown {
                *tile = Tile::Inner;
            }
        }
    }

    // 5. collect all corners
    let mut corners: Vec<Position> = Vec::new();
    for (r, row) in sm.tiles.iter().enumerate() {
        for (c, tile) in row.iter().enumerate() {
            if tile == &Tile::Corner {
                corners.push(Position { col: c, row: r });
            }
        }
    }

    // 6. create all possible rectangles
    let pref = PrefMatrix::from_map(&sm.tiles);
    let mut rects: Vec<Rectangle> = Vec::new();
    for pair in corners.iter().combinations(2) {
        if let &[c1, c2, ..] = pair.as_slice() {
            let rect = Rectangle::from_corners(c1, c2);
            // 7. filter only valid rectangles
            if pref.check_rect(&rect) {
                rects.push(rect);
            }
        }
    }

    // 8. calculate real arias
    let mut areas: Vec<usize> = Vec::new();
    for rect in rects {
        let real_c1 = sm.refl.get(&rect.c1).unwrap();
        let real_c2 = sm.refl.get(&rect.c2).unwrap();
        let real_rect = Rectangle::from_corners(real_c1, real_c2);
        areas.push(real_rect.area());
    }

    areas.iter().copied().max()
}

/// PrefMatrix - Summed Aria Table or Integral Image
/// --------------------------------------------------------------------------
struct PrefMatrix(Vec<Vec<usize>>);
impl PrefMatrix {
    /// Create from Map = Vec<Vec<Tiles>>>
    #[allow(clippy::ptr_arg)]
    fn from_map(tiles: &Vec<Vec<Tile>>) -> Self {
        let row_num = tiles.len();
        let col_num = tiles[0].len();
        // create matrix and add extra row and column at top and left sides
        let mut pref: Vec<Vec<usize>> = vec![vec![0; col_num + 1]; row_num + 1];
        // fill the matrix with shift 1 verticaly and 1 horizontaly
        for row in 0..row_num {
            for col in 0..col_num {
                let val = if tiles[row][col] == Tile::Outer { 1 } else { 0 };
                pref[row + 1][col + 1] =
                    val + pref[row][col + 1] + pref[row + 1][col] - pref[row][col];
            }
        }
        PrefMatrix(pref)
    }
    fn check_rect(&self, rect: &Rectangle) -> bool {
        let min_row = min(rect.c1.row, rect.c2.row) + 1;
        let max_row = max(rect.c1.row, rect.c2.row) + 1;
        let min_col = min(rect.c1.col, rect.c2.col) + 1;
        let max_col = max(rect.c1.col, rect.c2.col) + 1;
        // D - C - B + A
        self.0[max_row][max_col] + self.0[min_row][min_col]
            - self.0[max_row][min_col]
            - self.0[min_row][max_col]
            == 0
    }
}
impl Display for PrefMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for item in row {
                write!(f, "{:3}", item)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

/// Just rectangle
/// --------------------------------------------------------------------------
#[derive(Debug)]
struct Rectangle {
    c1: Position,
    c2: Position,
}
impl Rectangle {
    fn from_corners(c1: &Position, c2: &Position) -> Self {
        Self { c1: *c1, c2: *c2 }
    }
    fn min_row(&self) -> usize {
        min(self.c1.row, self.c2.row)
    }
    fn max_row(&self) -> usize {
        max(self.c1.row, self.c2.row)
    }
    fn min_col(&self) -> usize {
        min(self.c1.col, self.c2.col)
    }
    fn max_col(&self) -> usize {
        max(self.c1.col, self.c2.col)
    }
    fn area(self) -> usize {
        (self.max_row() - self.min_row() + 1) * (self.max_col() - self.min_col() + 1)
    }
}

/// Solve day09
#[cfg(not(tarpaulin_include))]
pub fn solve(reader: impl BufRead) -> Result<(Option<String>, Option<String>)> {
    let theater = read_theater(reader)?;

    let res1 = part_one(&theater).map(|v| v.to_string());
    let res2 = part_two(&theater).map(|v| v.to_string());

    Ok((res1, res2))
}

#[cfg(test)]
mod test {
    use crate::day09::*;

    const DATA: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    #[test]
    fn test_part_one() {
        let theater = read_theater(DATA.as_bytes()).unwrap();
        let res = part_one(&theater).unwrap();
        assert_eq!(res, 50);
    }

    #[test]
    fn test_part_two() {
        let theater = read_theater(DATA.as_bytes()).unwrap();
        let res = part_two(&theater).unwrap();
        assert_eq!(res, 24);
    }
}
