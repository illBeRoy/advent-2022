use std::collections::HashMap;
use std::ops::{Add, Mul, Rem, Sub};

use itertools::Itertools;
use regex::Regex;

use crate::day::Day;
use crate::input::input_for_day;

#[derive(Clone, Copy)]
pub struct Day11 {}

impl Day for Day11 {
    fn title(&self) -> &'static str {
        "Monkey in the Middle"
    }

    fn description(&self) -> &'static str {
        "
        Wow, ok.

        First task is straightforward: we implemented monkeys, operations, operands, tests.
        We iterate 20 times, where for each monkey, for each item, we run the operation, divide by 3, test, and pass it on the the correct monkey.
        This worked well.

        Then came the second task. How can we handle such large numbers? Answer: we don't. We don't need to know the exact number, as we only count how many items each monkey examined.
        Instead, we only need to know whether or not it passes the monkey test - that is, whether or not it is DIVISIBLE by the number given in the test.

        Back in uni, during Algebra 1, we learned about modular arithmetic. Simply put, we \"limit\" the range of possible integers to only include [0..N), and then just LOOP if you go past N.
        For instance, in mod 7 arithmetic, 6 + 5 = 4 (which is 11 % 7).

        So we create a ModularNumber struct, that supports the Add, Sub and Mul operations (implemented using std::ops::{...} traits). Each instance of the number contains all values in the respective
        fields, so if we have monkeys that divide by 3, 5 and 7, then each ModularNumber(a) actually contains a % 3, a % 5 and a % 7.

        That worked! We managed to calculate the value for the second part, but now a new problem arise: we can't use the new ModularNumber for task 1, because division is not well defined for modular numbers.
        The solution? Implement the Remainder trait for the ModularNumber struct, and make the Monkey struct contain a generic, so we can use u64 for task 1, and ModularNumber for task 2.
        "
    }

    fn task_1(&self) -> String {
        let input = input_for_day(11);

        let monkey_descriptions = input
            .lines()
            .chunks(7)
            .into_iter()
            .map(|mut chunk| {
                chunk.next();
                [(); 5].map(|_| chunk.next().unwrap())
            })
            .collect_vec();

        let mut monkeys = monkey_descriptions
            .iter()
            .map(|desc| parse_into_monkey_with_u64(&desc))
            .collect_vec();

        for _round in 1..=20 {
            for monkey_id in 0..monkeys.len() {
                for item in monkeys[monkey_id].items.clone().iter() {
                    let new_worry_level = monkeys[monkey_id].operation.apply(item) / item.set(3);
                    let monkey_to_throw_to = monkeys[monkey_id].test.decide(new_worry_level);

                    monkeys
                        .get_mut(monkey_to_throw_to)
                        .expect(format!("monkey {} does not exist", monkey_to_throw_to).as_str())
                        .items
                        .push(new_worry_level);

                    monkeys.get_mut(monkey_id).unwrap().items_inspected += 1;
                }

                monkeys.get_mut(monkey_id).unwrap().items.clear();
            }
        }

        let summary = monkeys
            .iter()
            .enumerate()
            .map(|(i, monkey)| {
                format!(
                    "monkey {}: inspected items {} times",
                    i, monkey.items_inspected
                )
            })
            .join("\n");

        let (highest_scores, second_highest) = monkeys
            .iter()
            .map(|monkey| monkey.items_inspected)
            .sorted()
            .rev()
            .take(2)
            .collect_tuple()
            .unwrap();

        let monkey_business = highest_scores * second_highest;

        format!(
            "summary: \n{}\namount of monkey business is {} * {} = {}",
            summary, highest_scores, second_highest, monkey_business
        )
    }

    fn task_2(&self) -> String {
        let input = input_for_day(11);
        let modular_fields = parse_modulo_fields(&input);
        let monkey_descriptions = input
            .lines()
            .chunks(7)
            .into_iter()
            .map(|mut chunk| {
                chunk.next();
                [(); 5].map(|_| chunk.next().unwrap())
            })
            .collect_vec();

        let mut monkeys = monkey_descriptions
            .iter()
            .map(|desc| parse_into_monkey_with_modulo(&desc, &modular_fields))
            .collect_vec();

        for _round in 1..=10000 {
            for monkey_id in 0..monkeys.len() {
                for item in monkeys[monkey_id].items.clone().iter() {
                    let new_worry_level = monkeys[monkey_id].operation.apply(item);
                    let monkey_to_throw_to =
                        monkeys[monkey_id].test.decide(new_worry_level.clone());

                    monkeys
                        .get_mut(monkey_to_throw_to)
                        .expect(format!("monkey {} does not exist", monkey_to_throw_to).as_str())
                        .items
                        .push(new_worry_level);

                    monkeys.get_mut(monkey_id).unwrap().items_inspected += 1;
                }

                monkeys.get_mut(monkey_id).unwrap().items.clear();
            }
        }

        let summary = monkeys
            .iter()
            .enumerate()
            .map(|(i, monkey)| {
                format!(
                    "monkey {}: inspected items {} times",
                    i, monkey.items_inspected
                )
            })
            .join("\n");

        let (highest_scores, second_highest) = monkeys
            .iter()
            .map(|monkey| monkey.items_inspected)
            .sorted()
            .rev()
            .take(2)
            .collect_tuple()
            .unwrap();

        let monkey_business = highest_scores * second_highest;

        format!(
            "summary: \n{}\namount of monkey business is {} * {} = {}",
            summary, highest_scores, second_highest, monkey_business
        )
    }
}

