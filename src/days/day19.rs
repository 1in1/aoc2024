use std::collections::HashMap;
use std::collections::HashSet;

use crate::solution;
pub struct Day19;

// We want to build a prefix tree
#[derive(Clone, Eq, PartialEq, Debug)]
struct Trie {
    root: Node,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Node {
    is_terminal: bool,
    children: HashMap<char, Node>,
}

impl Trie {
    fn from<'life>(strings: impl Iterator<Item = &'life str>) -> Self {
        let mut root = Node { is_terminal: false, children: HashMap::new() };
        for string in strings {
            let mut curr: &mut Node = &mut root;
            for c in string.chars() {
                curr = curr
                    .children
                    .entry(c)
                    .or_insert(Node { is_terminal: false, children: HashMap::new() });
            }
            curr.is_terminal = true;
        }
        return Trie { root: root };
    }

    fn prefixes<'life>(&self, in_string: &'life str) -> impl Iterator<Item=&'life str> + use<'life, '_> {
        in_string
            .chars()
            .enumerate()
            .scan(&self.root, |acc, (idx, c)| {
                if let Some(next) = (*acc).children.get(&c) {
                    *acc = next;
                    return Some((idx, next.is_terminal))
                } else {
                    return None;
                }
            })
            .filter(|(_, is_terminal)| *is_terminal)
            .map(|(idx, _)| &in_string[0..idx+1])
    }
}

impl Day19 {
    fn dfs<'life>(
        in_string: &'life str,
        trie: &Trie,
        cache: &mut HashMap<String, u64>
    ) -> u64 {
        if let Some(cached) = cache.get(&in_string.to_string()) {
            return *cached;
        }

        if in_string.len() == 0 {
            return 1;
        }

        let result = trie
            .prefixes(in_string)
            .map(|prefix|
                Self::dfs(&in_string[prefix.len()..], trie, cache)
            )
            .sum::<u64>();
        cache.insert(in_string.to_string(), result);
        return result;
    }
}

impl solution::Solution for Day19 {
    fn solve_p2(&self, input: &str) -> String {
        let towels = input
            .lines()
            .next()
            .unwrap()
            .split(", ");
        let trie = Trie::from(towels);

        input
            .lines()
            .skip(2)
            .map(|line| Self::dfs(line, &trie, &mut HashMap::new()))
            .sum::<u64>()
            .to_string()
    }

    fn solve_p1(&self, input: &str) -> String {
        let towels = input
            .lines()
            .next()
            .unwrap()
            .split(", ");
        let trie = Trie::from(towels);

        input
            .lines()
            .skip(2)
            .filter(|line| Self::dfs(line, &trie, &mut HashMap::new()) > 0)
            .count()
            .to_string()
    }
}
