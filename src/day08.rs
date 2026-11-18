//! Day 8: Playground
//!
//! <https://adventofcode.com/2025/day/8>

use anyhow::Result;
use itertools::Itertools;
use petgraph::algo::{connected_components, tarjan_scc};
use petgraph::graph::{NodeIndex, UnGraph};
use std::io::BufRead;

type Distance = u64;
type JBoxGraph = UnGraph<JBox, Distance>;
type DistanceRec = (NodeIndex, NodeIndex, Distance);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct JBox {
    x: i64,
    y: i64,
    z: i64,
}
impl JBox {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
    fn distance(&self, other: &JBox) -> Distance {
        // distance will be compared as power2 of differences
        // wee dont need exact distances as float from sqrt
        ((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2))
            as Distance
    }
}

/// Read JBox graph from file
fn read_data(reader: impl BufRead) -> Result<JBoxGraph> {
    let mut graph: JBoxGraph = JBoxGraph::default();
    for line in reader.lines() {
        let coord: Vec<i64> = line?
            .trim()
            .split(',')
            .map(|s| s.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()?;
        if let &[x, y, z, ..] = coord.as_slice() {
            graph.add_node(JBox::new(x, y, z));
        }
    }
    Ok(graph)
}

/// Calculate distances between all jboxes
fn calc_distances(graph: &JBoxGraph) -> Vec<DistanceRec> {
    let mut distances: Vec<DistanceRec> = Vec::new();
    for v in graph.node_indices().combinations(2) {
        let v0 = graph[v[0]];
        let v1 = graph[v[1]];
        let dr: DistanceRec = (v[0], v[1], v0.distance(&v1));
        distances.push(dr);
    }
    // sort by distance increasing
    distances.sort_by_key(|d| d.2);
    distances
}

/// Solve Part One
fn part_one(mut graph: JBoxGraph, distances: &[DistanceRec], edge_num: usize) -> u64 {
    // make edge_num connections with shortest distances
    for d in distances.iter().take(edge_num) {
        graph.update_edge(d.0, d.1, d.2);
    }

    // get the number of connected components
    let mut components = tarjan_scc(&graph);
    components.sort_by_key(|c| c.len());
    let res1 = components
        .iter()
        .rev() // take the biggest
        .take(3) // take only 3
        .map(|c| c.len())
        .product::<usize>();
    res1 as u64
}

// solve Part Two
fn part_two(mut graph: JBoxGraph, distances: &[DistanceRec]) -> i64 {
    let mut last_x0 = 0; // x of the node of the last added edge
    let mut last_x1 = 0; // x of the node of the last added edge
    for d in distances.iter() {
        graph.update_edge(d.0, d.1, d.2);

        // until all nodes are connected (connected components == 1)
        if connected_components(&graph) == 1 {
            last_x0 = graph.node_weight(d.0).unwrap().x;
            last_x1 = graph.node_weight(d.1).unwrap().x;
            // num = n;
            break;
        }
    }
    last_x0 * last_x1
}

/// Solve day08
#[cfg(not(tarpaulin_include))]
pub fn solve(reader: impl BufRead) -> Result<(Option<String>, Option<String>)> {
    let num = 1000;
    let graph = read_data(reader)?;

    let distances = calc_distances(&graph);
    let res1 = part_one(graph.clone(), &distances, num).to_string();
    let res2 = part_two(graph, &distances).to_string();

    Ok((Some(res1), Some(res2)))
}

#[cfg(test)]
mod test {

    use super::*;

    const DATA: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    #[test]
    fn test_part_one() {
        let graph = read_data(DATA.as_bytes()).unwrap();
        let distances = calc_distances(&graph);
        let res = part_one(graph, &distances, 10);
        assert_eq!(res, 40);
    }
    #[test]
    fn test_part_two() {
        let graph = read_data(DATA.as_bytes()).unwrap();
        let distances = calc_distances(&graph);
        let res = part_two(graph, &distances);
        assert_eq!(res, 25272);
    }
}
