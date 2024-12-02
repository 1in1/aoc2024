use std::collections::HashSet;

use crate::solution;
pub struct Day15;


impl Day15 {
    fn update_pos(
        pos: (usize, usize),
        dir: char
    ) -> (usize, usize) {
        match dir {
            '^' => (pos.0 - 1, pos.1),
            '>' => (pos.0, pos.1 + 1),
            'v' => (pos.0 + 1, pos.1),
            '<' => (pos.0, pos.1 - 1),
            _   => panic!(),
        }
    }

    // Tiles that move with this piece (excluding piece)
    fn connected_2(
        map: &Vec<Vec<char>>,
        to: (usize, usize)
    ) -> Vec<(usize, usize)> {
        match map[to.0][to.1] {
            '[' => vec![Self::update_pos(to, '>')],
            ']' => vec![Self::update_pos(to, '<')],
            _   => vec![],
        }
    }

    // Get all tiles which move with this piece
    fn connected(
        map: &Vec<Vec<char>>,
        to: (usize, usize)
    ) -> Vec<(usize, usize)> {
        match map[to.0][to.1] {
            '@' | 'O' => vec![to],
            '[' => vec![to, Self::update_pos(to, '>')],
            ']' => vec![to, Self::update_pos(to, '<')],
            _   => vec![],
        }
    }

    fn pieces_to_move(
        map: &Vec<Vec<char>>,
        flowing_from: (usize, usize),
        dir: char
    ) -> Option<Vec<(usize, usize)>> {
        let next = Self::update_pos(flowing_from, dir);

        match (map[flowing_from.0][flowing_from.1], map[next.0][next.1]) {
            (_, '#') => None,
            ('@' | 'O' | '[' | ']',
             'O' | '[' | ']') => {
                let mut v_v_connected = Self::connected(map, next)
                    .iter()
                    .filter(|connected| **connected != flowing_from)
                    .map(|connected|
                        Self::pieces_to_move(
                            map,
                            *connected,
                            dir
                        )
                    )
                    .collect::<Vec<_>>();

                if v_v_connected
                    .iter()
                    .any(|v| v.is_none()) {
                    return None;
                }

                v_v_connected.push(Some(Self::connected(map, flowing_from)));

                return Some(v_v_connected
                    .into_iter()
                    .filter_map(|x| x)
                    .flatten()
                    .collect());
            },
            ('@' | 'O' | '[' | ']',
             '.') => Some(
                Self::connected(map, flowing_from)
            ),
            _ => Some(vec![]),
        }
    }

    fn update_2(
        map: &mut Vec<Vec<char>>,
        robot_pos: (usize, usize),
        dir: char
    ) -> (usize, usize) {
        let pieces = Self::pieces_to_move(map, robot_pos, dir)
            .map(|vec| {
                let mut set: HashSet<(usize, usize)> = HashSet::new();
                let mut new_vec: Vec<(usize, usize)> = Vec::new();
                for x in vec {
                    if !set.contains(&x) {
                        new_vec.push(x);
                    }
                    set.insert(x);
                }
                new_vec.sort_by(|a, b|
                    match dir {
                        '<' => a.1.cmp(&b.1),
                        '>' => b.1.cmp(&a.1),
                        '^' => a.0.cmp(&b.0),
                        'v' => b.0.cmp(&a.0),
                        _ => panic!(),
                    }
                );

                return new_vec;
            });

        if let Some(to_update) = pieces {
            for updatable in to_update {
                let next = Self::update_pos(updatable, dir);

                // The updates are sorted, so this is safe
                map[next.0][next.1] = map[updatable.0][updatable.1];
                map[updatable.0][updatable.1] = '.';
            }

            return Self::update_pos(robot_pos, dir);
        } else {
            return robot_pos;
        }
    }
}

impl solution::Solution for Day15 {
    fn solve_p2(&self, input: &str) -> String {
        let mut map = input
            .lines()
            .take_while(|line| *line != "")
            .map(|line|
                 line
                    .chars()
                    .flat_map(|c|
                        match c {
                            '#' => vec!['#', '#'],
                            '.' => vec!['.', '.'],
                            'O' => vec!['[', ']'],
                            '@' => vec!['@', '.'],
                            _ => panic!(),
                        }
                    )
                    .collect::<Vec<char>>()
            )
            .collect::<Vec<Vec<char>>>();
        let instructions = input
            .lines()
            .skip_while(|line| *line != "")
            .flat_map(|line| line.chars())
            .collect::<Vec<_>>();

        let initial_robot_pos = map
            .iter()
            .enumerate()
            .filter_map(|(row_idx, row)|
                row
                    .iter()
                    .enumerate()
                    .filter_map(|(col_idx, c)|
                        if *c == '@' {
                            Some((row_idx, col_idx))
                        } else {
                            None
                        }
                    )
                    .next()
            )
            .next()
            .unwrap();

        let robot_pos = instructions
            .iter()
            .fold(initial_robot_pos, |pos, dir|
                Self::update_2(&mut map, pos, *dir)
            );

        map
            .into_iter()
            .enumerate()
            .map(|(row_idx, row)|
                row
                    .into_iter()
                    .enumerate()
                    .filter_map(|(col_idx, c)|
                        if c == '[' {
                            Some(100*(row_idx as u32) + (col_idx as u32))
                        } else {
                            None
                        }
                    )
                    .sum::<u32>()
            )
            .sum::<u32>()
            .to_string()
    }

    fn solve_p1(&self, input: &str) -> String {
        let mut map = input
            .lines()
            .take_while(|line| *line != "")
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let instructions = input
            .lines()
            .skip_while(|line| *line != "")
            .flat_map(|line| line.chars())
            .collect::<Vec<_>>();

        let initial_robot_pos = map
            .iter()
            .enumerate()
            .filter_map(|(row_idx, row)|
                row
                    .iter()
                    .enumerate()
                    .filter_map(|(col_idx, c)|
                        if *c == '@' {
                            Some((row_idx, col_idx))
                        } else {
                            None
                        }
                    )
                    .next()
            )
            .next()
            .unwrap();

        let robot_pos = instructions
            .iter()
            .fold(initial_robot_pos, |pos, dir|
                Self::update_2(&mut map, pos, *dir)
            );

        map
            .into_iter()
            .enumerate()
            .map(|(row_idx, row)|
                row
                    .into_iter()
                    .enumerate()
                    .filter_map(|(col_idx, c)|
                        if c == 'O' {
                            Some(100*(row_idx as u32) + (col_idx as u32))
                        } else {
                            None
                        }
                    )
                    .sum::<u32>()
            )
            .sum::<u32>()
            .to_string()

    }
}
