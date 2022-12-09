use std::collections::HashSet;

use itertools::Itertools;

use crate::day::Day;
use crate::input::read_input;

const INPUT_FILE: &str = "day9.txt";

#[derive(Clone, Copy)]
pub struct Day9 {}

impl Day for Day9 {
    fn title(&self) -> &'static str {
        "Day 9: Rope Bridge"
    }

    fn description(&self) -> &'static str {
        "
        We use a Rope struct and a set of coords (i64, i64).
        In both tasks we move the rope and then register its position in the set.
        Since sets only hold the same values once, all we have to do eventually is to just check the size of the set.

        In order to complete task 1, we created a Rope struct with head and tail. We implement the logic that moves the tail
        in accordance to the head.

        Task 2 introduces a longer rope. In order to support 10 knots, we update the Rope struct to have a vector of intermediate
        knots.

        Now, between moving the head and the tail, we iteratively move each intermediate knot in order. After each iteration, we make
        the most recent knot the \"head\" of the next one.

        That way, this struct works both in task 1 (with 0 intermediate knots), and in task 2 (with 8 intermediate knots).
        "
    }

    fn task_1(&self) -> String {
        let input = read_input(INPUT_FILE);
        let steps = parse_input_into_steps(&input);

        let mut rope = Rope::new(0);
        let mut set_of_visited_positions = HashSet::<Position>::from([rope.tail]);

        for step in steps {
            match step {
                Step::Left(by) => (0..by).for_each(|_| {
                    rope.left();
                    set_of_visited_positions.insert(rope.tail);
                }),
                Step::Up(by) => (0..by).for_each(|_| {
                    rope.up();
                    set_of_visited_positions.insert(rope.tail);
                }),
                Step::Right(by) => (0..by).for_each(|_| {
                    rope.right();
                    set_of_visited_positions.insert(rope.tail);
                }),
                Step::Down(by) => (0..by).for_each(|_| {
                    rope.down();
                    set_of_visited_positions.insert(rope.tail);
                }),
            }
        }

        let num_of_places_visited_by_tail = set_of_visited_positions.len();

        format!(
            "the tail visited {} unique locations",
            num_of_places_visited_by_tail
        )
    }

    fn task_2(&self) -> String {
        let input = read_input(INPUT_FILE);
        let steps = parse_input_into_steps(&input);

        let mut rope = Rope::new(8);
        let mut set_of_visited_positions = HashSet::<Position>::from([rope.tail]);

        for step in steps {
            match step {
                Step::Left(by) => (0..by).for_each(|_| {
                    rope.left();
                    set_of_visited_positions.insert(rope.tail);
                }),
                Step::Up(by) => (0..by).for_each(|_| {
                    rope.up();
                    set_of_visited_positions.insert(rope.tail);
                }),
                Step::Right(by) => (0..by).for_each(|_| {
                    rope.right();
                    set_of_visited_positions.insert(rope.tail);
                }),
                Step::Down(by) => (0..by).for_each(|_| {
                    rope.down();
                    set_of_visited_positions.insert(rope.tail);
                }),
            }
        }

        let num_of_places_visited_by_tail = set_of_visited_positions.len();

        format!(
            "the tail visited {} unique locations",
            num_of_places_visited_by_tail
        )
    }
}

type Position = (i64, i64);

struct Rope {
    head: Position,
    tail: Position,
    in_between_knots: Vec<Position>,
}

impl Rope {
    fn new(amount_of_in_between_knots: usize) -> Self {
        Self {
            head: (0, 0),
            tail: (0, 0),
            in_between_knots: vec![(0, 0); amount_of_in_between_knots],
        }
    }

    fn left(&mut self) {
        self.move_by(&(-1, 0))
    }

    fn up(&mut self) {
        self.move_by(&(0, -1))
    }

    fn right(&mut self) {
        self.move_by(&(1, 0))
    }

    fn down(&mut self) {
        self.move_by(&(0, 1))
    }

    fn move_by(&mut self, by: &Position) {
        self.head.0 += by.0;
        self.head.1 += by.1;

        let mut lead_position = &self.head;

        for knot in self.in_between_knots.iter_mut() {
            let new_knot_position = Rope::calculate_knot_position(knot, lead_position);

            knot.0 = new_knot_position.0;
            knot.1 = new_knot_position.1;

            lead_position = knot;
        }

        self.tail = Rope::calculate_knot_position(&self.tail, lead_position);
    }

    fn calculate_knot_position(knot: &Position, lead_position: &Position) -> Position {
        let (cur_lead_x, cur_lead_y) = lead_position.clone();

        match knot.clone() {
            (x, y) if y == cur_lead_y => {
                if (cur_lead_x - x).abs() > 1 {
                    let step = if cur_lead_x > x { 1 } else { -1 };
                    (x + step, y)
                } else {
                    (x, y)
                }
            }
            (x, y) if x == cur_lead_x => {
                if (cur_lead_y - y).abs() > 1 {
                    let step = if cur_lead_y > y { 1 } else { -1 };
                    (x, y + step)
                } else {
                    (x, y)
                }
            }
            (x, y) if (x - cur_lead_x).abs() == 1 && (y - cur_lead_y).abs() == 1 => (x, y),
            (x, y) => {
                let step_x = if cur_lead_x > x { 1 } else { -1 };
                let step_y = if cur_lead_y > y { 1 } else { -1 };
                (x + step_x, y + step_y)
            }
        }
    }
}

enum Step {
    Left(i64),
    Up(i64),
    Right(i64),
    Down(i64),
}

fn parse_input_into_steps(input: &String) -> Vec<Step> {
    input
        .lines()
        .map(|line| {
            match (
                line.split(" ").nth(0).unwrap(),
                line.split(" ").nth(1).unwrap(),
            ) {
                ("L", by) => Step::Left(by.parse::<i64>().unwrap()),
                ("U", by) => Step::Up(by.parse::<i64>().unwrap()),
                ("R", by) => Step::Right(by.parse::<i64>().unwrap()),
                ("D", by) => Step::Down(by.parse::<i64>().unwrap()),
                _ => panic!("unknown input"),
            }
        })
        .collect_vec()
}
