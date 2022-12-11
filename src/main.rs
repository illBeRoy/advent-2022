mod day;
mod days;
mod input;

use clap::{ArgAction, Parser};

use crate::day::Day;
use crate::days::day_10::Day10;
use crate::days::day_11::Day11;
use crate::days::day_2::Day2;
use crate::days::day_3::Day3;
use crate::days::day_4::Day4;
use crate::days::day_5::Day5;
use crate::days::day_6::Day6;
use crate::days::day_7::Day7;
use crate::days::day_8::Day8;
use crate::days::day_9::Day9;

#[derive(Parser)]
struct CLI {
    #[arg(long, help = "which day of the competition to run [2-30]")]
    day: usize,
    #[arg(long, help = "which task to run [1-2]")]
    task: u8,
    #[arg(
        long,
        action = ArgAction::SetTrue,
        help = "whether or not to display a description of the solution"
    )]
    describe: Option<bool>,
}

fn main() {
    let days: [Box<dyn Day>; 10] = [
        Box::from(Day2 {}),
        Box::from(Day3 {}),
        Box::from(Day4 {}),
        Box::from(Day5 {}),
        Box::from(Day6 {}),
        Box::from(Day7 {}),
        Box::from(Day8 {}),
        Box::from(Day9 {}),
        Box::from(Day10 {}),
        Box::from(Day11 {}),
    ];

    let args = CLI::parse();

    assert!(
        1 < args.day && args.day < 31,
        "invalid day (expected value between 2 to 30"
    );

    let day = days
        .get(args.day - 2)
        .expect(format!("day does not exist (day: {})", args.day).as_str());

    assert!(
        0 < args.task && args.task < 3,
        "invalid task index, expected 1 or 2"
    );

    println!("Advent of Code 2022");
    println!("");
    println!("Day {}", args.day);
    println!("{}", day.title());
    if args.describe == Some(true) {
        println!("{}", day.description());
    }
    println!("");
    println!("Task: {}", args.task);
    println!(
        "Result: {}",
        match args.task {
            1 => day.task_1(),
            2 => day.task_2(),
            _ => panic!("task should've been between 1 to 2. No idea what happened"),
        }
    );
}
