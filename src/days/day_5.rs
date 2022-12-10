use itertools::Itertools;
use regex::Regex;

use crate::day::Day;
use crate::input::input_for_day;

#[derive(Clone, Copy)]
pub struct Day5 {}

impl Day for Day5 {
    fn title(&self) -> &'static str {
        "Supply Stacks"
    }

    fn description(&self) -> &'static str {
        "
        The task itself is, again, simple. We hold a vector of stacks, where the crates are,
        and a set of instructions that tell us how many crates to move, and which two stacks to move them between.

        In the first task, we pop each crate and push it individually, simulating a crane that lifts one crate at a time.
        In the second, we take multiple crates at once and append them in the same order to the top of the stack.

        The parsing itself was quite the challenge, though. We have two parsing functions, the first goes over
        the (rather graphic) representation of the stacks, and generates our vector of stacks in the process.
        The second function uses a regex to match the \"instruction\" lines, and constructs a MoveInstruction
        from it.
        "
    }

    fn task_1(&self) -> String {
        let input = input_for_day(5);

        let mut stacks = parse_crate_stacks_from_input(input.as_str());
        let instructions = parse_move_instructions_from_input(input.as_str());

        instructions
            .iter()
            .for_each(|instruction| apply_instruction_to_stacks(&instruction, &mut stacks));

        let secret_password = stacks
            .iter()
            .map(|stack| stack.last().unwrap_or(&' '))
            .join("");

        format!("the password from the top crates is {:?}", secret_password)
    }

    fn task_2(&self) -> String {
        let input = input_for_day(5);

        let mut stacks = parse_crate_stacks_from_input(input.as_str());
        let instructions = parse_move_instructions_from_input(input.as_str());

        instructions.iter().for_each(|instruction| {
            apply_instruction_to_stacks_with_batch_moving(&instruction, &mut stacks)
        });

        let secret_password = stacks
            .iter()
            .map(|stack| stack.last().unwrap_or(&' '))
            .join("");

        format!("the password from the top crates is {:?}", secret_password)
    }
}

type CrateStack = Vec<char>;

struct MoveInstruction {
    amount: u32,
    from: usize,
    to: usize,
}

fn parse_crate_stacks_from_input(input: &str) -> Vec<CrateStack> {
    let mut lines = input.lines().peekable();

    let stacks_count = (lines.peek().expect("input is empty").len() + 1) / 4;
    let mut stacks = vec![CrateStack::new(); stacks_count];

    while lines.peek().filter(|l| !l.starts_with(" 1")).is_some() {
        let line = lines.next().expect("we peeked and there was a line");

        for (i, mut four_chars) in line.chars().chunks(4).into_iter().enumerate() {
            if four_chars.next() == Some('[') {
                let crate_name = four_chars
                    .next()
                    .expect("crate chunk should have at least three characters");

                stacks.get_mut(i).map(|stack| stack.insert(0, crate_name));
            }
        }
    }

    stacks
}

fn parse_move_instructions_from_input(input: &str) -> Vec<MoveInstruction> {
    let instruction_regex =
        Regex::new(r"move (?P<amount>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
    let is_instruction_line = |l: &str| instruction_regex.is_match(l);

    let instruction_lines = input.lines().skip_while(|l| !is_instruction_line(l));

    instruction_lines
        .map(|l| {
            let matches = instruction_regex
                .captures(l)
                .expect("could not match instruction line");

            let amount = matches
                .name("amount")
                .expect("invalid instruction: no 'amount' value")
                .as_str()
                .parse::<u32>()
                .unwrap();

            let from = matches
                .name("from")
                .expect("invalid instruction: no 'from' value")
                .as_str()
                .parse::<usize>()
                .unwrap();

            let to = matches
                .name("to")
                .expect("invalid instruction: no 'to' value")
                .as_str()
                .parse::<usize>()
                .unwrap();

            MoveInstruction { amount, from, to }
        })
        .collect_vec()
}

fn apply_instruction_to_stacks(instruction: &MoveInstruction, stacks: &mut Vec<CrateStack>) {
    for _ in 1..=(instruction.amount) {
        let pulled_crate_if_exists = stacks
            .get_mut(instruction.from - 1)
            .expect("stack index from instruction does not exist")
            .pop();

        if let Some(crate_name) = pulled_crate_if_exists {
            stacks
                .get_mut(instruction.to - 1)
                .expect("stack index from instruction does not exist")
                .push(crate_name);
        }
    }
}

fn apply_instruction_to_stacks_with_batch_moving(
    instruction: &MoveInstruction,
    stacks: &mut Vec<CrateStack>,
) {
    let from_stack = stacks
        .get_mut(instruction.from - 1)
        .expect("stack index from instruction does not exist");

    let mut crates_to_move = from_stack
        .drain((from_stack.len() - instruction.amount as usize)..)
        .collect_vec();

    let to_stack = stacks
        .get_mut(instruction.to - 1)
        .expect("stack index from instruction does not exist");

    to_stack.append(&mut crates_to_move);
}
