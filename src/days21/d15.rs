use crate::Result;

fn get_adjacent(i: usize, j: usize, m: usize, n: usize) -> Vec<(usize, usize)> {
    [(0, 1), (1, 0), (-1, 0), (0, -1)]
        .iter()
        .map(|&(x, y)| (i as isize + x, j as isize + y))
        .filter(|&(x, y)| {
            let (m, n) = (m as isize, n as isize);
            x >= 0 && x < m && y >= 0 && y < n
        })
        .map(|(x, y)| (x as usize, y as usize))
        .collect()
}

fn read(input: &str) -> Vec<Vec<Node>> {
    let (m, n) = (
        input.lines().count(),
        input.lines().next().unwrap().chars().count(),
    );
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| Node {
                    val: c.to_digit(10).unwrap(),
                    neighbors: get_adjacent(i, j, m, n),
                })
                .collect()
        })
        .collect()
}

fn read2(input: &str) -> Vec<Vec<Node>> {
    let m = input.lines().count() * 5;
    let n = input.lines().next().unwrap().chars().count() * 5;

    let tile: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let (rows, cols) = (m / 5, n / 5);

    (0..m)
        .into_iter()
        .map(|i| {
            (0..n)
                .into_iter()
                .map(|j| {
                    let next = tile[i % rows][j % cols] as usize + i / rows + j / cols - 1;
                    let neighbors = get_adjacent(i, j, m, n);
                    Node {
                        neighbors,
                        val: (next % 9 + 1) as u32,
                    }
                })
                .collect()
        })
        .collect()
}

use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug)]
struct Node {
    neighbors: Vec<(usize, usize)>,
    val: u32,
}

fn djikstra(graph: Vec<Vec<Node>>) -> u32 {
    // Given a graph of nodes `graph`, and a final node `(m,n)`:
    let (m, n) = (graph.len(), graph[0].len());
    let end = (m - 1, n - 1);

    // Give each node an initial distance-from-start value of infinity, except the initial node.
    let mut distances = vec![vec![u32::MAX; n]; m];
    distances[0][0] = 0;

    let mut path: Vec<Vec<Option<(usize, usize)>>> = vec![vec![None; n]; m];
    let mut queue = BinaryHeap::new();

    // Set the initial node as current.
    queue.push(Reverse((distances[0][0], (0, 0))));

    while let Some(Reverse((current_dist, current))) = queue.pop() {
        let (x, y) = current;
        let node = &graph[x][y];

        // If we reach the destination OR the smallest distance is infinity, stop.
        if current == end || distances[x][y] == u32::MAX {
            break;
        }

        // For each node, consider all unvisited neighbors and calculate their distance.
        // This is the minimum of (distances[neighbor], distances[current] + edge)
        for &(i, j) in &node.neighbors {
            let edge = &graph[i][j].val; // Per the problem, we consider the target node's value our edge.
            let new_distance = distances[x][y] + edge;

            // If new_distance < the neighbor's difference, by definition this is the shortest
            // path from current to neighbor at this moment in time. Update the value accordingly.
            if new_distance < distances[i][j] {
                distances[i][j] = new_distance;

                // Save this in our queue - the next node searched will be the one with the shortest distance
                queue.push(Reverse((new_distance, (i, j))));

                // We can also save this information to reconstruct the actual path later.
                path[i][j] = Some(current);
            }
        }
    }

    // If desired, we can reconstruct the path by going backwards through indices.
    reconstruct_path(&path);

    if let Some(tup) = path[m - 1][n - 1] {
        return distances[m - 1][n - 1];
    }
    0
}

fn reconstruct_path(path: &[Vec<Option<(usize, usize)>>]) {
    let (mut i, mut j) = (path.len() - 1, path[0].len() - 1);
    let mut res = vec![(i, j)];

    while (i, j) != (0, 0) {
        if let Some(node) = path[i][j] {
            res.push(node);
            i = node.0;
            j = node.1;
        } else {
            println!("There is no path from start to end");
            return;
        }
    }
    println!("{:?}", res);
}

pub fn part1(input: &str) -> Result<u32> {
    let graph = read(input);
    let min = djikstra(graph);

    Ok(min)
}

pub fn part2(input: &str) -> Result<u32> {
    let graph = read2(input);
    let min = djikstra(graph);

    Ok(min)
}

const INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

#[cfg(test)]
mod tests {
    use super::*;
    use eyre_test::test;

    #[test]
    fn test1() {
        assert_eq!(40, part1(INPUT).unwrap());
    }
    #[test]
    fn test2() {
        assert_eq!(315, part2(INPUT).unwrap());
    }
}
