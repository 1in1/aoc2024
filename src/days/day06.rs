use std::collections::HashSet;
use itertools::Itertools;

use crate::solution;
pub struct Day6;


#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Up,
            '<' => Direction::Left,
            _   => panic!(),
        }
    }

    fn update_2(
        &self,
        loc: (usize, usize),
        within: (usize, usize)
    ) -> Option<(usize, usize)> {
        match self {
            Direction::Up =>
                if loc.0 > 0 { Some((loc.0 - 1, loc.1)) }
                else { None },
            Direction::Right =>
                if loc.1 + 1 < within.1 { Some((loc.0, loc.1 + 1)) }
                else { None },
            Direction::Down =>
                if loc.0 + 1 < within.0 { Some((loc.0 + 1, loc.1)) }
                else { None },
            Direction::Left =>
                if loc.1 > 0 { Some((loc.0, loc.1 - 1)) }
                else { None },
        }
    }

    fn next(&self) -> Self {
        match self {
            Direction::Up    => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down  => Direction::Left,
            Direction::Left  => Direction::Up,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct State {
    loc: (usize, usize),
    dir: Direction,
}

struct WalkIterator<'a> {
    state: Option<State>,
    map: &'a Vec<Vec<char>>,
    boundaries: (usize, usize),
}

impl Iterator for WalkIterator<'_> {
    type Item = State;

    fn next(&mut self) -> Option<State> {
        let curr_state = self.state?;

        self.state = curr_state
            .dir
            .update_2(curr_state.loc, self.boundaries)
            .map(|fwd_loc|
                // We are inside the map still
                if self.map[fwd_loc.0][fwd_loc.1] != '#' {
                    State { loc: fwd_loc, dir: curr_state.dir }
                } else {
                    State { loc: curr_state.loc, dir: curr_state.dir.next() }
                }
            );

        return Some(curr_state);
    }
}

impl Day6 {
    fn parse(input: &str) -> (Vec<Vec<char>>, (usize, usize), Direction) {
        let map: Vec<Vec<char>> = input
            .lines()
            .map(|line| line
                 .chars()
                 .collect()
            )
            .collect();
        let loc = map
            .iter()
            .enumerate()
            .filter_map(|(row_idx, row)|
                row
                    .iter()
                    .enumerate()
                    .filter(|(_, &c)|
                        c != '.' && c != '#'
                    )
                    .next()
                    .map(|(col_idx, _)| (row_idx, col_idx))
            )
            .next()
            .unwrap();
        let dir = Direction::from_char(map[loc.0][loc.1]);

        return (map, loc, dir);
    }

    fn walk_no_loops(
        map: &Vec<Vec<char>>,
        init_loc: (usize, usize),
        init_dir: Direction
    ) -> HashSet<(usize, usize)> {
        HashSet::from_iter(WalkIterator { 
            state: Some(State { 
                loc: init_loc,
                dir: init_dir
            }),
            map: map,
            boundaries: (map.len(), map[0].len())
        }.map(|state| state.loc))
    }

    fn walk_find_loop(
        map: &Vec<Vec<char>>,
        init_loc: (usize, usize),
        init_dir: Direction
    ) -> bool {
        let iter = WalkIterator { 
            state: Some(State { 
                loc: init_loc,
                dir: init_dir
            }),
            map: map,
            boundaries: (map.len(), map[0].len())
        };

        let mut seen: HashSet<State> = HashSet::new();
        // Only look at states where we changed direction, for the sake of speed
        for (curr_state, _) in iter
            .tuple_windows()
            .filter(|(fst, snd)| fst.dir != snd.dir) {
            if seen.contains(&curr_state) {
                return true;
            }

            seen.insert(curr_state);
        }

        return false;
    }
}

impl solution::Solution for Day6 {
    fn solve_p1(&self, input: &str) -> String {
        let (map, loc, dir) = Day6::parse(input);
        let seen = Day6::walk_no_loops(&map, loc, dir);
        return seen.len().to_string();
    }

    fn solve_p2(&self, input: &str) -> String {
        let (mut map, init_loc, init_dir) = Day6::parse(input);
        // We have to collect here because we are going to mutate `map`
        let states_in_initial_walk = WalkIterator { 
            state: Some(State { 
                loc: (init_loc.0, init_loc.1),
                dir: init_dir
            }),
            map: &map,
            boundaries: (map.len(), map[0].len())
        }.collect::<Vec<_>>();

        let mut seen = HashSet::new();
        states_in_initial_walk
            .into_iter()
            .tuple_windows()
            // Only look at ones where we change location
            .filter(|(fst, snd)| (*fst).loc != (*snd).loc)
            // Only try and place a block the first time we see the location
            .filter(|(_, snd)| {
                if seen.contains(&snd.loc) { return false; }
                seen.insert(snd.loc);
                return true;
            })
            .filter(|(fst, snd)| {
                // Takes a mutable borrow of `map`
                //
                // Put the block at `snd`, and begin the walk at `fst`
                //
                // Something's wrong now...
                let tmp = map[(*snd).loc.0][(*snd).loc.1];
                map[(*snd).loc.0][(*snd).loc.1] = '#';
                let result = Self::walk_find_loop(&map, (*fst).loc, (*fst).dir);
                map[(*snd).loc.0][(*snd).loc.1] = tmp;
                return result;
            })
            .count()
            .to_string()
    }
}
