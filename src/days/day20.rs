use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use crate::solution;
pub struct Day20;

#[derive(Copy, Clone, Eq, Debug, PartialEq, Hash)]
struct CheatId {
    start: (usize, usize),
    end: (usize, usize),
}

struct WalkIterator<'life> {
    curr: Option<(usize, usize)>,
    prev: Option<(usize, usize)>,
    map: &'life Vec<Vec<char>>,
    end: (usize, usize),
}

impl Iterator for WalkIterator<'_> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr?;

        let next_loc = Day20::neighbours(current, (self.map.len(), self.map[0].len()))
            .filter(|next_loc| self.map[next_loc.0][next_loc.1] != '#')
            .filter(|next_loc| Some(*next_loc) != self.prev)
            .next();
        self.prev = self.curr;
        self.curr = next_loc;

        return Some(current);
    }
}

impl Day20 {
    fn neighbours(
        loc: (usize, usize),
        boundaries: (usize, usize),
    ) -> impl Iterator<Item=(usize, usize)> {
        [
            if loc.0 > 0 { Some((loc.0 - 1, loc.1)) }
            else { None },
            if loc.1 > 0 { Some((loc.0, loc.1 - 1)) }
            else { None },
            if loc.0 + 1 < boundaries.0 { Some((loc.0 + 1, loc.1)) }
            else { None },
            if loc.1 + 1 < boundaries.1 { Some((loc.0, loc.1 + 1)) }
            else { None },
        ]
            .into_iter()
            .flatten()
    }

    fn two_step_neighbours(
        loc: (usize, usize),
        boundaries: (usize, usize),
    ) -> impl Iterator<Item=(usize, usize)> {
        [
            if loc.0 > 1 { Some((loc.0 - 2, loc.1)) }
            else { None },
            if loc.1 > 1 { Some((loc.0, loc.1 - 2)) }
            else { None },
            if loc.0 + 2 < boundaries.0 { Some((loc.0 + 2, loc.1)) }
            else { None },
            if loc.1 + 2 < boundaries.1 { Some((loc.0, loc.1 + 2)) }
            else { None },
        ]
            .into_iter()
            .flatten()
    }

    fn reachable_in_n_steps(
        loc: (usize, usize),
        map: &Vec<Vec<char>>,
        n: i64,
    ) -> impl Iterator<Item=(usize, usize)> + use<'_> {
        // If we need to only pass through walls once, then modify this to bfs
        ((-20i64)..(21i64))
            .flat_map(move |row_shift|
                ((-20i64)..(21i64))
                    .filter(move |col_shift| row_shift.clone().abs() + col_shift.abs() <= n)
                    .map(move |col_shift| (loc.0 as i64 + row_shift.clone(), loc.1 as i64 + col_shift))
                    .filter(|(row, col)|
                        *row >= 0 && ((*row as usize) < map.len()) &&
                        *col >= 0 && ((*col as usize) < map[0].len())
                    )
                    .map(|(row, col)| (row as usize, col as usize))
            )
    }

    fn taxicab(
        a: (usize, usize),
        b: (usize, usize),
    ) -> usize {
        a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
    }
}

impl solution::Solution for Day20 {
    fn solve_p2(&self, input: &str) -> String {
        let map = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let start_pos = map
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
        let end_pos = map
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

        let walk_iter = WalkIterator {
            curr: Some(start_pos),
            prev: None,
            map: &map,
            end: end_pos,
        };

        let mut walked = HashMap::<(usize, usize), usize>::new();
        let mut cheats = Vec::new();
        for (idx, loc) in walk_iter.enumerate() {
            Day20::reachable_in_n_steps(loc, &map, 20)
                .filter_map(|reachable| walked.get(&reachable).map(|t| (t, reachable)))
                .for_each(|(old_idx, reachable)| cheats.push((reachable, loc, idx - old_idx - Self::taxicab(reachable, loc))));
            walked.insert(loc, idx);
        }

        cheats
            .iter()
            .map(|(_, _, saved)| saved)
            .filter(|saved| **saved >= 100)
            .count()
            .to_string()
    }

    fn solve_p1(&self, input: &str) -> String {
        let map = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let start_pos = map
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
        let end_pos = map
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

        let walk_iter = WalkIterator {
            curr: Some(start_pos),
            prev: None,
            map: &map,
            end: end_pos,
        };

        let mut walked = HashMap::<(usize, usize), usize>::new();
        let mut cheats = Vec::new();
        for (idx, loc) in walk_iter.enumerate() {
            Day20::two_step_neighbours(loc, (map.len(), map[0].len()))
                .filter_map(|reachable| walked.get(&reachable).map(|t| (t, reachable)))
                .for_each(|(old_idx, reachable)| cheats.push((reachable, loc, idx - old_idx - 2)));
            walked.insert(loc, idx);
        }

        cheats
            .iter()
            .map(|(_, _, saved)| saved)
            .filter(|saved| **saved >= 20)
            .count()
            .to_string()
    }
}
