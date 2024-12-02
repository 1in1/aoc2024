use std::collections::HashMap;
use std::collections::HashSet;

use gcd::Gcd;

use crate::solution;
pub struct Day8;

impl Day8 {
    fn antinodes_between_2(
        a: (usize, usize),
        b: (usize, usize),
        bounds: (usize, usize)
    ) -> HashSet<(usize, usize)> {
        let mut spots = HashSet::new();

        let b_minus_a = (b.0 as i32 - a.0 as i32, b.1 as i32 - a.1 as i32);

        let gcd = (b_minus_a.0.abs() as u32).gcd(b_minus_a.1.abs() as u32) as i32;

        let short_delta = (b_minus_a.0 / gcd, b_minus_a.1 / gcd);

        for vec in [short_delta, (-short_delta.0, -short_delta.1)] {
            let (mut curr_row, mut curr_col) = (b.0 as i32, b.1 as i32);
            while curr_row >= 0 && curr_row < (bounds.0 as i32) && curr_col >= 0 && curr_col < (bounds.1 as i32) {
                spots.insert((curr_row as usize, curr_col as usize));
                curr_row += vec.0;
                curr_col += vec.1;
            }
        }
        return spots;
    }

    fn antinodes_between(
        a: (usize, usize),
        b: (usize, usize),
        bounds: (usize, usize)
    ) -> HashSet<(usize, usize)> {
        let mut spots = HashSet::new();

        let b_minus_2a = ((2*b.0).checked_sub(a.0), (2*b.1).checked_sub(a.1));
        let a_minus_2b = ((2*a.0).checked_sub(b.0), (2*a.1).checked_sub(b.1));

        for entry in [b_minus_2a, a_minus_2b] {
            match entry {
                (Some(row), Some(col)) => {
                    if row < bounds.0 && col < bounds.1 {
                        spots.insert((row, col));
                    }
                },
                _ => continue,
            }
        }

        return spots;
    }
}

impl solution::Solution for Day8 {
    fn solve_p2(&self, input: &str) -> String {
        let row_count = input.lines().count();
        let col_count = input.lines().next().unwrap().len();
        let mut ants = HashMap::new();
        input
            .lines()
            .enumerate()
            .for_each(|(row_idx, row)|
                 row
                    .chars()
                    .enumerate()
                    .for_each(|(col_idx, c)| {
                        if c != '.' {
                            ants
                                .entry(c)
                                .or_insert(Vec::new())
                                .push((row_idx, col_idx));
                        }
                    })
            );

        let mut all_antinodes = HashSet::new();
        for (_, vals) in ants.iter() {
            for i in 0..vals.len() {
                for j in (i+1)..vals.len() {
                    for an in Day8::antinodes_between_2(
                        vals[i],
                        vals[j],
                        (row_count, col_count)
                    ) {
                        all_antinodes.insert(an);
                    }
                }
            }
        }

        return all_antinodes.len().to_string();
    }

    fn solve_p1(&self, input: &str) -> String {
        let row_count = input.lines().count();
        let col_count = input.lines().next().unwrap().len();
        let mut ants = HashMap::new();
        input
            .lines()
            .enumerate()
            .for_each(|(row_idx, row)|
                 row
                    .chars()
                    .enumerate()
                    .for_each(|(col_idx, c)| {
                        if c != '.' {
                            ants
                                .entry(c)
                                .or_insert(Vec::new())
                                .push((row_idx, col_idx));
                        }
                    })
            );

        let mut all_antinodes = HashSet::new();
        for (_, vals) in ants.iter() {
            for i in 0..vals.len() {
                for j in (i+1)..vals.len() {
                    for an in Day8::antinodes_between(
                        vals[i],
                        vals[j],
                        (row_count, col_count)
                    ) {
                        all_antinodes.insert(an);
                    }
                }
            }
        }

        return all_antinodes.len().to_string();
    }
}
