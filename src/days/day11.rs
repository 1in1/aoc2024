use std::collections::HashMap;

use crate::solution;
pub struct Day11;

impl Day11 {
    fn blink(stone: u64) -> Vec<u64> {
        if stone == 0 {
            return vec![1];
        }

        let stone_str = stone.to_string();
        if stone_str.len() % 2 == 0 {
            return vec![
                stone_str[0..(stone_str.len() / 2)].parse::<u64>().unwrap(),
                stone_str[(stone_str.len() / 2)..stone_str.len()].parse::<u64>().unwrap(),
            ];
        }

        return vec![stone * 2024];
    }

    fn blink_count(stone: u64, remaining: u64, mut cache: &mut HashMap<(u64, u64), u64>) -> u64 {
        if remaining == 0 {
            return 1;
        }

        let cached = cache.get(&(stone, remaining));
        if let Some(&x) = cached {
            return x;
        }

        let val = Self::blink(stone)
            .iter()
            .map(|s| Self::blink_count(*s, remaining - 1, &mut cache))
            .sum::<u64>();
        cache.insert((stone, remaining), val);
        return val;
    }
}

impl solution::Solution for Day11 {
    fn solve_p2(&self, input: &str) -> String {
        input
            .split_whitespace()
            .map(|c| c.parse::<u64>().unwrap())
            .map(|d| Day11::blink_count(d, 75, &mut HashMap::new()))
            .sum::<u64>()
            .to_string()
    }

    fn solve_p1(&self, input: &str) -> String {
        input
            .split_whitespace()
            .map(|c| c.parse::<u64>().unwrap())
            .map(|d| Day11::blink_count(d, 25, &mut HashMap::new()))
            .sum::<u64>()
            .to_string()
    }
}
