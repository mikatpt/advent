use crate::{get_input, Result};
use std::collections::{HashMap, HashSet};

type Graph<'a> = HashMap<&'a str, HashSet<&'a str>>;
type Visited<'a> = HashSet<&'a str>;

trait StringHelpers {
    fn is_uppercase(&self) -> bool;
}

impl StringHelpers for str {
    fn is_uppercase(&self) -> bool {
        self.to_uppercase() == *self
    }
}

fn build_graph_from_str(input: &str) -> Graph {
    let mut res = HashMap::new();
    for line in input.lines() {
        let (start, end) = line.split_once('-').unwrap();

        res.entry(start).or_insert_with(HashSet::new).insert(end);
        res.entry(end).or_insert_with(HashSet::new).insert(start);
    }
    res
}

fn count_paths<'a>(
    graph: &Graph<'a>,
    node: &'a str,
    visited: &mut HashMap<&'a str, u8>,
    mut revisited_cave: bool,
) -> u32 {
    if node == "end" {
        return 1;
    }

    if !node.is_uppercase() && visited.get(node) == Some(&2) {
        revisited_cave = true;
    }

    let mut res = 0;

    let edges = graph.get(node).expect("Malformed graph input");
    for next in edges {
        let val = *visited.entry(next).or_insert(0);
        let can_visit_small_cave = val == 0 || (!revisited_cave && val <= 1);

        if next.is_uppercase() || can_visit_small_cave {
            *visited.entry(next).or_insert(0) += 1;
            res += count_paths(graph, next, visited, revisited_cave);
            *visited.get_mut(next).unwrap() -= 1;
        }
    }
    res
}

pub fn solve() -> Result<(i32, i32)> {
    let input = get_input(1)?;

    Ok((part1(&input)? as i32, part2(&input)? as i32))
}

fn part1(input: &str) -> Result<u32> {
    let graph = build_graph_from_str(input);
    let res = count_paths(&graph, "start", &mut HashMap::from([("start", 3)]), true);

    Ok(res)
}

fn part2(input: &str) -> Result<u32> {
    let graph = build_graph_from_str(input);
    let res = count_paths(&graph, "start", &mut HashMap::from([("start", 3)]), false);
    Ok(res)
}

const INPUT: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

const INPUT2: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(10, part1(INPUT).unwrap());
        assert_eq!(19, part1(INPUT2).unwrap());
    }

    #[test]
    fn test3() {
        assert_eq!(36, part2(INPUT).unwrap());
    }

    #[test]
    fn test4() {
        assert_eq!(103, part2(INPUT2).unwrap());
    }
}
