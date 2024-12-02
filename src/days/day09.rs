use std::iter;

use crate::solution;
pub struct Day9;

impl solution::Solution for Day9 {
    fn solve_p2(&self, input: &str) -> String {
        // (start_idx, len, id)
        let all_regions: Vec<(usize, u32, Option<usize>)> = input
            .chars()
            .filter_map(|c| c.to_digit(10))
            .enumerate()
            .map(|(idx, d)|
                (if idx % 2 != 0 { None } else { Some(idx / 2) }, d)
            )
            .fold(Vec::new(), |mut acc, (id, d)| {
                let start_idx = acc.last().map(|(prev_start_idx, prev_len, _)| (*prev_start_idx) + (*prev_len as usize)).unwrap_or(0usize);
                let len = d.try_into().unwrap();
                acc.push((start_idx, len, id));
                acc
            });

        let mut empty_regions: Vec<(usize, u32)> = all_regions
            .iter()
            .filter_map(|(start_idx, len, maybe_id)|
                match maybe_id {
                    None => Some((*start_idx, *len)),
                    _ => None,
                }
            ).collect();
        let mut filled_regions: Vec<(usize, u32, usize)> = all_regions
            .iter()
            .filter_map(|(start_idx, len, maybe_id)|
                maybe_id.map(|id| (*start_idx, *len, id))
            ).collect();


        let mut filled_cursor = filled_regions.len() - 1;

        loop {
            let (file_start_idx, file_len, file_id) = filled_regions[filled_cursor];
            // Find the first empty region which fits the file
            let empty = empty_regions
                .iter_mut()
                .take_while(|(empty_start, _)| *empty_start < file_start_idx)
                .filter(|(_, empty_size)| *empty_size >= file_len)
                .next();
            match empty {
                Some((empty_start, empty_size)) => {
                    // Problem - need to be adding back in the empty space we moved out of?
                    // (or do we? once we've moved out of a spot, it is further right than anything
                    // else we are going to try and move. so i think we're ok there)
                    filled_regions[filled_cursor] = (*empty_start, file_len, file_id);
                    *empty_start = *empty_start + (file_len as usize);
                    *empty_size = *empty_size - file_len;
                },
                _ => {},
            }
            if filled_cursor == 0 {
                break;
            } else {
                filled_cursor -= 1;
            }
        }


        let checksum = filled_regions
            .iter()
            .map(|(a, b, c)| (*a as u64, *b as u64, *c as u64))
            .map(|(start_idx, len, id)|
                id * ((start_idx + (start_idx+len-1))*len/2)
            ).sum::<u64>()
            .to_string();

        return checksum;
    }

    fn solve_p1(&self, input: &str) -> String {
        let mut state = input
            .chars()
            .filter_map(|c| c.to_digit(10))
            .enumerate()
            .map(|(idx, d)|
                (if idx % 2 != 0 { None } else { Some(idx / 2) }, d)
            )
            .flat_map(|(id, d)|
                iter::repeat(id)
                    .take(d.try_into().unwrap())
            ).collect::<Vec<_>>();

        let mut left = 0;
        let mut right = state.len() - 1;

        while left < right {
            // Seek next empty block
            while state[left] != None {
                left += 1;
            }
            // Seek last populated block
            while state[right] == None {
                right -= 1;
            }

            if left < right {
                state[left] = state[right];
                state[right] = None;
            }
        }

        return state
            .iter()
            .filter_map(|&x| x)
            .enumerate()
            .map(|(idx, val)| idx * val)
            .sum::<usize>()
            .to_string();
    }
}
