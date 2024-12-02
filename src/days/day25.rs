use itertools::Itertools;

use crate::solution;
pub struct Day25;

impl solution::Solution for Day25 {
    fn solve_p2(&self, input: &str) -> String {
        String::new()
    }

    fn solve_p1(&self, input: &str) -> String {
        let (locks, keys): (Vec<_>, Vec<_>) = input
            .split("\n\n")
            .fold((Vec::new(), Vec::new()), |(mut locks, mut keys), block| {
                let arr = block
                    .lines()
                    .fold([0u32; 5], |mut acc, elt| {
                        elt
                            .lines()
                            .for_each(|row|
                                row
                                    .chars()
                                    .map(|c| (c == '#') as u32)
                                    .enumerate()
                                    .for_each(|(idx, c)| acc[idx] += c)
                            );
                        acc
                    });

                if block[0..5] == *"#####" {
                    locks.push(arr);
                } else {
                    keys.push(arr);
                }

                return (locks, keys);
            });

        locks
            .into_iter()
            .cartesian_product(keys.into_iter())
            .filter(|(lock, key)|
                lock
                    .into_iter()
                    .zip(key.into_iter())
                    .all(|(l_h, k_h)| l_h + k_h <= 7)
            )
            .count()
            .to_string()
    }
}
