//! Day 11: Reactor
//!
//! <https://adventofcode.com/2025/day/11>

use anyhow::Result;
use aoc2025::get_reader;
use petgraph::graph::{DiGraph, NodeIndex};
use std::{collections::HashMap, io::BufRead, path::Path};

struct Devices {
    graph: DiGraph<String, u32>,
    nodes: HashMap<String, NodeIndex>, // for getting NodeIndexes
}
impl Devices {
    fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            nodes: HashMap::<String, NodeIndex>::new(),
        }
    }
    fn get_node_index(&self, node: &str) -> Option<NodeIndex> {
        self.nodes.get(node).copied()
    }
    fn update_node(&mut self, node: &str) -> NodeIndex {
        *self
            .nodes
            .entry(node.to_string())
            .or_insert_with(|| self.graph.add_node(node.to_string()))
    }
    fn update_edge(&mut self, src_node: &str, dst_node: &str) {
        let src_ind = self.update_node(src_node);
        let dst_ind = self.update_node(dst_node);
        self.graph.update_edge(src_ind, dst_ind, 1);
    }
    fn count_paths(
        &self,
        current: NodeIndex,
        destination: NodeIndex,
        cache: &mut HashMap<NodeIndex, usize>,
    ) -> usize {
        // Already at the destination
        if current == destination {
            return 1;
        }
        // If ic was counted already get it from chach
        if let Some(&saved) = cache.get(&current) {
            return saved;
        }

        let mut total = 0;
        // Go on neighbours
        for neighbor in self.graph.neighbors(current) {
            total += self.count_paths(neighbor, destination, cache);
        }

        // Save on chach
        cache.insert(current, total);
        total
    }
}

/// Parse data from aoc format
struct DevicesParser;
impl DevicesParser {
    fn try_from_aoc_reader(reader: impl BufRead) -> Result<Devices> {
        let mut devices = Devices::new();
        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split(':').collect();

            let dest: Vec<&str> = parts[1].split_whitespace().collect();
            for d in dest {
                devices.update_edge(parts[0], d);
            }
        }
        Ok(devices)
    }
}

/// Solve part One
fn part_one(devices: &Devices) -> Option<usize> {
    let you = devices.get_node_index("you")?;
    let out = devices.get_node_index("out")?;
    let res = devices.count_paths(you, out, &mut HashMap::new());
    Some(res)
}

/// Solve part two
fn part_two(devices: &Devices) -> Option<usize> {
    let svr = devices.get_node_index("svr")?;
    let out = devices.get_node_index("out")?;
    let fft = devices.get_node_index("fft")?;
    let dac = devices.get_node_index("dac")?;

    let r1 = devices.count_paths(svr, fft, &mut HashMap::new());
    let r2 = devices.count_paths(fft, dac, &mut HashMap::new());
    let r3 = devices.count_paths(dac, out, &mut HashMap::new());
    let res_a = r1 * r2 * r3;

    let r4 = devices.count_paths(svr, dac, &mut HashMap::new());
    let r5 = devices.count_paths(dac, fft, &mut HashMap::new());
    let r6 = devices.count_paths(fft, out, &mut HashMap::new());
    let res_b = r4 * r5 * r6;

    Some(res_a + res_b)
}

pub fn solve(input_file: &Path) -> Result<(Option<String>, Option<String>)> {
    let reader = get_reader(input_file)?;
    let dev = DevicesParser::try_from_aoc_reader(reader)?;
    let res1 = part_one(&dev).map(|r| r.to_string());
    let res2 = part_two(&dev).map(|r| r.to_string());
    Ok((res1, res2))
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA_1: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

    const DATA_2: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

    #[test]
    fn test_part_one() {
        let dev = DevicesParser::try_from_aoc_reader(DATA_1.as_bytes()).unwrap();
        let res = part_one(&dev);
        assert_eq!(res, Some(5));
    }

    #[test]
    fn test_part_two() {
        let dev = DevicesParser::try_from_aoc_reader(DATA_2.as_bytes()).unwrap();
        let res = part_two(&dev);
        assert_eq!(res, Some(2));
    }
}
