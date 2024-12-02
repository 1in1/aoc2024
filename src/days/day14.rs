use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::solution;
pub struct Day14;

impl Day14 {
    fn move_pos(
        init: (usize, usize),
        vel: (i64, i64),
        dims: (usize, usize),
        t: i64
    ) -> (usize, usize) {
        (
            (init.0 as i64 + t * vel.0).rem_euclid(dims.0 as i64) as usize,
            (init.1 as i64 + t * vel.1).rem_euclid(dims.1 as i64) as usize,
        )
    }

    // 0 is top-left, going clockwise to 3
    fn quadrant(
        loc: (usize, usize),
        dims: (usize, usize)
    ) -> Option<u32> {
        // dims is assumed to be odd in both coords
        if loc.0 == dims.0 / 2 || loc.1 == dims.1 / 2 {
            return None;
        }

        Some(match (loc.0 < dims.0 / 2, loc.1 < dims.1 / 2) {
            (true, true) => 0,
            (false, true) => 1,
            (false, false) => 2,
            (true, false) => 3,
        })
    }
}

impl solution::Solution for Day14 {
    fn solve_p2(&self, input: &str) -> String {
        let re = Regex::new(r"p=(\d+),(\d+) v=([\d\-]+),([\d\-]+)").unwrap();
        //let (rows, cols) = (11usize, 7usize);
        let (rows, cols) = (101usize, 103usize);
        let robots = input
            .lines()
            .map(|line| {
                let captures = re.captures(line).unwrap();
                (
                    (
                        captures[1].parse::<usize>().unwrap(),
                        captures[2].parse::<usize>().unwrap(),
                    ),
                    (
                        captures[3].parse::<i64>().unwrap(),
                        captures[4].parse::<i64>().unwrap(),
                    ),
                    (
                        rows,
                        cols,
                    )
                )
            })
            .collect::<Vec<_>>();

        let mut u = (0..10000)
            .map(|t|
                robots
                    .iter()
                    .map(|(init, vel, dims)| Self::move_pos(*init, *vel, *dims, t))
                    .fold(HashSet::new(), |mut acc, elt| {
                        acc.insert(elt);
                        acc
                    })
                    .into_iter()
                    .filter_map(|final_pos| Self::quadrant(final_pos, (rows, cols)))
                    .fold(HashMap::new(), |mut acc, elt| {
                        let curr = acc.entry(elt).or_insert(0);
                        *curr += 1;
                        acc
                    })
                    .into_values()
                    .product::<u32>()
            )
            .enumerate()
            .collect::<Vec<_>>();
        u.sort_by(|(_, x), (_, y)| x.cmp(y));
        dbg!(u.iter().take(10).collect::<Vec<_>>());

        for t in u.iter().take(4).map(|(idx, _)| *idx as i64) {
            println!("{}", t);
            // get all positions at time
            let positions_at_time: HashSet<(usize, usize)> = robots
                .iter()
                .map(|(init, vel, dims)| Self::move_pos(*init, *vel, *dims, t))
                .fold(HashSet::new(), |mut acc, elt| {
                    acc.insert(elt);
                    acc
                });
            let string = (0..rows)
                .map(|y|
                    (0..cols)
                        .map(|x|
                            if positions_at_time.contains(&(x, y)) {
                                '*'
                            } else {
                                ' '
                            }
                        )
                        .collect::<String>()
                );
            for line in string {
                println!("{}", line);
            }
            println!("\n\n");
        }
        String::new()
    }

    fn solve_p1(&self, input: &str) -> String {
        let re = Regex::new(r"p=(\d+),(\d+) v=([\d\-]+),([\d\-]+)").unwrap();
        //let (rows, cols) = (11usize, 7usize);
        let (rows, cols) = (101usize, 103usize);
        input
            .lines()
            .map(|line| {
                let captures = re.captures(line).unwrap();
                Self::move_pos(
                    (
                        captures[1].parse::<usize>().unwrap(),
                        captures[2].parse::<usize>().unwrap(),
                    ),
                    (
                        captures[3].parse::<i64>().unwrap(),
                        captures[4].parse::<i64>().unwrap(),
                    ),
                    (
                        rows,
                        cols,
                    ),
                    100
                )
            })
            .filter_map(|final_pos| Self::quadrant(final_pos, (rows, cols)))
            .fold(HashMap::new(), |mut acc, elt| {
                let curr = acc.entry(elt).or_insert(0);
                *curr += 1;
                acc
            })
            .into_values()
            .product::<u32>()
            .to_string()

    }
}
