use crate::day::Day;
use crate::input::read_input;
use bitmaps::Bitmap;
use itertools::Itertools;

const INPUT_FILE: &str = "day3.txt";

#[derive(Clone, Copy)]
pub struct Day3 {}

impl Day for Day3 {
    fn title(&self) -> &'static str {
        "Rucksack Reorganization"
    }

    fn task_1(&self) -> String {
        let input = read_input(INPUT_FILE);

        let rucksacks = input.lines().map(parse_line_into_rucksack);

        let duplicate_items = rucksacks
            .map(|rucksack| get_item_that_shows_in_both_compartments_of_a_rucksack(&rucksack));

        let sum_of_duplicate_items: u32 = duplicate_items.map(|item| item.score).sum();

        format!(
            "The sum of all duplicate items is {}",
            sum_of_duplicate_items
        )
    }

    fn task_2(&self) -> String {
        let input = read_input(INPUT_FILE);

        let rucksacks = input.lines().map(parse_line_into_rucksack);

        let mut sum_of_shared_items = 0;
        let mut rucksacks_iter = rucksacks.into_iter().peekable();

        while rucksacks_iter.peek().is_some() {
            let (elf1, elf2, elf3) = rucksacks_iter.next_tuple().expect("expected input to have trios of elf rucksacks, but for some reason could not take three elves at a time");

            let shared_item = get_item_shared_between_three_rucksacks((&elf1, &elf2, &elf3));
            sum_of_shared_items += shared_item.score;
        }

        format!("sum of all badges is {}", sum_of_shared_items)
    }
}

struct Rucksack {
    compartment_1: Compartment,
    compartment_2: Compartment,
}

struct Compartment {
    items: Vec<Item>,
}

#[derive(Clone, Copy)]
struct Item {
    score: u32,
}

impl Item {
    fn from(char: char) -> Self {
        let score: u32 = if char.is_lowercase() {
            let value_of_first_letter = 'a'.to_digit(36).unwrap();
            let value_of_given_letter = char.to_digit(36).unwrap();
            let offset_of_given_letter = value_of_given_letter - value_of_first_letter;

            let score_of_lowercase_a = 1;
            let score = offset_of_given_letter + score_of_lowercase_a;

            score as u32
        } else {
            let value_of_first_letter = 'A'.to_digit(36).unwrap();
            let value_of_given_letter = char.to_digit(36).unwrap();
            let offset_of_given_letter = value_of_given_letter - value_of_first_letter;

            let score_of_uppercase_a = 27;
            let score = offset_of_given_letter + score_of_uppercase_a;

            score as u32
        };

        Item { score }
    }
}

fn parse_line_into_rucksack(line: &str) -> Rucksack {
    let item_count_in_each_compartment = line.len() / 2;

    let compartment_1 = Compartment {
        items: line
            .chars()
            .take(item_count_in_each_compartment)
            .map(|char| Item::from(char))
            .collect(),
    };

    let compartment_2 = Compartment {
        items: line
            .chars()
            .skip(item_count_in_each_compartment)
            .take(item_count_in_each_compartment)
            .map(|char| Item::from(char))
            .collect(),
    };

    Rucksack {
        compartment_1,
        compartment_2,
    }
}

fn get_item_that_shows_in_both_compartments_of_a_rucksack(rucksack: &Rucksack) -> Item {
    let mut bitmap = Bitmap::<53>::new();

    for item in &rucksack.compartment_1.items {
        bitmap.set(item.score as usize, true);
    }

    *rucksack
        .compartment_2
        .items
        .iter()
        .find(|item| bitmap.get(item.score as usize))
        .expect("could not find an item that shows up in both compartments")
}

fn get_item_shared_between_three_rucksacks(rucksacks: (&Rucksack, &Rucksack, &Rucksack)) -> Item {
    let (rucksack1, rucksack2, rucksack3) = rucksacks;

    let all_items_in_rucksack1 = rucksack1
        .compartment_1
        .items
        .iter()
        .chain(rucksack1.compartment_2.items.iter());

    let rucksack1_bitmap = all_items_in_rucksack1.fold(Bitmap::<53>::new(), |mut bitmap, item| {
        bitmap.set(item.score as usize, true);
        bitmap
    });

    let all_items_in_rucksack2 = rucksack2
        .compartment_1
        .items
        .iter()
        .chain(rucksack2.compartment_2.items.iter());

    let rucksack2_bitmap = all_items_in_rucksack2.fold(Bitmap::<53>::new(), |mut bitmap, item| {
        bitmap.set(item.score as usize, true);
        bitmap
    });

    let all_items_in_rucksack3 = rucksack3
        .compartment_1
        .items
        .iter()
        .chain(rucksack3.compartment_2.items.iter());

    let rucksack3_bitmap = all_items_in_rucksack3.fold(Bitmap::<53>::new(), |mut bitmap, item| {
        bitmap.set(item.score as usize, true);
        bitmap
    });

    let score = (1..53)
        .find(|s| rucksack1_bitmap.get(*s) && rucksack2_bitmap.get(*s) && rucksack3_bitmap.get(*s))
        .expect("no item shared between three rucksacks was found");

    Item {
        score: score as u32,
    }
}
