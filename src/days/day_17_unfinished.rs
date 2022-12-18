use std::collections::{HashMap, HashSet, VecDeque};

use bitmaps::Bitmap;
use itertools::Itertools;

use crate::day::Day;
use crate::input::input_for_day;

#[derive(Clone, Copy)]
pub struct Day17 {}

impl Day for Day17 {
    fn title(&self) -> &'static str {
        "Pyroclastic Flow"
    }

    fn description(&self) -> &'static str {
        "
        Task 1: Simulate Tetris
        Task 2: Simulate Tetris until you identify a recurring pattern, then stop simulating and just project the results.

        I did some steps manually today, though, so the implementation for task 2 stands unfinished and therefore unusable.
        "
    }

    fn task_1(&self) -> String {
        let input = input_for_day(17);

        let mut rock_formations = [
            ["####", "", "", ""],
            [".#.", "###", ".#.", ""],
            ["..#", "..#", "###", ""],
            ["#", "#", "#", "#"],
            ["##", "##", "", ""],
        ]
        .map(Rock::from)
        .into_iter()
        .cycle();

        let mut jet_stream = input
            .chars()
            .map(|c| if c == '<' { Jet::Left } else { Jet::Right })
            .cycle();

        let mut top: isize = 0;
        let mut rested_rocks = VecDeque::<Rock>::new();

        for _ in 0..2022 {
            let rock_blueprint = rock_formations.next().unwrap();
            let rock_position: Position = (2, top - 3 - rock_blueprint.height());

            let mut rock = rock_blueprint.clone_to_position(rock_position);
            loop {
                let jet = jet_stream.next();

                match jet {
                    Some(Jet::Left) => {
                        if rock.is_in_bounds(1, 7) {
                            rock.position.0 -= 1;
                            if rested_rocks.iter().any(|r| r.hits(&rock)) {
                                rock.position.0 += 1;
                            }
                        }
                    }
                    Some(Jet::Right) => {
                        if rock.is_in_bounds(0, 6) {
                            rock.position.0 += 1;
                            if rested_rocks.iter().any(|r| r.hits(&rock)) {
                                rock.position.0 -= 1;
                            }
                        }
                    }
                    None => panic!("should never get here, jet is a cyclic iterator"),
                }

                rock.position.1 += 1;
                if rock.position.1 == 1 - rock.height()
                    || rested_rocks.iter().any(|r| r.hits(&rock))
                {
                    rock.position.1 -= 1;
                    break;
                }
            }

            top = top.min(rock.position.1);

            rested_rocks.push_back(rock);

            if rested_rocks.len() > 200 {
                rested_rocks.pop_front();
            }
        }

        format!("the highest point in the stack is {}", -top)
    }

    fn task_2(&self) -> String {
        let input = input_for_day(17);

        let mut rock_formations = [
            ["####", "", "", ""],
            [".#.", "###", ".#.", ""],
            ["..#", "..#", "###", ""],
            ["#", "#", "#", "#"],
            ["##", "##", "", ""],
        ]
        .map(Rock::from)
        .into_iter()
        .enumerate()
        .cycle()
        .peekable();

        let mut jet_stream = input
            .chars()
            .map(|c| if c == '<' { Jet::Left } else { Jet::Right })
            .enumerate()
            .cycle()
            .peekable();

        let mut top: isize = 0;
        let mut rested_rocks = VecDeque::<Rock>::new();
        let mut observed_patterns = HashMap::<(usize, usize, u8), isize>::new();

        for round in 0..1000000000000 as usize {
            let (_, rock_blueprint) = rock_formations.next().unwrap();
            let rock_position: Position = (2, top - 3 - rock_blueprint.height());

            let mut rock = rock_blueprint.clone_to_position(rock_position);
            loop {
                let jet = jet_stream.next();

                match jet {
                    Some((_, Jet::Left)) => {
                        if rock.is_in_bounds(1, 7) {
                            rock.position.0 -= 1;
                            if rested_rocks.iter().any(|r| r.hits(&rock)) {
                                rock.position.0 += 1;
                            }
                        }
                    }
                    Some((_, Jet::Right)) => {
                        if rock.is_in_bounds(0, 6) {
                            rock.position.0 += 1;
                            if rested_rocks.iter().any(|r| r.hits(&rock)) {
                                rock.position.0 -= 1;
                            }
                        }
                    }
                    None => panic!("should never get here, jet is a cyclic iterator"),
                }

                rock.position.1 += 1;
                if rock.position.1 == 1 - rock.height()
                    || rested_rocks.iter().any(|r| r.hits(&rock))
                {
                    rock.position.1 -= 1;
                    break;
                }
            }

            top = top.min(rock.position.1);

            rested_rocks.push_back(rock);

            let top_row_encoding: (usize, usize, u8) = (
                jet_stream.peek().unwrap().0,
                rock_formations.peek().unwrap().0,
                (0..7).fold(0, |prev, x| {
                    if rested_rocks.iter().any(|r| r.is_within(&(x, top))) {
                        prev << 1 | 1
                    } else {
                        prev << 1
                    }
                }),
            );

            if let Some(top_before) = observed_patterns.get(&top_row_encoding)
            // && jet_stream.peek().unwrap().0 == 0
            // && rock_formations.peek().unwrap().0 == 0
            {
                println!("boom! observed pattern at {}", top);
                println!("it was observed before at {}", top_before);
                println!("the pattern: {:?}", top_row_encoding);
                break;
            } else {
                observed_patterns.insert(top_row_encoding, top);
            }

            if rested_rocks.len() > 200 {
                rested_rocks.pop_front();
            }
        }

        format!("the highest point in the stack is {}", -top)
    }
}

#[derive(Clone, Debug)]
struct Rock {
    area: Vec<Bitmap<4>>,
    position: Position,
}

impl Rock {
    fn from(visual: [&str; 4]) -> Self {
        let area = visual
            .iter()
            .filter(|ln| !ln.is_empty())
            .map(|ln| {
                let mut bitmap = Bitmap::<4>::new();

                ln.chars().enumerate().for_each(|(i, c)| {
                    if c == '#' {
                        bitmap.set(i, true);
                    }
                });

                bitmap
            })
            .collect_vec();

        Self {
            area,
            position: (0, 0),
        }
    }

    fn clone_to_position(&self, position: Position) -> Self {
        Self {
            area: self.area.clone(),
            position,
        }
    }

    fn height(&self) -> isize {
        self.area.len() as isize
    }

    fn width(&self) -> isize {
        ((0..4)
            .rev()
            .find(|w| self.area.iter().any(|bm| bm.get(*w)))
            .unwrap()
            + 1) as isize
    }

    fn hits(&self, other: &Self) -> bool {
        for x in 0..self.width() {
            for y in 0..self.height() {
                if self.is_within(&(self.position.0 + x, self.position.1 + y))
                    && other.is_within(&(self.position.0 + x, self.position.1 + y))
                {
                    return true;
                }
            }
        }

        false
    }

    fn is_within(&self, p: &Position) -> bool {
        p.0 >= self.position.0
            && p.0 < self.position.0 + self.width()
            && p.1 >= self.position.1
            && p.1 < self.position.1 + self.height()
            && self.area[(p.1 - self.position.1) as usize].get((p.0 - self.position.0) as usize)
    }

    fn is_in_bounds(&self, from: isize, to: isize) -> bool {
        self.position.0 >= from && self.position.0 + self.width() <= to
    }
}

type Position = (isize, isize);

#[derive(Clone)]
enum Jet {
    Right,
    Left,
}
