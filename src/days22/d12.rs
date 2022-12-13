use crate::{get_input, Result};

pub fn solve() -> Result<(i32, i32)> {
    let input = get_input(12)?;

    Ok((part1(&input).unwrap(), part2(&input).unwrap()))
}

fn part1(input: &str) -> Option<i32> {
    let (start, end, steps) = parse(input);

    let res = dijkstra(&steps, start, end);

    Some(res)
}

fn part2(input: &str) -> Option<i32> {
    let (start, end, steps) = parse(input);

    let mut possible_starts = vec![start];
    let mut shortest_path = i32::MAX;

    for (i, row) in steps.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if cell.elevation == 'a' {
                possible_starts.push((i, j));
            }
        }
    }

    for s in possible_starts {
        let path = dijkstra(&steps, s, end);
        shortest_path = std::cmp::min(shortest_path, path);
    }

    Some(shortest_path)
}

fn solution(input: &str) -> Option<i32> {
    let (start, end, steps) = parse(input);

    let res = dijkstra(&steps, start, end);

    Some(res)
}

type Input = ((usize, usize), (usize, usize), Vec<Vec<Node>>);

fn parse(input: &str) -> Input {
    let (mut start, mut end) = ((0, 0), (0, 0));
    let steps: Vec<Vec<_>> = input
        .lines()
        .enumerate()
        .map(|(i, row)| {
            let mut v = vec![];
            for (j, c) in row.chars().enumerate() {
                match c {
                    'S' => start = (i, j),
                    'E' => end = (i, j),
                    _ => {}
                };
                v.push(Node {
                    val: 1,
                    elevation: c,
                });
            }
            v
        })
        .collect();

    (start, end, steps)
}

use std::cmp::Reverse;
use std::collections::BinaryHeap;

const DIRS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(Debug)]
struct Node {
    val: i32,
    elevation: char,
}

fn dijkstra(graph: &[Vec<Node>], start: (usize, usize), end: (usize, usize)) -> i32 {
    // Dijkstra's pseudocode:
    // #1 Given a graph of nodes `graph`, and the size of the graph `(m, n)`:
    // #2 Give each node an initial distance-from-start value of infinity, except the initial node.
    // #3 Save indices for the shortest path for reproduction. i.e. paths[end] should point to
    //    paths[2nd-to-end] ... all the way to paths[start].
    // #4 Use a priority queue so we always process the shortest path first.
    // #5 Begin with the first node defined in the problem. In this case, the location of `S`.

    let (m, n) = (graph.len(), graph[0].len()); // #1

    let mut distances = vec![vec![i32::MAX; n]; m]; // #2
    distances[start.0][start.1] = 0;

    let mut possible_paths = vec![vec![None; n]; m]; // #3

    let mut queue = BinaryHeap::new(); // #4

    queue.push(Reverse((distances[start.0][start.1], start))); // #5

    // For each node, consider all unvisited neighbors and calculate their distance.
    // This is the minimum of (distances[neighbor], distances[current] + edge)
    // In this problem, the edge's distance value is always 1.

    // #1 Find a valid neighbor. For each neighbor:
    // #2 If calculated distance < saved neighbor's distance, this means we've found a shorter
    //    path to that neighbor. Then:
    // #3 Update the distance accordingly.
    // #4 Save the neighbor in our priority queue.
    // #5 Save the current node's location at the neighbor's index in `paths` to reproduce it later
    //    via backtracking.
    // #6 Visit the next shortest distance.
    // #7 The end condition of Dijkstra's is to reach the last node OR to hit a smallest distance of
    //    infinity, which means there's no possible path.

    // #6
    while let Some(Reverse((current_dist, curr_position))) = queue.pop() {
        let (x, y) = curr_position;
        let current_node = &graph[x][y];

        // #7
        if curr_position == end || distances[x][y] == i32::MAX {
            break;
        }

        for (dx, dy) in DIRS {
            let (x2, y2) = (x as isize + dx, y as isize + dy);

            if invalid_neighbor(graph, curr_position, (x2, y2)) {
                continue;
            }
            let (x2, y2) = (x2 as usize, y2 as usize); // #1

            let edge = &graph[x2][y2].val; // edge == 1

            let new_distance = distances[x][y] + edge;

            // #2
            if new_distance < distances[x2][y2] {
                distances[x2][y2] = new_distance; // #3
                queue.push(Reverse((new_distance, (x2, y2)))); // #4
                possible_paths[x2][y2] = Some(curr_position); // #5
            }
        }
    }

    // If desired, we can reconstruct the path by going backwards through indices.
    // reconstruct_path(&possible_paths, start);

    // As long as a position has been saved at the end node, we know we found a path.
    if let Some(_position) = possible_paths[end.0][end.1] {
        return distances[end.0][end.1];
    }
    i32::MAX
}

fn invalid_neighbor(
    graph: &[Vec<Node>],
    current_pos: (usize, usize),
    next_pos: (isize, isize),
) -> bool {
    !in_bounds(
        next_pos.0,
        next_pos.1,
        graph.len() as isize,
        graph[0].len() as isize,
    ) || !can_move_to(
        graph[current_pos.0][current_pos.1].elevation,
        graph[next_pos.0 as usize][next_pos.1 as usize].elevation,
    )
}

/// Starting at `a`, you can move to `b` if `b` is at most one step higher.
fn can_move_to(mut a: char, mut b: char) -> bool {
    if a == 'S' {
        a = 'a';
    }
    if b == 'E' {
        b = 'z';
    }
    (a as u32) + 1 >= (b as u32)
}

fn in_bounds(x: isize, y: isize, m: isize, n: isize) -> bool {
    x >= 0 && y >= 0 && x < m && y < n
}

fn reconstruct_path(path: &[Vec<Option<(usize, usize)>>], start: (usize, usize)) {
    let (mut i, mut j) = (path.len() - 1, path[0].len() - 1);
    let mut res = vec![(i, j)];

    while (i, j) != start {
        if let Some(node) = path[i][j] {
            res.push(node);
            i = node.0;
            j = node.1;
        } else {
            println!("There is no path from start to end");
            return;
        }
    }

    for i in 0..path.len() {
        for j in 0..path[0].len() {
            if res.contains(&(i, j)) {
                print!("x");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

const INPUT: &str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
        crate::trace::init();
        assert_eq!(31, part1(INPUT).unwrap());
        assert_eq!(29, part2(INPUT).unwrap());
    }
}
