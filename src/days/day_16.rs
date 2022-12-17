use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use regex::Regex;

use crate::day::Day;
use crate::input::input_for_day;

#[derive(Clone, Copy)]
pub struct Day16 {}

impl Day for Day16 {
    fn title(&self) -> &'static str {
        "Proboscidea Volcanium"
    }

    fn description(&self) -> &'static str {
        "
        Today we're brute forcing. But before we do, we need to optimize the data.

        First, we get an input that describes a graph. The graph contains valves - some work, some don't.
        The mission is to find the max amount of pressure we can release in 30 minutes, which means that both the path AND the order
        matter (opening A then B will not necessarily yield the same pressure as B then A).

        The naive solution would be to simulate every possible permutation of the graph and find the maximum.
        
        That, of course, is not doable, as the runtime of the algorithm is exponential, and we will find ourselves running forever, yielding no result.

        So we're going to leave the runtime exponential, but SIGNIFICANTLY reduce the input.
        Instead of 51 valves, we are only going to keep the ones that actually WORK (in my input that's 15).

        We are going to calculate the cost in minutes for traversing between any two WORKING valves ahead of time, by running BFS
        from any working valve to any working valve.

        Now that we have the reduced input, we brute force every possible sequence of valves that fits into the 30 minutes budget.
        This approach takes 2s to complete, despite the fact that we actually run a huge amount of permutations.

        For task 2, it's brute force time again, but this time we're trying out every possible division of the work between us and
        the elephant. NOTE that it doesn't matter who does which part of the work, which means that we don't have to try every possible permutation, only half of them ((14, 1) is the same is (1, 14)).

        This still takes a LONG time, but it's manageable. Off to see how others solved this hell of a problem :)
        "
    }

    fn task_1(&self) -> String {
        let input = input_for_day(16);
        let valves = parse_input_into_valves(&input);

        let working_valves = valves
            .clone()
            .into_iter()
            .filter(|valve| valve.id == "AA" || valve.flow_rate > 0)
            .collect_vec();

        let distances = find_distances_between_valves(&valves);

        let map = ValveMap::from(&working_valves, &distances);

        let max_score = find_max_score(vec!["AA".to_string()], &map, 30);

        format!(
            "the maximum amount of pressure we can release is {}",
            max_score
        )
    }

    fn task_2(&self) -> String {
        let input = input_for_day(16);
        let valves = parse_input_into_valves(&input);

        let working_valves = valves
            .clone()
            .into_iter()
            .filter(|valve| valve.flow_rate > 0)
            .collect_vec();
        let relevant_valves = valves
            .clone()
            .into_iter()
            .filter(|valve| valve.id == "AA" || valve.flow_rate > 0)
            .collect_vec();

        let distances = find_distances_between_valves(&valves);

        fn explore_division_of_labor(
            my_valves: HashSet<String>,
            elephant_valves: HashSet<String>,
            working_valves: &Vec<Valve>,
            distances: &HashMap<String, u8>,
        ) -> u64 {
            let my_map = ValveMap::from(
                &working_valves
                    .iter()
                    .filter(|v| v.id == "AA" || my_valves.contains(&v.id))
                    .map(|v| v.clone())
                    .collect_vec(),
                distances,
            );

            let elephant_map = ValveMap::from(
                &working_valves
                    .iter()
                    .filter(|v| v.id == "AA" || elephant_valves.contains(&v.id))
                    .map(|v| v.clone())
                    .collect_vec(),
                distances,
            );

            find_max_score(vec!["AA".to_string()], &my_map, 26)
                + find_max_score(vec!["AA".to_string()], &elephant_map, 26)
        }

        let max_score =
            (0..=working_valves.len() / 2).fold(0 as u64, |cur_max, elephant_work_size| {
                println!(
                    "my work: {} elephant work: {}",
                    working_valves.len() - elephant_work_size,
                    elephant_work_size
                );

                cur_max.max(
                    working_valves
                        .iter()
                        .map(|v| &v.id)
                        .permutations(elephant_work_size)
                        .fold(0, |cur_max, perm| {
                            cur_max.max(explore_division_of_labor(
                                HashSet::from_iter(
                                    working_valves
                                        .iter()
                                        .filter(|v| !perm.contains(&&v.id))
                                        .map(|v| v.id.clone()),
                                ),
                                HashSet::from_iter(
                                    working_valves
                                        .iter()
                                        .filter(|v| perm.contains(&&v.id))
                                        .map(|v| v.id.clone()),
                                ),
                                &relevant_valves,
                                &distances,
                            ))
                        }),
                )
            });

        format!(
            "the maximum pressure we can release together with an elephant is {}",
            max_score
        )
    }
}

