use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::solution;
pub struct Day16;


#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    // A position is a point on the graph plus a facing direction
    position: ((usize, usize), char),
    // We also need to carry around the end position to compute the objective
    end_position: (usize, usize),
    // For part 2, need the path taken
    path: Vec<((usize, usize), char)>,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        let f_self = self.cost + Day16::h(self.position.0, self.end_position);
        let f_other = other.cost + Day16::h(other.position.0, other.end_position);
        f_other.cmp(&f_self)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Day16 {
    // Our heuristic
    fn h(
        p: (usize, usize),
        target: (usize, usize)
    ) -> usize {
        if p.0 == target.0 {
            p.1.abs_diff(target.1)
        }

        else if p.1 == target.1 {
            p.0.abs_diff(target.0)
        }

        else {
            p.1.abs_diff(target.1)
                + p.0.abs_diff(target.0)
                + 1000
        }
    }

    fn next_dir(dir: char) -> char {
        match dir {
            '>' => 'v',
            'v' => '<',
            '<' => '^',
            '^' => '>',
            _ => panic!(),
        }
    }

    fn prev_dir(dir: char) -> char {
        match dir {
            '>' => '^',
            '^' => '<',
            '<' => 'v',
            'v' => '>',
            _ => panic!(),
        }
    }

    fn update_pos(
        pos: (usize, usize),
        dir: char
    ) -> (usize, usize) {
        match dir {
            '^' => (pos.0 - 1, pos.1),
            '>' => (pos.0, pos.1 + 1),
            'v' => (pos.0 + 1, pos.1),
            '<' => (pos.0, pos.1 - 1),
            _   => panic!(),
        }
    }
}

impl solution::Solution for Day16 {
    fn solve_p2(&self, input: &str) -> String {
        let map = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let start = map
            .iter()
            .enumerate()
            .filter_map(|(row_idx, row)|
                row
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| **c == 'S')
                    .map(|(col_idx, _)| (row_idx, col_idx))
                    .next()
            )
            .next()
            .unwrap();
        let end = map
            .iter()
            .enumerate()
            .filter_map(|(row_idx, row)|
                row
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| **c == 'E')
                    .map(|(col_idx, _)| (row_idx, col_idx))
                    .next()
            )
            .next()
            .unwrap();

        let mut heap = BinaryHeap::new();
        let mut cheapest_route = HashMap::new();

        heap.push(State { cost: 0, position: (start, '>'), end_position: end, path: Vec::new() });
        cheapest_route.insert((start, '>'), 0);

        let mut on_a_cheap_path: HashSet<(usize, usize)> = HashSet::new();
        let mut min_cost: Option<usize> = None;
        while let Some(State { cost, position, end_position, mut path }) = heap.pop() {
            if position.0 == end_position {
                if let Some(real_min) = min_cost {
                    if real_min < cost { continue; }
                }

                for pos in path.iter() {
                    on_a_cheap_path.insert(pos.0);
                }
                min_cost = Some(cost);
                continue;
            }
            // We may want to keep track of the cheapest way to each state
            if let Some(prev_cost) = cheapest_route.get(&position) {
                if *prev_cost < cost { continue; }
            }
            cheapest_route.insert(position, cost);

            path.push(position);

            // Add in all the others
            // Changes of direction
            heap.push(State {
                cost: cost + 1000,
                position: (position.0, Day16::next_dir(position.1)),
                end_position: end_position,
                path: path.clone()
            });
            heap.push(State {
                cost: cost + 1000,
                position: (position.0, Day16::prev_dir(position.1)),
                end_position: end_position,
                path: path.clone()
            });
            // Walking forwards
            let fwd = Day16::update_pos(position.0, position.1);
            if map[fwd.0][fwd.1] != '#' {
                heap.push(State {
                    cost: cost + 1,
                    position: (Day16::update_pos(position.0, position.1), position.1),
                    end_position: end_position,
                    path: path.clone()
                });
            }
        }

        // +1 to account for the end state
        (on_a_cheap_path.len() + 1).to_string()
    }

    fn solve_p1(&self, input: &str) -> String {
        let map = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let start = map
            .iter()
            .enumerate()
            .filter_map(|(row_idx, row)|
                row
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| **c == 'S')
                    .map(|(col_idx, _)| (row_idx, col_idx))
                    .next()
            )
            .next()
            .unwrap();
        let end = map
            .iter()
            .enumerate()
            .filter_map(|(row_idx, row)|
                row
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| **c == 'E')
                    .map(|(col_idx, _)| (row_idx, col_idx))
                    .next()
            )
            .next()
            .unwrap();

        let mut heap = BinaryHeap::new();
        let mut cheapest_route = HashMap::new();

        heap.push(State { cost: 0, position: (start, '>'), end_position: end, path: Vec::new() });
        cheapest_route.insert((start, '>'), 0);

        while let Some(State { cost, position, end_position, .. }) = heap.pop() {
            if position.0 == end_position {
                return cost.to_string();
            }
            // We may want to keep track of the cheapest way to each state
            if let Some(prev_cost) = cheapest_route.get(&position) {
                if *prev_cost < cost { continue; }
            }
            cheapest_route.insert(position, cost);

            // Add in all the others
            // Changes of direction
            heap.push(State {
                cost: cost + 1000,
                position: (position.0, Day16::next_dir(position.1)),
                end_position: end_position,
                path: Vec::new()
            });
            heap.push(State {
                cost: cost + 1000,
                position: (position.0, Day16::prev_dir(position.1)),
                end_position: end_position,
                path: Vec::new()
            });
            // Walking forwards
            let fwd = Day16::update_pos(position.0, position.1);
            if map[fwd.0][fwd.1] != '#' {
                heap.push(State {
                    cost: cost + 1,
                    position: (Day16::update_pos(position.0, position.1), position.1),
                    end_position: end_position,
                    path: Vec::new()
                });
            }
        }

        panic!();
    }
}
