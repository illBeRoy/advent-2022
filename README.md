# Advent of Code 2022
⚙️ Rust Edition

## About
This is a rust implementation of AoC2022. It will be updated daily (hopefully) with each new task.

## How to Use
> You need to have **rust** and **cargo** installed in order to run this code

Start by running:
```sh
cargo build
```

You can then run the project by running
```sh
cargo run -- --day <DAY> --task <TASK>
```
Where:
- DAY: The day of the puzzle you want to run (2-30)
- TASK: Which task of the given day to run (1-2)

In addition, if you want to get a description of how I solved the task, you can provide the `--describe` arg:

```sh
cargo run -- --day 2 --task 1 --describe

Day 2
Rock Paper Scissors

        The solution is pretty straightforward: we introduce a "Match" struct that
        handles all the relevant logic: who wins over whom, and the scoring.

        From that point onwards, all that changes between the two tasks is how we interpret the input:
        In the first, we interpret XYZ into specific hands. In the second we interpret them according to the opponent hand.
        

Task: 2
Result: total score: 13889
```

## Q&A
### Why no day 1?
I completed the first day's puzzle in Typescript :)

### How to run with my own input?
Come on, the fun part of AoC is solving the puzzles yourself.
That said, you can find the inputs in `assets/inputs` directory. Simply edit the relevant input file for the day and task you wish to solve.