struct Monkey<
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Rem<u64, Output = u64> + Clone + Set,
> {
    items: Vec<T>,
    items_inspected: u64,
    operation: MonkeyOperation,
    test: MonkeyTest,
}

enum MonkeyOperation {
    Add(Operand, Operand),
    Subtract(Operand, Operand),
    Multiply(Operand, Operand),
}

impl MonkeyOperation {
    fn apply<
        T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Rem<u64, Output = u64> + Clone + Set,
    >(
        &self,
        old_value: &T,
    ) -> T {
        match self {
            Self::Add(a, b) => a.resolve(old_value) + b.resolve(old_value),
            Self::Subtract(a, b) => a.resolve(old_value) - b.resolve(old_value),
            Self::Multiply(a, b) => a.resolve(old_value) * b.resolve(old_value),
        }
    }
}

enum Operand {
    OldValue,
    Constant(u8),
}

impl Operand {
    fn resolve<
        T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Rem<u64, Output = u64> + Clone + Set,
    >(
        &self,
        old_value: &T,
    ) -> T {
        match self {
            Self::OldValue => old_value.clone(),
            Self::Constant(val) => old_value.set(*val as u64),
        }
    }
}

struct MonkeyTest {
    divisible_by: u8,
    if_true_throw_to: usize,
    if_false_throw_to: usize,
}

impl MonkeyTest {
    fn decide<
        T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Rem<u64, Output = u64> + Clone + Set,
    >(
        &self,
        worry_level: T,
    ) -> usize {
        if worry_level % (self.divisible_by as u64) == 0 {
            self.if_true_throw_to.clone()
        } else {
            self.if_false_throw_to.clone()
        }
    }
}

#[derive(Clone, Debug)]
struct ModularNumber {
    value_by_field: HashMap<u8, u8>,
}

impl ModularNumber {
    fn new(initial: u8, modular_fields: &Vec<u8>) -> Self {
        let value_by_field: HashMap<u8, u8> = modular_fields
            .iter()
            .map(|field| (field.clone(), initial % field))
            .collect();

        Self { value_by_field }
    }

    fn value_in_field(&self, field: u8) -> u8 {
        self.value_by_field
            .get(&field)
            .expect(format!("no value was found for field {}", field).as_str())
            .clone()
    }
}

impl Set for ModularNumber {
    fn set(&self, value: u64) -> Self {
        let new_value_by_field: HashMap<u8, u8> = self
            .value_by_field
            .iter()
            .map(|(field, _)| (field.clone(), (value % *field as u64) as u8))
            .collect();

        Self {
            value_by_field: new_value_by_field,
        }
    }
}

