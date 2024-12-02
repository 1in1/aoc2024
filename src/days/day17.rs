use regex::Regex;
use itertools::Itertools;
use std::iter::zip;
use std::ops;

use crate::solution;
pub struct Day17;
use std::io::{stdin, stdout, Read, Write};

pub fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    a: u64,
    b: u64,
    c: u64,
    i: usize,
}

impl Day17 {
    fn run(program: &Vec<u64>, initial_state: State) -> Vec<u64> {
        let mut state = initial_state;
        let mut output = Vec::new();
        while state.i < program.len() {
            let operation = program[state.i];

            let combo_operand = match program[state.i + 1] {
                0 | 1 | 2 | 3 => program[state.i + 1],
                4 => state.a,
                5 => state.b,
                6 => state.c,
                _ => panic!(),
            };

            match operation {
                0 => state.a = state.a >> combo_operand,
                1 => state.b = state.b ^ program[state.i + 1],
                2 => state.b = combo_operand % 8,
                3 => if state.a != 0 { state.i = program[state.i + 1] as usize; continue; },
                4 => state.b = state.b ^ state.c,
                5 => output.push(combo_operand % 8),
                6 => state.b = state.a >> combo_operand,
                7 => state.c = state.a >> combo_operand,
                _ => panic!(),
            }

            state.i += 2;
        }

        return output;
    }

    // The only way to change a is to rshift it by `combo_operator`
    // Outside analysis of the program (boo) reveals that this is
    // only called with the operand 3
    fn dfs(
        program: &Vec<u64>,
        initial_state: &State,
        answer_chunks: Vec<u64>,
        must_match: usize
    ) -> Option<u64> {
        let a = answer_chunks
            .iter()
            .fold(0, |acc, elt| (acc << 3) + elt);
        let output = Self::run(
            &program,
            {
                let mut tmp = initial_state.clone();
                tmp.a = a;
                tmp
            }
        );

        // If we don't match at least `must_match`, we bail
        let match_count = zip(output.into_iter().rev(), program.iter().rev())
            .take_while(|(o, p)| o == *p)
            .count();
        if match_count < must_match { return None; }

        if must_match == program.len() {
            // We've matched the whole thing
            return Some(a);
        }

        // We have 8 directions we can go in
        for i in 0..8 {
            if let Some(sol) = Self::dfs(
                program,
                initial_state,
                {
                    let mut tmp = answer_chunks.clone();
                    tmp.push(i);
                    tmp
                },
                must_match + 1
            ) {
                return Some(sol);
            }
        }

        return None;
    }
}

impl solution::Solution for Day17 {
    fn solve_p2(&self, input: &str) -> String {
        let re = Regex::new(r"[^\d]*(\d*)").unwrap();
        let mut initial_state = match &input.lines().take(3).map(|line| re.captures(line).unwrap()[1].parse::<u64>().unwrap()).collect::<Vec<_>>()[..] {
            &[a, b, c] => State { a: a, b: b, c: c, i: 0 },
            _ => panic!(), 
        };
        let re2 = Regex::new(r".*Program: (.*)").unwrap();
        let program: Vec<u64> = re2
            .captures(input)
            .unwrap()[1]
            .split(",")
            .map(|c| c.parse::<u64>().unwrap())
            .collect();

        Self::dfs(
            &program,
            &initial_state,
            Vec::new(),
            0
        )
            .unwrap()
            .to_string()
    }

    fn solve_p1(&self, input: &str) -> String {
        let re = Regex::new(r"[^\d]*(\d*)").unwrap();
        let initial_state = match &input.lines().take(3).map(|line| re.captures(line).unwrap()[1].parse::<u64>().unwrap()).collect::<Vec<_>>()[..] {
            &[a, b, c] => State { a: a, b: b, c: c, i: 0 },
            _ => panic!(), 
        };
        let re2 = Regex::new(r".*Program: (.*)").unwrap();
        let program: Vec<u64> = re2
            .captures(input)
            .unwrap()[1]
            .split(",")
            .map(|c| c.parse::<u64>().unwrap())
            .collect();

        let output = Self::run(&program, initial_state);

        output
            .into_iter()
            .map(|x| x.to_string())
            .join(",")
    }
}
