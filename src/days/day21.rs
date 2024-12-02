use itertools::Itertools;
use std::iter::{once, repeat};
use std::cell::LazyCell;
use std::collections::HashMap;

use crate::solution;
pub struct Day21;

const NUMERIC_KEYPAD: LazyCell<HashMap<char, (isize, isize)>> = LazyCell::new(|| {
    ["789", "456", "123", "#0A"]
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row
                .chars()
                .enumerate()
                .map(move |(col_idx, c)| (c, (row_idx as isize, col_idx as isize)))
        })
        .collect()
});

const DIRECTIONAL_KEYPAD: LazyCell<HashMap<char, (isize, isize)>> = LazyCell::new(|| {
    ["#^A", "<v>"]
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row
                .chars()
                .enumerate()
                .map(move |(col_idx, c)| (c, (row_idx as isize, col_idx as isize)))
        })
        .collect()
});

impl Day21 {
    fn sequence_cost<KeyIter>(
        seq: KeyIter,
        layer: usize,
        costs: &HashMap<(usize, char, char), usize>,
    ) -> usize where KeyIter: Iterator<Item=char> {
        once('A')
            .chain(seq)
            .tuple_windows()
            .map(|(from, to)|
                match layer {
                    // The user can just press the button
                    0 => 1,
                    // Hit the cached cost
                    _ => *costs.get(&(layer, from, to)).unwrap()
                }
            )
            .sum()
    }

    fn precompute_single_layer(
        costs: &mut HashMap<(usize, char, char), usize>,
        layer: usize,
        keypad: &HashMap<char, (isize, isize)>,
    ) {
        keypad
            .iter()
            .cartesian_product(keypad.iter())
            .for_each(|( (&key_from, &(col_from, row_from)), (&key_to, &(col_to, row_to)) )| {
                // It is always cheaper to do <all horizontal> then <all vertical>, OR
                // <all vertical> then <all horizontal> moves
                //
                // Then we need to press A
                //
                // So the cost of this layer is the lesser of these two options
                let row_walk = repeat(if row_to > row_from { '>' } else { '<' })
                    .take(row_to.abs_diff(row_from));
                let col_walk = repeat(if col_to < col_from { '^' } else { 'v' })
                    .take(col_to.abs_diff(col_from));

                let min_row_then_col = if (col_from, row_to) != keypad[&'#'] {
                    Self::sequence_cost(
                        row_walk
                            .clone()
                            .chain(col_walk.clone())
                            .chain(once('A')),
                        layer - 1,
                        costs,
                    )
                } else {
                    //dbg!(((row_to, col_from), (row_from, col_to)));
                    // We would walk off the keypad - don't do this
                    usize::MAX
                };

                let min_col_then_row = if (col_to, row_from) != keypad[&'#'] {
                    Self::sequence_cost(
                        col_walk
                            .chain(row_walk)
                            .chain(once('A')),
                        layer - 1,
                        costs,
                    )
                } else {
                    //dbg!(((row_to, col_from), (row_from, col_to)));
                    usize::MAX
                };

                costs.insert(
                    (layer, key_from, key_to),
                    min_row_then_col.min(min_col_then_row),
                );
            })
    }

    fn precompute_costs(n: usize) -> HashMap<(usize, char, char), usize> {
        let mut costs: HashMap<(usize, char, char), usize> = HashMap::new();

        for i in (0..n) {
            Self::precompute_single_layer(&mut costs, i+1, &DIRECTIONAL_KEYPAD);
        }
        Self::precompute_single_layer(&mut costs, n+1, &NUMERIC_KEYPAD);

        return costs;
    }
}

impl solution::Solution for Day21 {
    fn solve_p2(&self, input: &str) -> String {
        let costs = Self::precompute_costs(25);
        input
            .lines()
            .map(|line|
                Self::sequence_cost(line.chars(), 26, &costs)
                    * line[0..line.len() - 1].parse::<usize>().unwrap()
            )
            .sum::<usize>()
            .to_string()
    }

    fn solve_p1(&self, input: &str) -> String {
        let costs = Self::precompute_costs(2);
        input
            .lines()
            .map(|line|
                Self::sequence_cost(line.chars(), 3, &costs)
                    * line[0..line.len() - 1].parse::<usize>().unwrap()
            )
            .sum::<usize>()
            .to_string()
    }
}
