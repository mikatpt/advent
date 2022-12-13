use crate::{get_input, Result};

pub fn solve() -> Result<(i32, i32)> {
    let input = get_input(13)?;

    Ok((part1(&input).unwrap(), part2(&input).unwrap()))
}

use std::cmp::Ordering::{self, Equal, Greater, Less};

fn part1(input: &str) -> Option<i32> {
    let nodes = parse(input)?;
    let mut ordered = 0;

    let mut iter = nodes.into_iter().enumerate();

    while let Some((idx, first)) = iter.next() && let Some((_, second)) = iter.next() {
        if first.cmp(&second) == Less {
            ordered += 1 + (idx as i32 / 2);
        }
    }
    Some(ordered)
}

fn part2(input: &str) -> Option<i32> {
    use Node::{Int, List};

    let mut nodes = parse(input)?;
    let (d1, d2) = (
        List(vec![List(vec![Int(2)])]),
        List(vec![List(vec![Int(6)])]),
    );
    nodes.extend([d1.clone(), d2.clone()]);

    let mut key = 0;
    nodes.sort();
    for (idx, node) in nodes.into_iter().enumerate() {
        if node == d1 {
            key += idx as i32 + 1;
        }
        if node == d2 {
            key *= idx as i32 + 1;
        }
    }
    Some(key)
}

#[derive(Clone, PartialEq, Eq)]
enum Node {
    List(Vec<Node>),
    Int(i32),
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::List(arg0) => f.debug_list().entries(arg0.iter()).finish(),
            Self::Int(arg0) => write!(f, "{arg0}"),
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        use Node::{Int, List};

        match (self, other) {
            (List(l), List(r)) => {
                let (mut liter, mut riter) = (l.iter(), r.iter());
                loop {
                    match (liter.next(), riter.next()) {
                        (None, None) => return Equal,
                        (None, _) => return Less,
                        (_, None) => return Greater,
                        (Some(l), Some(r)) => {
                            let res = l.cmp(r);
                            if res != Equal {
                                return res;
                            }
                        }
                    }
                }
            }
            (List(l), Int(r)) => self.cmp(&List(vec![other.clone()])),
            (Int(l), List(r)) => List(vec![self.clone()]).cmp(other),
            (Int(l), Int(r)) => l.cmp(r),
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> Option<Vec<Node>> {
    let mut nodes = vec![];
    let mut lines = input.lines();

    while let Some(first) = lines.next() {
        let second = lines.next()?;
        let (n1, n2) = (parse_node(first)?, parse_node(second)?);
        nodes.push(n1);
        nodes.push(n2);
        lines.next();
    }

    Some(nodes)
}

fn parse_node(line: &str) -> Option<Node> {
    recurse(Some(Node::List(vec![])), &line[1..line.len() - 1], &mut 0)
}

fn recurse(mut root: Option<Node>, line: &str, idx: &mut usize) -> Option<Node> {
    while *idx < line.len() {
        let c = &line[*idx..*idx + 1];
        *idx += 1;

        match c {
            "[" => {
                if let Some(Node::List(ref mut l)) = root {
                    l.push(recurse(Some(Node::List(vec![])), line, idx)?);
                }
            }
            "]" => break,
            "," | "" => {}
            _ => {
                *idx -= 1;
                let num: String = line
                    .chars()
                    .skip(*idx)
                    .take_while(|c| {
                        if c.is_numeric() {
                            *idx += 1;
                            return true;
                        }
                        false
                    })
                    .collect();
                let num: i32 = num.parse().expect("should be numeric");
                if let Some(Node::List(ref mut l)) = root {
                    l.push(Node::Int(num));
                }
            }
        }
    }

    root
}

const INPUT: &str = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
        crate::trace::init();
        assert_eq!(13, part1(INPUT).unwrap());
        assert_eq!(140, part2(INPUT).unwrap());
    }
}