#[derive(PartialEq)]
struct ValveMap {
    valves: HashMap<String, Valve>,
    distances: HashMap<String, u8>,
}

impl ValveMap {
    fn from(valves: &Vec<Valve>, distances: &HashMap<String, u8>) -> Self {
        Self {
            valves: valves
                .clone()
                .into_iter()
                .map(|v| (v.id.clone(), v))
                .collect(),
            distances: distances.clone(),
        }
    }

    fn get_distance(&self, from: &String, to: &String) -> u8 {
        self.distances[&format!("{}->{}", from, to)]
    }

    fn get_flow_rate(&self, valve: &String) -> u8 {
        self.valves[valve].flow_rate
    }

    fn list_all_valves(&self) -> Vec<String> {
        self.valves.keys().map(|k| k.clone()).collect_vec()
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Valve {
    id: String,
    flow_rate: u8,
    leads_to: HashSet<String>,
}

fn parse_input_into_valves(input: &String) -> Vec<Valve> {
    let matcher = Regex::new(
        r"^Valve (?P<valve_id>\w\w) .+ rate=(?P<flow_rate>\d+); .+ valves? (?P<leads_to_valves>.+)$",
    )
    .unwrap();

    input
        .lines()
        .map(|line| matcher.captures(line).unwrap())
        .map(|caps| Valve {
            id: caps.name("valve_id").unwrap().as_str().to_string(),
            flow_rate: caps
                .name("flow_rate")
                .unwrap()
                .as_str()
                .parse::<u8>()
                .unwrap(),
            leads_to: caps
                .name("leads_to_valves")
                .unwrap()
                .as_str()
                .split(", ")
                .map(|s| s.to_string())
                .collect(),
        })
        .collect_vec()
}

fn find_distances_between_valves(valves: &Vec<Valve>) -> HashMap<String, u8> {
    let valves_graph: HashMap<String, &Valve> = valves.iter().map(|v| (v.id.clone(), v)).collect();

    fn find_distance(graph: &HashMap<String, &Valve>, from: &String, to: &String) -> u8 {
        let mut q = VecDeque::from([(0 as u8, from.clone())]);
        let mut visited = HashSet::from([from.clone()]);

        while let Some((distance, node_id)) = q.pop_front() {
            if node_id == *to {
                return distance;
            }

            graph[&node_id].leads_to.iter().for_each(|connected| {
                if !visited.contains(connected) {
                    visited.insert(connected.clone());
                    q.push_back((distance + 1, connected.clone()));
                }
            });
        }

        panic!("should not get here");
    }

    valves_graph
        .values()
        .permutations(2)
        .map(|nodes| (nodes[0], nodes[1]))
        .flat_map(|(a, b)| {
            let d = find_distance(&valves_graph, &a.id, &b.id);
            [
                (format!("{}->{}", a.id, b.id), d),
                (format!("{}->{}", b.id, a.id), d),
            ]
        })
        .collect()
}

fn find_max_score(cur_path: Vec<String>, map: &ValveMap, minutes_left: u8) -> u64 {
    let cur_valve = cur_path.last().unwrap().clone();
    let possible_next_valves = map
        .list_all_valves()
        .iter()
        .filter(|v| !cur_path.contains(v))
        .filter(|v| map.get_distance(&cur_valve, v) + 1 <= minutes_left)
        .map(|v| v.clone())
        .collect_vec();

    let my_score = map.get_flow_rate(&cur_valve) as u64 * minutes_left as u64;
    let path_with_highest_score = possible_next_valves
        .iter()
        .map(|next| {
            find_max_score(
                [cur_path.clone(), vec![next.clone()]].concat(),
                map,
                minutes_left - map.get_distance(&cur_valve, &next) - 1,
            )
        })
        .max();

    my_score as u64 + path_with_highest_score.unwrap_or(0)
}
