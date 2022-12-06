use itertools::Itertools;
use std::collections::HashSet;

use crate::day::Day;
use crate::input::read_input;

const INPUT_FILE: &str = "day6.txt";

#[derive(Clone, Copy)]
pub struct Day6 {}

impl Day for Day6 {
    fn title(&self) -> &'static str {
        "Tuning Trouble"
    }

    fn description(&self) -> &'static str {
        "
        In order to decode the message, we introduce a custom iterator called RollingStringIterator.

        This iterator accepts a string and buffer size, and with each iteration returns a substring that
        begins with the next index, and is as long as the given buffer size.

        For the first question, we run through this iterator with a buffer size of 4. For each iteration,
        we check that all characters in the buffer are unique.

        For the second question we do the same, but with a buffer with 14 characters instead of 4.

        In order to make the uniqueness check efficient, we use a HashSet to determine if there are any
        repeating characters. The use of HashSet makes this check an O(n) in average.
        "
    }

    fn task_1(&self) -> String {
        let input = read_input(INPUT_FILE);

        let char_count_until_packet_start = RollingStringIterator::new(input, 4)
            .enumerate()
            .find(|(_, four_chars)| all_unique(&four_chars))
            .map(|(i, _)| i)
            .expect("");

        let first_char_in_message = char_count_until_packet_start + 4;

        format!(
            "there are {} characters before the first start-of-packet",
            first_char_in_message
        )
    }

    fn task_2(&self) -> String {
        let input = read_input(INPUT_FILE);

        let char_count_until_packet_start = RollingStringIterator::new(input, 14)
            .enumerate()
            .find(|(_, fourteen_chars)| all_unique(&fourteen_chars))
            .map(|(i, _)| i)
            .expect("");

        let first_char_in_message = char_count_until_packet_start + 14;

        format!(
            "there are {} characters before the first start-of-message",
            first_char_in_message
        )
    }
}

struct RollingStringIterator {
    string: String,
    index: usize,
    buffer_size: usize,
}

impl RollingStringIterator {
    fn new(string: String, buffer_size: usize) -> Self {
        Self {
            string,
            index: 0,
            buffer_size,
        }
    }
}

impl Iterator for RollingStringIterator {
    type Item = Vec<char>;

    fn next(&mut self) -> Option<Self::Item> {
        let substr = self.string[self.index..(self.index + self.buffer_size)]
            .chars()
            .collect_vec();

        if substr.len() > 0 {
            self.index += 1;
            Some(substr)
        } else {
            None
        }
    }
}

fn all_unique(chars: &Vec<char>) -> bool {
    let mut known_values_set = HashSet::<&char>::new();

    for val in chars.iter() {
        if known_values_set.contains(val) {
            return false;
        }

        known_values_set.insert(val);
    }

    true
}
