use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::day::Day;
use crate::input::input_for_day;

#[derive(Clone, Copy)]
pub struct Day12 {}

impl Day for Day12 {
    fn title(&self) -> &'static str {
        "Hill Climbing Algorithm"
    }

    fn description(&self) -> &'static str {
        "
        BFS Day :)

        We need to identify the shortest way from S to E over our grid. We have several rules that define
        whether or not we can move between any two adjacent spots on the grid.

        Out of that, we can define a directed graph, where each position on the grid is a vertex. The edges are defined 
        as following:
        1. letter to the following one - edge
        2. letter to any letter that comes before it in the alphabet - edge
        3. S is considered as a and E is considered as z
        4. Otherwise, no edge

        Given these constraints, we just implement a plain and simple BFS algorithm that returns the distance between S and E.

        As for part 2 - what we can do is find the path from E to the nearest a using BFS. Basically, we're going to build the path
        backwards. One thing to remember is that since the graph is directed, we need to also reverse the edges, that is:
        allow going down only once, but allow 'climbing' as many letters as we want.
        "
    }

    fn task_1(&self) -> String {
        let input = input_for_day(12);
        let grid = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        let distance = bfs(&grid, &'S', &'E', &is_traversable);

        format!("the shortest path to the exit is {}", distance)
    }

    fn task_2(&self) -> String {
        let input = input_for_day(12);
        let grid = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        let distance = bfs(&grid, &'E', &'a', &is_traversable_reverse);

        format!(
            "the shortest hiking trail from any 'a' spot is {}",
            distance
        )
    }
}

type Coords = (usize, usize);

fn bfs(
    grid: &Vec<Vec<char>>,
    from: &char,
    to: &char,
    edge_discovery_fn: &dyn Fn(char, char) -> bool,
) -> usize {
    let start_coords = grid
        .iter()
        .enumerate()
        .find(|(_, row)| row.contains(from))
        .map(|(y, row)| (row.iter().position(|c| c == from).unwrap(), y))
        .expect("could not find coords of the starting point in the grid");

    let mut discovered = HashSet::<Coords>::from([start_coords]);
    let mut to_visit = vec![start_coords];
    let mut dist = HashMap::<Coords, usize>::from([(start_coords, 0)]);

    while !to_visit.is_empty() {
        let next_node = to_visit.remove(0);
        let value_at_node = grid[next_node.1][next_node.0];
        let dist_of_node = dist[&next_node];

        if value_at_node == *to {
            return dist[&next_node];
        }

        let neighbors = vec![
            ((next_node.0 as i32 - 1) as usize, next_node.1),
            (next_node.0 + 1, next_node.1),
            (next_node.0, (next_node.1 as i32 - 1) as usize),
            (next_node.0, next_node.1 + 1),
        ];

        neighbors
            .iter()
            .filter(|n| n.0 < grid[0].len() && n.1 < grid.len())
            .filter(|n| edge_discovery_fn(value_at_node, grid[n.1][n.0]))
            .filter(|n| !discovered.contains(*n))
            .collect_vec()
            .iter()
            .for_each(|n| {
                discovered.insert(*n.clone());
                to_visit.push(*n.clone());
                dist.insert(*n.clone(), dist_of_node + 1);
            });
    }

    panic!("no way out");
}

fn is_traversable(a: char, b: char) -> bool {
    let l_val = match a {
        'S' => 'a'.to_digit(36).unwrap(),
        'E' => 'z'.to_digit(36).unwrap(),
        other => other.to_digit(36).unwrap(),
    };

    let r_val = match b {
        'S' => 'a'.to_digit(36).unwrap(),
        'E' => 'z'.to_digit(36).unwrap(),
        other => other.to_digit(36).unwrap(),
    };

    r_val < l_val || r_val - l_val <= 1
}

fn is_traversable_reverse(a: char, b: char) -> bool {
    let l_val = match a {
        'S' => 'a'.to_digit(36).unwrap(),
        'E' => 'z'.to_digit(36).unwrap(),
        other => other.to_digit(36).unwrap(),
    };

    let r_val = match b {
        'S' => 'a'.to_digit(36).unwrap(),
        'E' => 'z'.to_digit(36).unwrap(),
        other => other.to_digit(36).unwrap(),
    };

    r_val > l_val || l_val - r_val <= 1
}
