use itertools::Itertools;

use crate::solution;
pub struct Day2;

impl solution::Solution for Day2 {
    fn solve_p1(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| 
                line
                    .split_whitespace()
                    .map(|d| d.parse::<i32>().unwrap())
                    .into_iter()
                    .tuple_windows()
                    .map(|(a, b)| 
                        a - b
                    )
            )
            .filter(|diffs|
                diffs.clone().all(|diff| diff >= 1 && diff <= 3) ||
                diffs.clone().all(|diff| diff <= -1 && diff >= -3)
            )
            .count()
            .to_string()
    }

    fn solve_p2(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| 
                line
                    .split_whitespace()
                    .map(|d| d.parse::<i32>().unwrap())
            )
            .filter(|vals|
                vals.clone()
                    .enumerate()
                    .map(|(i, _)|
                        vals
                            .clone()
                            .enumerate()
                            .filter(move |&(j, _)| i != j)
                            .map(|(_, v)| v)
                            .into_iter()
                            .tuple_windows()
                            .map(|(a, b)| 
                                 a - b
                            )
                    )
                    .any(|sublist_diffs|
                        sublist_diffs.clone().all(|diff| diff >= 1 && diff <= 3) ||
                        sublist_diffs.clone().all(|diff| diff <= -1 && diff >= -3)
                    )
            )
            .count()
            .to_string()
    }
}
