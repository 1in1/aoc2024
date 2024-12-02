use std::ops;

use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;

use crate::solution;
pub struct Day22;

/*
 * 
    Calculate the result of multiplying the secret number by 64. Then, mix this result into the secret number. Finally, prune the secret number.
    Calculate the result of dividing the secret number by 32. Round the result down to the nearest integer. Then, mix this result into the secret number. Finally, prune the secret number.
    Calculate the result of multiplying the secret number by 2048. Then, mix this result into the secret number. Finally, prune the secret number.

Each step of the above process involves mixing and pruning:

    To mix a value into the secret number, calculate the bitwise XOR of the given value and the secret number. Then, the secret number becomes the result of that operation. (If the secret number is 42 and you were to mix 15 into the secret number, the secret number would become 37.)
    To prune the secret number, calculate the value of the secret number modulo 16777216. Then, the secret number becomes the result of that operation. (If the secret number is 100000000 and you were to prune the secret number, the secret number would become 16113920.)

    */

impl Day22 {
    fn evolve(x: u64) -> u64 {
        let step_1 = (x ^ (x << 6)) & ((1 << 24) - 1);
        let step_2 = (step_1 ^ (step_1 >> 5)) & ((1 << 24) - 1);
        let step_3 = (step_2 ^ (step_2 << 11)) & ((1 << 24) - 1);
        return step_3;
    }
}

impl solution::Solution for Day22 {
    fn solve_p2(&self, input: &str) -> String {
        // For each buyer, generate a hashmap from diff tuples to gains
        input
            .lines()
            .map(|x| x.parse::<u64>().unwrap())
            .map(|x|
                (0..2000)
                    .scan(x, |acc, _| {
                        let ret = *acc;
                        *acc = Self::evolve(*acc);
                        Some(ret)
                    })
                    .map(|x| (x % 10) as i32)
                    .tuple_windows()
                    .map(|(fst, snd)| (snd, snd - fst))
                    .tuple_windows()
                    .fold(HashMap::new(), |mut acc, (a, b, c, d)| {
                        // Store the tuples for just buyer and its associated gain
                        let diffs = (a.1, b.1, c.1, d.1);
                        if acc.contains_key(&diffs) {
                            return acc;
                        }
                        acc.insert(diffs, d.0);
                        acc
                    })
            )
            .fold(HashMap::new(), |mut acc, elt| {
                for (k, v) in elt.iter() {
                    *(acc.entry(*k).or_insert(0)) += v;
                }
                acc
            })
            .into_values()
            .max()
            .unwrap()
            .to_string()
    }

    fn solve_p1(&self, input: &str) -> String {
        input
            .lines()
            .map(|x| x.parse::<u64>().unwrap())
            .map(|x|
                (0..2000)
                    .fold(x, |acc, _|
                        Self::evolve(acc)
                    )
            )
            .sum::<u64>()
            .to_string()
    }
}
