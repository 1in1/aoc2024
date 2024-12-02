use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use itertools::Itertools;

use crate::solution;
pub struct Day24;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Value {
    Known(bool),
    Unknown(String, String, String),
}

impl Day24 {
    fn apply(
        op: &str,
        lhs: bool,
        rhs: bool,
    ) -> bool {
        match op {
            "AND" => lhs && rhs,
            "OR" => lhs || rhs,
            "XOR" => lhs ^ rhs,
            _ => panic!(),
        }
    }

    fn get_val(
        id: &String,
        values: &mut HashMap<String, Value>,
    ) -> bool {
        let existing_value: Value = (*values.get(id).unwrap()).clone();
        let value: bool = match existing_value {
            Value::Known(known_value) => known_value,
            Value::Unknown(op, lhs, rhs) => Self::apply(
                &op,
                Self::get_val(
                    &lhs,
                    values
                ),
                Self::get_val(
                    &rhs,
                    values
                )
            )
        };

        values.insert(id.clone(), Value::Known(value));
        return value;
    }

    fn add_using_circuit(
        x: u64,
        y: u64,
        gate_state: &mut HashMap<String, Value>,
    ) -> u64 {
        // Set the inputs
        let mut x_acc = x;
        let mut y_acc = y;
        for i in 0..64 {
            gate_state.insert(format!("x{:02}", i), Value::Known((x_acc % 2) == 1));
            gate_state.insert(format!("y{:02}", i), Value::Known((y_acc % 2) == 1));
            x_acc = x_acc >> 1;
            y_acc = y_acc >> 1;
        }

        let mut targets = gate_state
            .keys()
            .filter(|id| id.chars().nth(0).unwrap() == 'z')
            .map(|id| id.clone())
            .collect::<Vec<_>>();
        targets.sort();

        targets
            .iter()
            .map(|id| Self::get_val(id, gate_state))
            .map(|bit| bit as u64)
            .rev()
            .fold(0, |acc, elt|
                (acc << 1) + elt
            )
    }

    fn bad_inputs(
        gate_states: &mut HashMap<String, Value>,
    ) -> u64 {
        let mut bad = 0;
        for i in 0..63 {
            for j in 0..63 {
                let x = (1 << i);
                let y = (1 << j);
                let z = Self::add_using_circuit(x, y, gate_states);
                if z != x+y {
                    //println!("{:?}", (x, y, z));
                    //out.push((x, y, z));
                    bad += 1;
                }
            }
        }
        return bad;
    }

    fn bad_gates(
        gate_states: &HashMap<String, Value>,
    ) -> Vec<String> {
        let xor_str = "XOR".to_string();
        gate_states
            .iter()
            .filter(|(key, val)| {
                let key_first_char = key.chars().nth(0).unwrap();
                if key_first_char == 'z' {
                    if let Value::Unknown(op, _, _) = *val {
                        return *op != xor_str;
                    } else {
                        return false;
                    }
                } else if let Value::Unknown(op, lhs, rhs) = *val {
                    if *op == xor_str {
                        let lhs_first_char = lhs.chars().nth(0).unwrap();
                        let rhs_first_char = rhs.chars().nth(0).unwrap();
                        let first_chars = [key_first_char, lhs_first_char, rhs_first_char];
                        return !first_chars
                            .iter()
                            .any(|c| *c == 'z' || *c == 'x' || *c == 'y');
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            })
            .map(|(key, _)| key.clone())
            .collect::<Vec<_>>()
    }

    fn swap(
        id1: String,
        id2: String,
        gate_states: &mut HashMap<String, Value>
    ) {
        let tmp = gate_states.get(&id1).unwrap().clone();
        gate_states.insert(id1, gate_states.get(&id2).unwrap().clone());
        gate_states.insert(id2, tmp);
    }

    fn has_cycle(
        gate_states: &HashMap<String, Value>,
    ) -> bool {
        gate_states
            .keys()
            .filter(|id| id.chars().nth(0).unwrap() == 'z')
            .map(|id| id.clone())
            .any(|target_id|
                Self::dfs_has_cycle(
                    target_id,
                    gate_states,
                    &mut HashSet::new()
                )
            )
    }

    fn dfs_has_cycle(
        id: String,
        gate_states: &HashMap<String, Value>,
        seen: &mut HashSet<String>
    ) -> bool {
        if seen.contains(&id) {
            return true;
        }
        seen.insert(id.clone());

        match gate_states.get(&id).unwrap() {
            Value::Known(_) => false,
            Value::Unknown(_, lhs, rhs) =>
                Self::dfs_has_cycle(lhs.clone(), gate_states, seen) ||
                Self::dfs_has_cycle(rhs.clone(), gate_states, seen),
        }
    }
}

impl solution::Solution for Day24 {
    fn solve_p2(&self, input: &str) -> String {
        // P2 done with offline analysis :(
        // Automated solution to come one day, if I can be bothered
        String::new()
    }

    fn solve_p1(&self, input: &str) -> String {
        let re = Regex::new(r"(.*): (\d+)").unwrap();
        let re2 = Regex::new(r"(.*) (.*) (.*) -> (.*)").unwrap();
        let mut gate_state = input
            .lines()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                let captures = re.captures(line).unwrap();
                (
                    captures[1].to_string(),
                    Value::Known(captures[2].parse::<u32>().unwrap() == 1),
                )
            })
            .chain(
                input
                    .lines()
                    .skip_while(|line| !line.is_empty())
                    .filter_map(|line| re2.captures(line))
                    .map(|captures|
                        (
                            captures[4].to_string(),
                            Value::Unknown(
                                captures[2].to_string(),
                                captures[1].to_string(),
                                captures[3].to_string(),
                            ),
                        )
                    )
            )
            .collect::<HashMap<_, _>>();

        let mut targets = gate_state
            .keys()
            .filter(|id| id.chars().nth(0).unwrap() == 'z')
            .map(|id| id.clone())
            .collect::<Vec<_>>();
        targets.sort();

        targets
            .iter()
            .map(|id| Self::get_val(id, &mut gate_state))
            .map(|bit| bit as u64)
            .rev()
            .fold(0, |acc, elt|
                (acc << 1) + elt
            )
            .to_string()
    }
}
