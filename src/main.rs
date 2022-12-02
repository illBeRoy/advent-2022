mod day;
mod days;

use crate::{day::Day, days::day_2::Day2};
use clap::Parser;

#[derive(Parser)]
struct CLI {
    #[arg(long, help = "which day of the competition to run [2-30]")]
    day: usize,
    #[arg(long, help = "which task to run [1-2]")]
    task: u8,
}

fn main() {
    let days = [Day2 {}];
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

    println!("Day {}", args.day);
    println!("{}", day.title());
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