impl Add for ModularNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let new_value_by_field: HashMap<u8, u8> = self
            .value_by_field
            .iter()
            .map(|(field, old_value)| {
                (
                    field.clone(),
                    (old_value + rhs.value_in_field(*field)) % field,
                )
            })
            .collect();

        Self {
            value_by_field: new_value_by_field,
        }
    }
}

impl Sub for ModularNumber {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let new_value_by_field: HashMap<u8, u8> = self
            .value_by_field
            .iter()
            .map(|(field, old_value)| {
                (
                    field.clone(),
                    ((old_value.clone() as i8 - rhs.value_in_field(*field) as i8)
                        % field.clone() as i8) as u8,
                )
            })
            .collect();

        Self {
            value_by_field: new_value_by_field,
        }
    }
}

impl Mul for ModularNumber {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let new_value_by_field: HashMap<u8, u8> = self
            .value_by_field
            .iter()
            .map(|(field, old_value)| {
                (
                    field.clone(),
                    ((old_value.clone() as u64 * rhs.value_in_field(*field) as u64)
                        % field.clone() as u64) as u8,
                )
            })
            .collect();

        Self {
            value_by_field: new_value_by_field,
        }
    }
}

impl Rem<u64> for ModularNumber {
    type Output = u64;

    fn rem(self, rhs: u64) -> Self::Output {
        self.value_in_field(rhs as u8) as u64
    }
}

trait Set {
    fn set(&self, value: u64) -> Self;
}

impl Set for u64 {
    fn set(&self, value: u64) -> Self {
        value
    }
}

fn parse_modulo_fields(input: &String) -> Vec<u8> {
    input
        .lines()
        .filter(|line| line.contains("divisible by"))
        .map(|line| {
            line.trim()
                .split("divisible by ")
                .nth(1)
                .unwrap()
                .parse::<u8>()
                .unwrap()
        })
        .collect_vec()
}

fn parse_into_monkey_with_u64(monkey_description: &[&str; 5]) -> Monkey<u64> {
    let starting_items = monkey_description[0]
        .trim()
        .split(": ")
        .nth(1)
        .expect("expected first line to contain starting items")
        .split(", ")
        .map(|val_as_str| {
            val_as_str
                .parse::<u64>()
                .expect("expected values list to contain numerical strings only")
        })
        .collect_vec();

    let operation_matcher = Regex::new(
        r"^(?P<left_operand>old|\d+) (?P<operation>\+|-|\*) (?P<right_operand>old|\d+)$",
    )
    .unwrap();

    let matched_operation = monkey_description[1]
        .trim()
        .split("new = ")
        .nth(1)
        .map(|the_math| operation_matcher.captures(the_math))
        .expect("expected second line to contain operation")
        .expect("expected operation to be a valid binary operation in infix notation");

    let left_operand = match matched_operation.name("left_operand").map(|m| m.as_str()) {
        Some("old") => Operand::OldValue,
        Some(num_as_str) => Operand::Constant(num_as_str.parse::<u8>().unwrap()),
        _ => panic!("at the disco"),
    };
    let right_operand = match matched_operation.name("right_operand").map(|m| m.as_str()) {
        Some("old") => Operand::OldValue,
        Some(num_as_str) => Operand::Constant(num_as_str.parse::<u8>().unwrap()),
        _ => panic!("at the disco"),
    };
    let operation = match matched_operation.name("operation").map(|m| m.as_str()) {
        Some("+") => MonkeyOperation::Add(left_operand, right_operand),
        Some("-") => MonkeyOperation::Subtract(left_operand, right_operand),
        Some("*") => MonkeyOperation::Multiply(left_operand, right_operand),
        _ => panic!("do not panic"),
    };

    let divisible_by = monkey_description[2]
        .trim()
        .split("divisible by ")
        .nth(1)
        .map(|num_as_str| num_as_str.parse::<u8>())
        .expect("expected third line to contain the divisible test")
        .expect("divisible operand should be a valid non-negative int");

    let if_true_throw_to = monkey_description[3]
        .trim()
        .split("If true: throw to monkey ")
        .nth(1)
        .map(|monkey_index_as_str| monkey_index_as_str.parse::<usize>())
        .expect("expected fourth line to contain id of monkey to throw if true")
        .expect("id of monkey must be a non-negative int");
    let if_false_throw_to = monkey_description[4]
        .trim()
        .split("If false: throw to monkey ")
        .nth(1)
        .map(|monkey_index_as_str| monkey_index_as_str.parse::<usize>())
        .expect("expected fifth line to contain id of monkey to throw if false")
        .expect("id of monkey must be a non-negative int");

    Monkey {
        items: starting_items,
        items_inspected: 0,
        operation: operation,
        test: MonkeyTest {
            divisible_by,
            if_true_throw_to,
            if_false_throw_to,
        },
    }
}

