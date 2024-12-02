use std::collections::HashMap;
use std::iter::zip;
use itertools::Itertools;

use crate::solution;

pub struct Day1;

impl solution::Solution for Day1 {
    fn solve_p2(&self, input: &str) -> String {
        let (left, right): (Vec<_>, HashMap<_, _>) = input
            .lines()
            .map(|line|
                line
                    .split_whitespace()
                    .map(|x| x.parse::<u32>().unwrap())
                    .next_tuple()
                    .unwrap()
            )
            .fold((Vec::new(), HashMap::new()), |(mut acc_v, mut acc_m), (l, r)| {
                acc_v.push(l);
                *(acc_m.entry(r).or_insert(0)) += 1;
                (acc_v, acc_m)
            });

        left
            .into_iter()
            .map(|l| l * right.get(&l).cloned().unwrap_or(0) )
            .sum::<u32>()
            .to_string()
    }

    fn solve_p1(&self, input: &str) -> String {
        let (mut left, mut right): (Vec<_>, Vec<_>) = input
            .lines()
            .map(|line|
                line
                    .split_whitespace()
                    .map(|x| x.parse::<u32>().unwrap())
                    .next_tuple()
                    .unwrap()
            )
            .unzip();

        left.sort();
        right.sort();

        zip(left, right)
            .map(|(l, r)| l.abs_diff(r))
            .sum::<u32>()
            .to_string()
    }
}
