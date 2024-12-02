use std::collections::HashSet;

use crate::solution;
pub struct Day12;

impl Day12 {
    fn has_corner(
        top_left_row: i64,
        top_left_col: i64,
        region: &HashSet<(usize, usize)>
    ) -> usize {
        let square_coords = [
            (top_left_row, top_left_col),
            (top_left_row + 1, top_left_col),
            (top_left_row, top_left_col + 1),
            (top_left_row + 1, top_left_col + 1),
        ];
        let num_in_curr_region = square_coords
            .iter()
            .enumerate()
            .filter(|(_, (x, y))|
                if
                    ((*x) >= 0) &&
                    ((*y) >= 0) &&
                    region.contains(&(*x as usize, *y as usize))
                {
                    true
                } else {
                    false
                },
            ).collect::<Vec<_>>();

        if num_in_curr_region.len() == 0 {
            return 0;
        }

        if num_in_curr_region.len() == 1 || num_in_curr_region.len() == 3 {
            return 1;
        }

        if num_in_curr_region.iter().map(|(idx, _)| *idx).collect::<Vec<_>>() == vec![0, 3] ||
            num_in_curr_region.iter().map(|(idx, _)| *idx).collect::<Vec<_>>() == vec![1, 2] {
            // We have two opposite corners
            return 2;
        }

        return 0;
    }

    fn _fill(
        map: &Vec<Vec<char>>,
        row: usize,
        col: usize,
        mut seen: &mut HashSet<(usize, usize)>
    ) {
        if seen.contains(&(row, col)) {
            return
        }

        seen.insert((row, col));
        let poss_next = [
            (row.checked_sub(1), Some(col)),
            (Some(row), Some(col + 1)),
            (Some(row + 1), Some(col)),
            (Some(row), col.checked_sub(1)),
        ];
        let exp_next = poss_next
            .iter()
            .filter_map(|(maybe_x, maybe_y)|
                match (maybe_x, maybe_y) {
                    (Some(x), Some(y)) => if (*x as usize) < map.len() && (*y as usize) < map[0].len() && map[*x][*y] == map[row][col] { Some((x, y)) } else { None },
                    _ => None,
                }
            );

        for (x, y) in exp_next {
            Self::_fill(map, *x, *y, &mut seen);
        }
    }

    fn fill(
        map: &Vec<Vec<char>>,
        row: usize,
        col: usize
    ) -> HashSet<(usize, usize)> {
        let mut set: HashSet<(usize, usize)> = HashSet::new();
        Self::_fill(map, row, col, &mut set);
        return set;
    }

    fn region_area_and_verts(
        map: &Vec<Vec<char>>,
        row: usize,
        col: usize,
        seen: &mut HashSet<(usize, usize)>
    ) -> (u64, u64) {
        if seen.contains(&(row, col)) {
            return (0, 0);
        }

        let region = Self::fill(map, row, col);

        let mut checked_corners = HashSet::new();
        for (x_row, x_col) in region.clone() {
            // Mark these spaces as seen
            seen.insert((x_row, x_col));

            // And look for vertices
            let to_check = [
                (x_row as i64, x_col as i64),
                (x_row as i64 - 1, x_col as i64),
                (x_row as i64, x_col as i64 - 1),
                (x_row as i64 - 1, x_col as i64 - 1),
            ];
            for (p, q) in to_check {
                checked_corners.insert((p, q));
            }
        }

        let verts = checked_corners
            .iter()
            .map(|(p, q)| Self::has_corner(*p, *q, &region))
            .sum::<usize>();

        return (region.len() as u64, verts as u64);
    }

    fn fill_region(map: &Vec<Vec<char>>, row: usize, col: usize, mut seen: &mut HashSet<(usize, usize)>) -> (u64, u64) {
        if seen.contains(&(row, col)) {
            // contributions already accounted for
            return (0, 0);
        }

        seen.insert((row, col));

        let poss_next = [
            (row.checked_sub(1), Some(col)),
            (Some(row + 1), Some(col)),
            (Some(row), col.checked_sub(1)),
            (Some(row), Some(col + 1)),
        ];

        let (mut area, mut perim) = (1, 0);
        for next in poss_next {
            if let (Some(next_row), Some(next_col)) = next {
                if next_row < map.len() && next_col < map[0].len() {
                    if map[next_row][next_col] == map[row][col] {
                        let (contrib_area, contrib_perim) = Self::fill_region(map, next_row, next_col, &mut seen);
                        area += contrib_area;
                        perim += contrib_perim;

                        continue;
                    }
                }
            }

            // We are looking at at boundary
            perim += 1;
        }

        return (area, perim);
    }
}

impl solution::Solution for Day12 {
    fn solve_p2(&self, input: &str) -> String {
        // Suffices to count vertices
        let map: Vec<Vec<_>> = input
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        let mut seen = HashSet::new();

        let price = (0..map.len())
            .map(|row|
                (0..map[0].len())
                    .map(|col| Self::region_area_and_verts(&map, row, col, &mut seen))
                    .map(|(area, vs)|
                        (area * vs) as u64
                    )
                    .sum::<u64>()
            )
            .sum::<u64>()
            .to_string();

        return price;
    }

    fn solve_p1(&self, input: &str) -> String {
        let map: Vec<Vec<_>> = input
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        let mut seen = HashSet::new();

        let price = (0..map.len())
            .map(|row|
                (0..map[0].len())
                    .map(|col| Self::fill_region(&map, row, col, &mut seen))
                    .map(|(area, perim)| area * perim)
                    .sum::<u64>()
            )
            .sum::<u64>()
            .to_string();

        return price;
    }
}
