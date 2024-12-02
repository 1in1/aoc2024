use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;

use crate::solution;
pub struct Day23;

impl Day23 {
    fn largest_totally_connected<'life>(
        subgraph: &mut HashSet<&'life str>,
        available: &mut HashSet<&'life str>,
        edges: &HashMap<&'life str, HashSet<&'life str>>,
        cache: &mut HashMap<Vec<&'life str>, HashSet<&'life str>>,
    ) -> HashSet<&'life str> {
        // Use a sorted vec of the labels as the cache key, because `HashSet` is not `Hashable`
        let cache_key = {
            let mut tmp = subgraph.iter().map(|x| *x).collect::<Vec<_>>();
            tmp.sort();
            tmp
        };
        if let Some(cached) = cache.get(&cache_key) {
            return cached.clone();
        }

        let mut best: HashSet<&str> = subgraph.clone();
        // Collect `available` so that we can take a mutable borrow in the same scope
        for node in available.clone() {
            // Check connected to all of subgraph
            if !subgraph
                    .iter()
                    .all(|existing| edges
                        .get(existing)
                        .unwrap()
                        .contains(node)
                    ) {
                continue;
            }

            // Replace if adding this node improves the best
            subgraph.insert(node);
            available.remove(node);
            let largest_including_node = Self::largest_totally_connected(
                subgraph,
                available,
                edges,
                cache
            );
            if largest_including_node.len() > best.len() {
                best = largest_including_node;
            }
            subgraph.remove(node);
            available.insert(node);
        }

        cache.insert(cache_key, best.clone());
        return best;
    }
}

impl solution::Solution for Day23 {
    fn solve_p2(&self, input: &str) -> String {
        let (neighbours, mut nodes) = input
            .lines()
            .map(|line| 
                line
                    .split("-")
                    .tuple_windows()
                    .next()
                    .unwrap()
            )
            .fold((HashMap::new(), HashSet::new()), |(mut edge_acc, mut node_acc), (lhs, rhs)| {
                // Add edges in both directions
                (*(edge_acc.entry(lhs).or_insert(HashSet::new()))).insert(rhs);
                (*(edge_acc.entry(rhs).or_insert(HashSet::new()))).insert(lhs);
                // Record both nodes
                node_acc.insert(lhs);
                node_acc.insert(rhs);
                (edge_acc, node_acc)
            });

        let best_subgraph = Self::largest_totally_connected(
            &mut HashSet::new(),
            &mut nodes,
            &neighbours,
            &mut HashMap::new()
        );
        dbg!(&best_subgraph);

        return best_subgraph.len().to_string();
    }

    fn solve_p1(&self, input: &str) -> String {
        let (neighbours, nodes) = input
            .lines()
            .map(|line| 
                line
                    .split("-")
                    .tuple_windows()
                    .next()
                    .unwrap()
            )
            .fold((HashMap::new(), HashSet::new()), |(mut edge_acc, mut node_acc), (lhs, rhs)| {
                // Add edges in both directions
                (*(edge_acc.entry(lhs).or_insert(HashSet::new()))).insert(rhs);
                (*(edge_acc.entry(rhs).or_insert(HashSet::new()))).insert(lhs);
                // Record both nodes
                node_acc.insert(lhs);
                node_acc.insert(rhs);
                (edge_acc, node_acc)
            });

        // Find the 3-cycles
        // For each node
        // Walk from this node until reaching own node
        let three_cycles = nodes
            .iter()
            .flat_map(|a|
                neighbours
                    .get(a)
                    .unwrap()
                    .iter()
                    .flat_map(|b|
                        neighbours
                            .get(b)
                            .unwrap()
                            .iter()
                            .filter(|c|
                                neighbours
                                    .get(*c)
                                    .unwrap()
                                    .contains(a)
                            )
                            .map(|c| {
                                let mut tmp = [a.clone(), b.clone(), c.clone()]
                                    .into_iter()
                                    .collect::<Vec<_>>();
                                tmp.sort();
                                return tmp;
                            })
                            .filter(|group| group.iter().any(|node| node.chars().nth(0).unwrap() == 't'))
                    )
            )
            .collect::<HashSet<_>>();
        return three_cycles.len().to_string();
    }
}
