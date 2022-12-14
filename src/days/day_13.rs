use std::cmp::Ordering;

use itertools::Itertools;
use json::{self, array, JsonValue};

use crate::day::Day;
use crate::input::input_for_day;

#[derive(Clone, Copy)]
pub struct Day13 {}

impl Day for Day13 {
    fn title(&self) -> &'static str {
        "Distress Signal"
    }

    fn description(&self) -> &'static str {
        "
        First, the parsing: today we use a sneaky way to parse the input... we just parse each line as a JSON array!
        Next thing was to implement a comparator function that follows the instruction.

        For task 1, we parse the input in pairs, and run the comparator function over each pair in order to identify
        the ones in the right order.

        For task 2, we simply parse all pairs, and use the native sort function with our existing comparator function from task 1.
        "
    }

    fn task_1(&self) -> String {
        let input = input_for_day(13);

        let pairs = input
            .lines()
            .chunks(3)
            .into_iter()
            .map(|mut chunk| {
                (
                    json::parse(chunk.next().unwrap()).unwrap(),
                    json::parse(chunk.next().unwrap()).unwrap(),
                )
            })
            .collect_vec();

        let sum_of_indices_of_pairs_in_right_order = pairs
            .iter()
            .enumerate()
            .map(|(i, (l, r))| (i, cmp_packets(&l, &r)))
            .filter(|(_, cmp)| cmp.is_lt())
            .map(|(i, _)| i + 1)
            .sum::<usize>();

        format!(
            "sum of indices of pairs in right order is {}",
            sum_of_indices_of_pairs_in_right_order
        )
    }

    fn task_2(&self) -> String {
        let input = input_for_day(13);
        let mut packets = input
            .lines()
            .filter(|ln| ln.len() > 0)
            .map(|ln| json::parse(ln).unwrap())
            .collect_vec();

        let first_packet = array![array![2]];
        packets.push(first_packet.clone());

        let second_packet = array![array![6]];
        packets.push(second_packet.clone());

        packets.sort_by(cmp_packets);

        let index_of_first_packet = packets
            .iter()
            .enumerate()
            .find(|(_, packet)| packet.clone().eq(&first_packet))
            .map(|(i, _)| i + 1)
            .unwrap();

        let index_of_second_packet = packets
            .iter()
            .enumerate()
            .find(|(_, packet)| packet.clone().eq(&second_packet))
            .map(|(i, _)| i + 1)
            .unwrap();

        format!(
            "index of first packet is {}, of second is {}, their product is {}",
            index_of_first_packet,
            index_of_second_packet,
            index_of_first_packet * index_of_second_packet
        )
    }
}

fn cmp_packets(left: &JsonValue, right: &JsonValue) -> Ordering {
    if left.is_null() {
        return Ordering::Less;
    }

    if right.is_null() {
        return Ordering::Greater;
    }

    if let (Some(left_num), Some(right_num)) = (left.as_i64(), right.as_i64()) {
        return left_num.cmp(&right_num);
    }

    let left_ar = if left.is_array() {
        left.clone()
    } else {
        array![left.as_i64().unwrap()]
    };
    let right_ar = if right.is_array() {
        right.clone()
    } else {
        array![right.as_i64().unwrap()]
    };

    for i in 0..left_ar.len().max(right_ar.len()) {
        let order = cmp_packets(&left_ar[i], &right_ar[i]);
        if order == Ordering::Less || order == Ordering::Greater {
            return order;
        }
    }

    Ordering::Equal
}
