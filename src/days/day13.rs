use itertools::Itertools;
use regex::Regex;

use crate::solution;
pub struct Day13;


impl Day13 {
    fn solve(
        a: (i64, i64),
        b: (i64, i64),
        p: (i64, i64)
    ) -> Option<i64> {
        // We have Ax = p, for
        // A = [  a.0, b.0
        //        a.1, b.1 ]
        // So we need to invert A in Z
        //
        // We get that
        // A' := [  b.1, -b.0
        //         -a.1,  a.0 ]
        // => A'Ax = (a.0 b.1 - a.1 b.0) x = A'p
        //
        // So long as the det != 0, we are good to invert this in Z and test
        let d = a.0 * b.1 - a.1 * b.0;
        if d == 0 { panic!(); }
        let aprime_p = (
             b.1 * p.0 - b.0 * p.1,
            -a.1 * p.0 + a.0 * p.1,
        );

        // Check if divisible
        if aprime_p.0 % d != 0 || aprime_p.1 % d != 0 {
            return None;
        }

        let x = (
            aprime_p.0 / d,
            aprime_p.1 / d,
        );
        return Some(3*x.0 + x.1);
    }
}

impl solution::Solution for Day13 {
    fn solve_p2(&self, input: &str) -> String {
        let re = Regex::new(r"(\d+)[^\d]+(\d+)").unwrap();
        input
            .lines()
            .chunks(4)
            .into_iter()
            .filter_map(|chunk| {
                let it = chunk
                    .take(3)
                    .map(|line| {
                        let captures = re.captures(line).unwrap();
                        (
                            captures[1].parse::<i64>().unwrap(),
                            captures[2].parse::<i64>().unwrap(),
                        )
                    })
                    .collect::<Vec<_>>();
                match &it[..] {
                    &[a, b, p, ..] => Self::solve(a, b, (p.0 + 10000000000000, p.1 + 10000000000000)),
                    _ => panic!(),
                }
            })
            .sum::<i64>()
            .to_string()
    }

    fn solve_p1(&self, input: &str) -> String {
        let re = Regex::new(r"(\d+)[^\d]+(\d+)").unwrap();
        input
            .lines()
            .chunks(4)
            .into_iter()
            .filter_map(|chunk| {
                let it = chunk
                    .take(3)
                    .map(|line| {
                        let captures = re.captures(line).unwrap();
                        (
                            captures[1].parse::<i64>().unwrap(),
                            captures[2].parse::<i64>().unwrap(),
                        )
                    })
                    .collect::<Vec<_>>();
                match &it[..] {
                    &[a, b, p, ..] => Self::solve(a, b, p),
                    _ => panic!(),
                }
            })
            .sum::<i64>()
            .to_string()
    }
}
