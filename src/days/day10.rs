use std::collections::HashSet;

use crate::solution;
pub struct Day10;

impl Day10 {
    fn dfs_count_all_paths(map: &Vec<Vec<u32>>, row: usize, col: usize) -> u32 {
        let curr = map[row][col];
        if curr == 9 {
            return 1;
        }

        let mut next = Vec::new();
        next.push((row+1, col));
        next.push((row, col+1));
        if row > 0 { next.push((row-1, col)); }
        if col > 0 { next.push((row, col-1)); }

        return next
            .iter()
            .filter(|(next_row, next_col)|
                *next_row < map.len() && *next_col < map[0].len()
            )
            .filter(|(next_row, next_col)|
                map[*next_row][*next_col] == curr + 1
            )
            .map(|(next_row, next_col)|
                Self::dfs_count_all_paths(map, *next_row, *next_col)
            )
            .sum()
    }

    fn dfs(map: &Vec<Vec<u32>>, row: usize, col: usize, mut seen: &mut HashSet<(usize, usize)>) -> u32 {
        if (*seen).contains(&(row, col)) { return 0; }
        seen.insert((row, col));

        let curr = map[row][col];
        if curr == 9 {
            return 1;
        }

        let mut next = Vec::new();
        next.push((row+1, col));
        next.push((row, col+1));
        if row > 0 { next.push((row-1, col)); }
        if col > 0 { next.push((row, col-1)); }

        return next
            .iter()
            .filter(|(next_row, next_col)|
                *next_row < map.len() && *next_col < map[0].len()
            )
            .filter(|(next_row, next_col)|
                map[*next_row][*next_col] == curr + 1
            )
            .map(|(next_row, next_col)|
                Self::dfs(map, *next_row, *next_col, &mut seen)
            )
            .sum()
    }
}

impl solution::Solution for Day10 {
    fn solve_p2(&self, input: &str) -> String {
        let map: Vec<Vec<u32>> = input
            .lines()
            .map(|line|
                line
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect()
            )
            .collect();

        let zeros: Vec<(usize, usize)> = map
            .iter()
            .enumerate()
            .flat_map(|(row_idx, row)|
                row
                    .iter()
                    .enumerate()
                    .filter(|(_, u)| *u == &0u32)
                    .map(move |(col_idx, _)| (row_idx, col_idx))
            )
            .collect();

        return zeros
            .iter()
            .map(|(row, col)| Day10::dfs_count_all_paths(&map, *row, *col))
            .sum::<u32>()
            .to_string();
    }

    fn solve_p1(&self, input: &str) -> String {
        let map: Vec<Vec<u32>> = input
            .lines()
            .map(|line|
                line
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect()
            )
            .collect();

        let zeros: Vec<(usize, usize)> = map
            .iter()
            .enumerate()
            .flat_map(|(row_idx, row)|
                row
                    .iter()
                    .enumerate()
                    .filter(|(_, u)| *u == &0u32)
                    .map(move |(col_idx, _)| (row_idx, col_idx))
            )
            .collect();

        return zeros
            .iter()
            .map(|(row, col)| Day10::dfs(&map, *row, *col, &mut HashSet::new()))
            .sum::<u32>()
            .to_string();
    }
}
