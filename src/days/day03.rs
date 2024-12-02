use regex::Regex;

use crate::solution;
pub struct Day3;


impl solution::Solution for Day3 {
    fn solve_p1(&self, input: &str) -> String {
        Regex::new(r"mul\((\d+),(\d+)\)").unwrap()
            .captures_iter(input)
            .map(|c| c.extract())
            .map(|(_, [fst, snd])| fst.parse::<u32>().unwrap() * snd.parse::<u32>().unwrap())
            .sum::<u32>()
            .to_string()
    }

    fn solve_p2(&self, input: &str) -> String {
        Regex::new(r"mul\((\d+),(\d+)\)|(do)()\(\)|(don't)()\(\)").unwrap()
            .captures_iter(input)
            .map(|c| c.extract())
            .fold((0u32, true), |(acc_sum, acc_enabled), (_, [fst, snd])| {
                if fst == "do" { return (acc_sum, true); }
                if fst == "don't" { return (acc_sum, false); }

                if acc_enabled {
                    return (
                        acc_sum + (fst.parse::<u32>().unwrap() * snd.parse::<u32>().unwrap()),
                        acc_enabled
                    );
                } else {
                    return (acc_sum, acc_enabled);
                }
            })
            .0
            .to_string()
    }
}
