use std::cmp::Ordering;

use itertools::Itertools;
use regex::Regex;

use crate::day::Day;
use crate::input::input_for_day;

#[derive(Clone, Copy)]
pub struct Day15 {}

impl Day for Day15 {
    fn title(&self) -> &'static str {
        "Beacon Exclusion Zone"
    }

    fn description(&self) -> &'static str {
        "
        So we need to find, for a specific row, how much of it is actually covered by sensors and therefore cannot have the lost beacon.

        First thing that came to mind was to calculate the area of each sensor's range. But that would be costly and probably not what we need.
        Instead, what we really need is to figure out *which part of the given row is covered by the sensor*.

        So we create a Sensor struct that has a method which receives the desired row (2,000,000 for task 1). For each sensor we calculate
        where it covers this row by projecting which part of its area covers it.

        How? We calculate the distance between the row and the sensor's y position.
        Since a radius dictated by a manhattan distance creates a symmetrical diamond shape, we know that the farther we go from the center
        of the shape, the narrower it gets on the opposing axis (in our case, the farther we go on the y axis, the narrower we get on the x).

        Therefore, we calculate [sensor.manhattan_radius - abs(row - sensor.y)]. This creates the margin, and therefore the range covered
        by the sensor at the row is [sensor.x - margin, sensor.x + margin]. (if the row's distance is larger than the manhattan distance,
        this of course means that our row is beyond the sensor's reach).

        Finally, we created a Coverage struct, that handles lists of ranges. It knows to merge overlapping ranges and sort them.
        This prevents us from counting overlapping ranges once we get to sum the coverage of our row.

        For task 1: we simply run a single iteration over row 2,000,000, get the coverage, subtract the # of beacons that can be found on
        that row, and that's it.

        For task 2: we actually iterate from 0 to 4,000,000, and for each row, run the coverage calculation we described above.
        We then look for the first row where *any* of the ranges in the coverage actually ends within 0 to 4,000,000 (the puzzle assures there's only 1).

        I was afraid that task 2 would take really long, but it actually takes less than a second to complete on my M1 Mac, which is nice!
        "
    }

    fn task_1(&self) -> String {
        let input = input_for_day(15);
        let row = 2_000_000;
        let sensors = parse_input_into_sensors(&input);

        let area_covered_by_sensors = get_coverage_for_row_with_sensors(row, &sensors);

        let beacons_in_row = sensors
            .iter()
            .map(|sensor| sensor.connected_to_beacon)
            .filter(|beacon| beacon.1 == row)
            .dedup()
            .filter(|beacon| area_covered_by_sensors.contains(beacon.0))
            .count();

        let positions_where_beacons_cannot_be_found =
            area_covered_by_sensors.total_coverage() - beacons_in_row as u64;

        format!(
            "there are {} positions where the distress beacon could not be found",
            positions_where_beacons_cannot_be_found
        )
    }

    fn task_2(&self) -> String {
        let input = input_for_day(15);
        let sensors = parse_input_into_sensors(&input);

        let only_position_for_distress_beacon = (0..=4_000_000)
            .map(|row| {
                let coverage = get_coverage_for_row_with_sensors(row, &sensors);

                if let Some(range_that_ends_within_bounds) = coverage
                    .ranges
                    .iter()
                    .find(|r| 0 <= r.1 && r.1 <= 4_000_000)
                {
                    Some((range_that_ends_within_bounds.1 + 1, row))
                } else {
                    None
                }
            })
            .find(|coords| coords.is_some())
            .unwrap()
            .unwrap();

        format!(
            "the only position where the distress signal can come from is at {:?}",
            only_position_for_distress_beacon
        )
    }
}

#[derive(Debug)]
struct Sensor {
    x: i64,
    y: i64,
    connected_to_beacon: (i64, i64),
    manhattan_radius: i64,
}

impl Sensor {
    fn get_coverage_at(&self, row: i64) -> Option<CoverageRange> {
        let dist = self.y.abs_diff(row) as i64;

        if dist <= self.manhattan_radius {
            let margin = self.manhattan_radius - dist;
            Some((self.x - margin, self.x + margin))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Coverage {
    ranges: Vec<CoverageRange>,
}

impl Coverage {
    fn new() -> Self {
        Self { ranges: vec![] }
    }

    fn add_range(&mut self, range: CoverageRange) {
        let range_containing_start = self
            .ranges
            .iter()
            .enumerate()
            .find(|(_, r)| r.0 <= range.0 && range.0 <= r.1);
        let range_containing_ending = self
            .ranges
            .iter()
            .enumerate()
            .find(|(_, r)| r.0 <= range.1 && range.1 <= r.1);

        let final_merged_range = match (range_containing_start, range_containing_ending) {
            (Some((i1, _)), Some((i2, r2))) => {
                self.ranges[i1].1 = r2.1;

                if i1 != i2 {
                    self.ranges.remove(i2);
                }

                self.ranges[i1].clone()
            }
            (Some((i, r)), None) | (None, Some((i, r))) => {
                let (old_start, old_finish) = r.clone();
                self.ranges[i].0 = old_start.min(range.0);
                self.ranges[i].1 = old_finish.max(range.1);
                self.ranges[i].clone()
            }
            (None, None) => {
                self.ranges.push(range);
                range.clone()
            }
        };

        self.ranges = self
            .ranges
            .iter()
            .map(|r| r.clone())
            .filter(|r| !(final_merged_range.0 < r.0 && r.1 < final_merged_range.1))
            .sorted_by(|a, b| {
                if a.1 < b.0 {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            })
            .collect_vec();
    }

    fn contains(&self, x: i64) -> bool {
        self.ranges.iter().any(|r| r.0 <= x && x <= r.1)
    }

    fn total_coverage(&self) -> u64 {
        self.ranges.iter().map(|r| (r.1 - r.0) as u64 + 1).sum()
    }
}

type CoverageRange = (i64, i64);

fn parse_input_into_sensors(input: &String) -> Vec<Sensor> {
    input.lines().map(|line| {
        let coords_matcher = Regex::new(r"^.+x=(?P<sensor_x>[^,]+), y=(?P<sensor_y>[^:]+).+x=(?P<beacon_x>[^,]+), y=(?P<beacon_y>.+)$").unwrap();
        let caps = coords_matcher.captures(line).unwrap();

        let sensor_x = caps.name("sensor_x").map(|s| s.as_str().parse::<i64>()).unwrap().unwrap();
        let sensor_y = caps.name("sensor_y").map(|s| s.as_str().parse::<i64>()).unwrap().unwrap();
        let beacon_x = caps.name("beacon_x").map(|s| s.as_str().parse::<i64>()).unwrap().unwrap();
        let beacon_y = caps.name("beacon_y").map(|s| s.as_str().parse::<i64>()).unwrap().unwrap();

        let manhattan_distance = (sensor_x.abs_diff(beacon_x) + sensor_y.abs_diff(beacon_y)) as i64;

        Sensor {
            x: sensor_x,
            y: sensor_y,
            connected_to_beacon: (beacon_x, beacon_y),
            manhattan_radius: manhattan_distance
        }
    }).collect_vec()
}

fn get_coverage_for_row_with_sensors(row: i64, sensors: &Vec<Sensor>) -> Coverage {
    let mut area_covered_by_sensors = Coverage::new();

    for sensor in sensors.iter() {
        if let Some(range) = sensor.get_coverage_at(row) {
            area_covered_by_sensors.add_range(range);
        }
    }

    area_covered_by_sensors
}
