use crate::solution;
pub struct Day7;

impl Day7 {
    fn can_work(numbers: &Vec<u64>,
                target_val: u64,
                curr_val: u64,
                curr_idx: usize) -> bool {
        if curr_idx >= numbers.len() {
            // we are at the end
            return target_val == curr_val;
        }

        return Self::can_work(numbers,
                              target_val,
                              curr_val + numbers[curr_idx],
                              curr_idx + 1) ||
            Self::can_work(numbers,
                           target_val,
                           curr_val * numbers[curr_idx],
                           curr_idx + 1);
    }

    fn can_work_2(numbers: &Vec<u64>,
                  target_val: u64,
                  curr_val: u64,
                  curr_idx: usize) -> bool {
        if curr_idx >= numbers.len() {
            // we are at the end
            return target_val == curr_val;
        }

        return Self::can_work_2(numbers,
                        target_val,
                        curr_val + numbers[curr_idx],
                        curr_idx + 1) ||
            Self::can_work_2(numbers,
                     target_val,
                     curr_val * numbers[curr_idx],
                     curr_idx + 1) ||
            Self::can_work_2(numbers,
                             target_val,
                             (curr_val * 10u64.pow(1 + numbers[curr_idx].ilog10())) + numbers[curr_idx],
                             curr_idx + 1);
    }
}

impl solution::Solution for Day7 {
    fn solve_p2(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| {
                let mut iter = line.split(":");
                let target = iter
                    .next()
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                let nums: Vec<u64> = iter
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect();
                let can_work = Day7::can_work_2(&nums,
                                              target,
                                              nums[0],
                                              1);
                if can_work {
                    return target;
                } else {
                    return 0;
                }
            })
            .sum::<u64>()
            .to_string()
    }

    fn solve_p1(&self, input: &str) -> String {
        // We effectively want to DFS the list, and bail out if we've gone too high
        input
            .lines()
            .map(|line| {
                let mut iter = line.split(":");
                let target = iter
                    .next()
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                let nums: Vec<u64> = iter
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect();
                let can_work = Day7::can_work(&nums,
                                              target,
                                              nums[0],
                                              1);
                if can_work {
                    return target;
                } else {
                    return 0;
                }
            })
            .sum::<u64>()
            .to_string()
    }
}