fn parse_into_monkey_with_modulo(
    monkey_description: &[&str; 5],
    modulo_fields: &Vec<u8>,
) -> Monkey<ModularNumber> {
    let starting_items = monkey_description[0]
        .trim()
        .split(": ")
        .nth(1)
        .expect("expected first line to contain starting items")
        .split(", ")
        .map(|val_as_str| {
            val_as_str
                .parse::<u8>()
                .expect("expected values list to contain numerical strings only")
        })
        .map(|reg_number| ModularNumber::new(reg_number, modulo_fields))
        .collect_vec();

    let operation_matcher = Regex::new(
        r"^(?P<left_operand>old|\d+) (?P<operation>\+|-|\*) (?P<right_operand>old|\d+)$",
    )
    .unwrap();

    let matched_operation = monkey_description[1]
        .trim()
        .split("new = ")
        .nth(1)
        .map(|the_math| operation_matcher.captures(the_math))
        .expect("expected second line to contain operation")
        .expect("expected operation to be a valid binary operation in infix notation");

    let left_operand = match matched_operation.name("left_operand").map(|m| m.as_str()) {
        Some("old") => Operand::OldValue,
        Some(num_as_str) => Operand::Constant(num_as_str.parse::<u8>().unwrap()),
        _ => panic!("at the disco"),
    };
    let right_operand = match matched_operation.name("right_operand").map(|m| m.as_str()) {
        Some("old") => Operand::OldValue,
        Some(num_as_str) => Operand::Constant(num_as_str.parse::<u8>().unwrap()),
        _ => panic!("at the disco"),
    };
    let operation = match matched_operation.name("operation").map(|m| m.as_str()) {
        Some("+") => MonkeyOperation::Add(left_operand, right_operand),
        Some("-") => MonkeyOperation::Subtract(left_operand, right_operand),
        Some("*") => MonkeyOperation::Multiply(left_operand, right_operand),
        _ => panic!("do not panic"),
    };

    let divisible_by = monkey_description[2]
        .trim()
        .split("divisible by ")
        .nth(1)
        .map(|num_as_str| num_as_str.parse::<u8>())
        .expect("expected third line to contain the divisible test")
        .expect("divisible operand should be a valid non-negative int");

    let if_true_throw_to = monkey_description[3]
        .trim()
        .split("If true: throw to monkey ")
        .nth(1)
        .map(|monkey_index_as_str| monkey_index_as_str.parse::<usize>())
        .expect("expected fourth line to contain id of monkey to throw if true")
        .expect("id of monkey must be a non-negative int");
    let if_false_throw_to = monkey_description[4]
        .trim()
        .split("If false: throw to monkey ")
        .nth(1)
        .map(|monkey_index_as_str| monkey_index_as_str.parse::<usize>())
        .expect("expected fifth line to contain id of monkey to throw if false")
        .expect("id of monkey must be a non-negative int");

    Monkey {
        items: starting_items,
        items_inspected: 0,
        operation: operation,
        test: MonkeyTest {
            divisible_by,
            if_true_throw_to,
            if_false_throw_to,
        },
    }
}
