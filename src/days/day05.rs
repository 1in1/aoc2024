use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::Ordering;

use itertools::Itertools;

use crate::solution;
pub struct Day5;

impl solution::Solution for Day5 {
    fn solve_p2(&self, input: &str) -> String {
        let mut lines = input.lines();

        let must_not_follow: HashMap<u32, HashSet<u32>> = lines
            .by_ref() // Do it all in a single scan!
            .take_while(|line| !(line.is_empty()))
            .map(|line|
                line
                    .split("|")
                    .map(|x| x.parse::<u32>().unwrap())
                    .next_tuple()
                    .unwrap()
            )
            .fold(HashMap::new(), |mut acc, (u, v)| {
                (*(acc.entry(u).or_insert(HashSet::new()))).insert(v);
                acc
            });

        lines
            .map(|line| {
                let mut elts: Vec<u32> = line
                    .split(",")
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect();
                let mut was_out_of_order = false;
                elts.sort_by(|a, b| {
                    // It would be much quicker to do a real topological sort
                    // But this is a strong lazy option
                    let cannot_follow_a = must_not_follow
                        .get(a)
                        .unwrap();
                    if cannot_follow_a.contains(b) {
                        was_out_of_order = true;
                        return Ordering::Less;
                    } else {
                        return Ordering::Greater;
                    }
                });

                if was_out_of_order {
                    return elts[elts.len() / 2];
                } else {
                    return 0;
                }
            })
            .sum::<u32>()
            .to_string()
    }

    fn solve_p1(&self, input: &str) -> String {
        let mut lines = input.lines();

        let must_not_follow: HashMap<u32, HashSet<u32>> = lines
            .by_ref() // Do it all in a single scan!
            .take_while(|line| !(line.is_empty()))
            .map(|line|
                line
                    .split("|")
                    .map(|x| x.parse::<u32>().unwrap())
                    .next_tuple()
                    .unwrap()
            )
            .fold(HashMap::new(), |mut acc, (u, v)| {
                (*(acc.entry(u).or_insert(HashSet::new()))).insert(v);
                acc
            });

        lines
            .map(|line| {
                let elts: Vec<u32> = line
                    .split(",")
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect();
/*
                elts
                    .iter()
                    .scan(HashSet::new(), |acc, elt| {
                        if !(*acc).is_disjoint(&must_not_follow.get(elt).unwrap()) {
                            return None;
                        }
                        *acc.insert(*elt);
                        return Some(0)
                    })
                    .next()
                    .map(|_| 0)
                    .unwrap_or(elts[elts.len() / 2])
*/
                let mut seen = HashSet::new();
                for a in elts.iter() {
                    let cannot_follow_a = must_not_follow
                        .get(a)
                        .unwrap();
                    if !seen.is_disjoint(&cannot_follow_a) {
                        return 0;
                    }
                    seen.insert(*a);
                }

                return elts[elts.len() / 2];
            })
            .sum::<u32>()
            .to_string()
    }
}
