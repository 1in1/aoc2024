use std::collections::HashSet;
use std::collections::BinaryHeap;

use std::cmp::Ordering;

use crate::solution;
use crate::days::day17;
pub struct Day18;

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    steps: u32,
    pos: (u32, u32),
    end_position: (u32, u32),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        let f_self = self.steps + Day18::h(self.pos, self.end_position);
        let f_other = other.steps + Day18::h(other.pos, other.end_position);
        f_other.cmp(&f_self)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Day18 {
    // Our heuristic
    fn h(
        p: (u32, u32),
        target: (u32, u32)
    ) -> u32 {
        p.1.abs_diff(target.1)
            + p.0.abs_diff(target.0)
    }

    fn next_steps(
        pos: (u32, u32),
        boundaries: (u32, u32),
        blocked: &HashSet<(u32, u32)>
    ) -> Vec<(u32, u32)> {
        vec![
            if pos.0 > 0 { Some((pos.0 - 1, pos.1)) } else { None },
            if pos.1 > 0 { Some((pos.0, pos.1 - 1)) } else { None },
            if pos.0 + 1 <= boundaries.0 { Some((pos.0 + 1, pos.1)) } else { None },
            if pos.1 + 1 <= boundaries.1 { Some((pos.0, pos.1 + 1)) } else { None },
        ]
            .iter()
            .flatten()
            .filter(|x| !blocked.contains(x))
            .map(|x| x.clone())
            .collect()
    }
}

impl solution::Solution for Day18 {
    // Can speed this up by binary searching
    fn solve_p2(&self, input: &str) -> String {
        let mut blocked = HashSet::new();

        let blocked_iter = input
            .lines()
            .map(|line| {
                let v = line
                    .split(",")
                    .map(|i| i.parse::<u32>().unwrap())
                    .collect::<Vec<_>>();
                (v[0], v[1])
            });



        'blocked_loop: for blocked_pos in blocked_iter {
            blocked.insert(blocked_pos);
            let mut heap = BinaryHeap::new();
            heap.push(State { steps: 0, pos: (0, 0), end_position: (70, 70) });
            let mut seen = HashSet::new();

            'walk_loop: while let Some(State { steps, pos, end_position }) = heap.pop() {
                if seen.contains(&pos) {
                    continue 'walk_loop;
                }

                seen.insert(pos);
                if pos == end_position {
                    continue 'blocked_loop;
                }

                let options = Self::next_steps(
                    pos,
                    end_position,
                    &blocked
                );

                for option in options {
                    heap.push(State { steps: steps + 1, pos: option, end_position: end_position });
                }
            }

            // If we are here, then we are blocked
            return blocked_pos.0.to_string() + "," + &blocked_pos.1.to_string();
        }

        String::new()
    }

    fn solve_p1(&self, input: &str) -> String {
        let blocked = input
            .lines()
            .take(1024)
            .map(|line| {
                let v = line
                    .split(",")
                    .map(|i| i.parse::<u32>().unwrap())
                    .collect::<Vec<_>>();
                (v[0], v[1])
            })
            .collect::<HashSet<_>>();

        let mut heap = BinaryHeap::new();
        heap.push(State { steps: 0, pos: (0, 0), end_position: (70, 70) });
        let mut seen = HashSet::new();

        while let Some(State { steps, pos, end_position }) = heap.pop() {
            if seen.contains(&(pos)) {
                continue;
            }

            seen.insert((pos));
            if pos == end_position {
                return steps.to_string();
            }

            let options = Self::next_steps(
                pos,
                end_position,
                &blocked
            );

            for option in options {
                heap.push(State { steps: steps + 1, pos: option, end_position: end_position });
            }
        }

        panic!();
    }
}
