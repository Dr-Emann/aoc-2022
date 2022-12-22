use ahash::{HashMap, HashMapExt, HashSet};
use bitvec::prelude::*;
use core::fmt;
use std::cmp;
use std::fmt::Write;
use std::ops::{Index, IndexMut};

type Idx = u8;
#[derive(Debug, Copy, Clone)]
pub struct Node {
    flow: u8,
}

#[derive(Default, Debug)]
pub struct Input {
    nodes: Vec<Node>,
    edges: HashSet<(Idx, Idx)>,
    distances: Vec<u8>,
    start: Idx,
}

impl Input {
    fn add_node(&mut self, node: Node) -> Idx {
        let idx = self.nodes.len().try_into().unwrap();
        self.nodes.push(node);
        idx
    }

    fn add_edge(&mut self, from: Idx, to: Idx) {
        let pair = if from < to { (from, to) } else { (to, from) };
        self.edges.insert(pair);
    }

    fn dist_between(&self, from: Idx, to: Idx) -> u8 {
        let (from, to) = if from < to { (from, to) } else { (to, from) };
        self.distances[usize::from(from) * self.nodes.len() + usize::from(to)]
    }

    fn floyd_warshall(&mut self) {
        assert!(self.distances.is_empty());
        let node_count = self.nodes.len();
        let node_idx_range = 0..u8::try_from(node_count).unwrap();

        let mut dist = vec![u8::MAX; node_count * node_count];
        let idx = |i, j| usize::from(i) * node_count + usize::from(j);

        for &(from, to) in &self.edges {
            dist[idx(from, to)] = 1;
            dist[idx(to, from)] = 1;
        }

        for node_idx in node_idx_range.clone() {
            dist[idx(node_idx, node_idx)] = 0;
        }

        for k in node_idx_range.clone() {
            for i in node_idx_range.clone() {
                for j in node_idx_range.clone() {
                    let piecewise_dist = dist[idx(i, k)].saturating_add(dist[idx(k, j)]);

                    if dist[idx(i, j)] > piecewise_dist {
                        dist[idx(i, j)] = piecewise_dist;
                    }
                }
            }
        }
        self.distances = dist;
    }
}

impl Index<Idx> for Input {
    type Output = Node;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.nodes[usize::from(index)]
    }
}

impl IndexMut<Idx> for Input {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.nodes[usize::from(index)]
    }
}

pub fn generator(s: &str) -> Input {
    let mut nodes = Vec::<(u8, ValveName, Vec<ValveName>)>::with_capacity(64);
    for line in s.lines() {
        let line = line.strip_prefix("Valve ").unwrap();
        let valve_name = ValveName::from_bytes(line.as_bytes());
        let line = &line[2..];
        let line = line.strip_prefix(" has flow rate=").unwrap();
        let (rate, line) = line
            .split_once("; tunnels lead to valves ")
            .or_else(|| line.split_once("; tunnel leads to valve "))
            .unwrap();
        let flow: u8 = rate.parse().unwrap();
        let links = line
            .split(", ")
            .map(|n| ValveName::from_bytes(n.as_bytes()));

        nodes.push((flow, valve_name, links.collect()));
    }
    nodes.sort_unstable_by_key(|n| cmp::Reverse(n.0));
    let mut input = Input::default();
    let mut node_idx_by_name = HashMap::with_capacity(64);
    for &(flow, name, _) in &nodes {
        node_idx_by_name.insert(name, input.add_node(Node { flow }));
    }
    for (i, (_flow, _name, links)) in nodes.iter().enumerate() {
        let idx = i as Idx;
        for link in links {
            input.add_edge(idx, node_idx_by_name[link]);
        }
    }
    input.start = node_idx_by_name[&ValveName(b'A', b'A')];

    input.floyd_warshall();

    input
}

type NodeSet = BitArr!(for 16);

struct QueueItem {
    time_remaining: u8,
    current: Idx,
    activated_links: NodeSet,
    released_pressure: u16,
}

fn pressure_releases<F>(input: &Input, total_time: u8, mut f: F)
where
    F: FnMut(NodeSet, u16),
{
    let working_nodes = input.nodes.partition_point(|n| n.flow > 0);
    let mut queue = Vec::with_capacity(256);
    assert!(working_nodes < NodeSet::ZERO.len());

    queue.push(QueueItem {
        time_remaining: total_time,
        current: input.start,
        activated_links: NodeSet::ZERO,
        released_pressure: 0,
    });

    while let Some(item) = queue.pop() {
        f(item.activated_links, item.released_pressure);
        for next_node in 0..working_nodes {
            if item.activated_links[next_node] {
                continue;
            }
            let next_idx = next_node as Idx;
            // One minute to open the valve
            let time_remaining = item
                .time_remaining
                .saturating_sub(input.dist_between(item.current, next_idx) + 1);
            if time_remaining == 0 {
                continue;
            }
            let released_pressure = item.released_pressure
                + u16::from(input[next_idx].flow) * u16::from(time_remaining);
            let mut activated_links = item.activated_links;
            activated_links.set(next_node, true);
            queue.push(QueueItem {
                time_remaining,
                current: next_idx,
                activated_links,
                released_pressure,
            });
        }
    }
}

pub fn part_1(input: &Input) -> u16 {
    let mut max_pressure = 0;
    pressure_releases(input, 30, |_, pressure| {
        max_pressure = max_pressure.max(pressure)
    });
    max_pressure
}

pub fn part_2(input: &Input) -> u16 {
    let mut max_pressures: HashMap<NodeSet, u16> = HashMap::with_capacity(4096);
    pressure_releases(input, 26, |nodes, pressure| {
        let dst = max_pressures.entry(nodes).or_default();
        *dst = (*dst).max(pressure);
    });

    let mut max_pressures: Vec<(NodeSet, u16)> = max_pressures.into_iter().collect();
    max_pressures.sort_unstable_by_key(|&(_, pressure)| cmp::Reverse(pressure));

    let mut max_pressure = 0;
    for (i, &(my_nodes, my_pressure)) in max_pressures.iter().enumerate() {
        for &(elephant_nodes, elephant_pressure) in &max_pressures[i + 1..] {
            // Pressures are sorted in decreasing pressures, if this can't get enough pressure to beat the max,
            // no further one will either
            if my_pressure + elephant_pressure < max_pressure {
                break;
            }
            if (my_nodes & elephant_nodes).not_any() {
                max_pressure = max_pressure.max(my_pressure + elephant_pressure);
            }
        }
    }
    max_pressure
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ValveName(u8, u8);

impl ValveName {
    fn from_bytes(bytes: &[u8]) -> Self {
        Self(bytes[0], bytes[1])
    }
}

impl fmt::Debug for ValveName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(self.0 as char)?;
        f.write_char(self.1 as char)?;
        Ok(())
    }
}

super::day_test! {demo_1 == 1651}
super::day_test! {demo_2 == 1707}
super::day_test! {part_1 == 1862}
super::day_test! {part_2 == 2422}
