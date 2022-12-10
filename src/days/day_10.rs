use bitmaps::Bitmap;
use itertools::Itertools;

use crate::day::Day;
use crate::input::input_for_day;

#[derive(Clone, Copy)]
pub struct Day10 {}

impl Day for Day10 {
    fn title(&self) -> &'static str {
        "Cathode-Ray Tube"
    }

    fn description(&self) -> &'static str {
        "
        A straightforward puzzle.

        I started by implementing a CPU and Instruction structs. The CPU contains one register and counts cycles.
        This was enough for task one. I parsed the input and ran the CPU 20, 60, 100, 140, 180 and 220 times, and
        calculated the required output.

        For task 2, I also implemented a CRT. I used a slice of 6 bitmaps, each bitmap represents a pixel on string.
        The CRT counts its own cycles as well. Every cycle, it receives a pixel position value (which I take from the CPU) and
        decides whether or not to flip the <cycle % 40> bit in the <cycle / 40> row.

        I had some problems with unsigned ints (I assumed that the register could not hold negative values, and I was wrong),
        but after identifying the issue everything fell into place and the secret code was revealed before me.
        "
    }

    fn task_1(&self) -> String {
        let input = input_for_day(10);
        let program = parse_input_into_program(&input);

        let mut cpu = CPU::new(program);

        cpu.run_until_cycle(20);
        let val_at_20 = cpu.reg as u32;

        cpu.run_until_cycle(60);
        let val_at_60 = cpu.reg as u32;

        cpu.run_until_cycle(100);
        let val_at_100 = cpu.reg as u32;

        cpu.run_until_cycle(140);
        let val_at_140 = cpu.reg as u32;

        cpu.run_until_cycle(180);
        let val_at_180 = cpu.reg as u32;

        cpu.run_until_cycle(220);
        let val_at_220 = cpu.reg as u32;

        let sum = (20 * val_at_20)
            + (60 * val_at_60)
            + (100 * val_at_100)
            + (140 * val_at_140)
            + (180 * val_at_180)
            + (220 * val_at_220);

        format!("the sum of signal strength is {}", sum)
    }

    fn task_2(&self) -> String {
        let input = input_for_day(10);
        let program = parse_input_into_program(&input);

        let mut cpu = CPU::new(program);
        let mut monitor = CRT::new();

        for _ in 1..=240 {
            cpu.next_cycle();
            monitor.run_draw_cycle(cpu.reg);
        }

        let monitor_text = monitor.draw_to_string();
        format!("The text displaying on the monitor is\n{}", monitor_text)
    }
}

struct CPU {
    program: Program,
    reg: i8,
    cycle: usize,
    execution: Execution,
}

impl CPU {
    fn new(program: Program) -> Self {
        Self {
            reg: 1,
            cycle: 0,
            execution: Execution {
                instruction: 0,
                cycles_left: program[0].cycles(),
            },
            program,
        }
    }

    fn next_cycle(&mut self) {
        if self.execution.cycles_left == 0 {
            let instruction = &self.program[self.execution.instruction];

            match instruction {
                Instruction::AddX(by) => self.reg = ((self.reg as i8) + by) as i8,
                _ => {}
            }

            let next_instruction = self.execution.instruction + 1;

            self.execution = Execution {
                instruction: next_instruction,
                cycles_left: self.program[next_instruction].cycles(),
            };
        }

        self.cycle += 1;
        self.execution.cycles_left -= 1;
    }

    fn run_until_cycle(&mut self, cycle: usize) {
        while self.cycle < cycle {
            self.next_cycle();
        }
    }
}

struct CRT {
    monitor: [Bitmap<40>; 6],
    cycle: usize,
}

impl CRT {
    fn new() -> Self {
        Self {
            monitor: [Bitmap::new(); 6],
            cycle: 0,
        }
    }

    fn run_draw_cycle(&mut self, pixel_position: i8) {
        let row = self.cycle / 40;
        let pixel = self.cycle % 40;

        let range_of_visible_pixels = (pixel_position - 1)..=(pixel_position + 1);
        if range_of_visible_pixels.contains(&(pixel as i8)) {
            self.monitor[row as usize].set(pixel as usize, true);
        }

        self.cycle += 1;
    }

    fn draw_to_string(&self) -> String {
        self.monitor
            .map(|row| (0..40).map(|i| if row.get(i) { '#' } else { '.' }).join(""))
            .join("\n")
    }
}

type Program = Vec<Instruction>;

enum Instruction {
    Noop,
    AddX(i8),
}

impl Instruction {
    fn cycles(&self) -> usize {
        match self {
            Self::Noop => 1,
            Self::AddX(_) => 2,
        }
    }
}

struct Execution {
    instruction: usize,
    cycles_left: usize,
}

fn parse_input_into_program(input: &String) -> Program {
    input
        .lines()
        .map(|line| line.trim())
        .map(|line| {
            let inst = line.split(" ").nth(0).unwrap();
            let param = line.split(" ").nth(1);
            match inst {
                "noop" => Instruction::Noop,
                "addx" => Instruction::AddX(
                    param
                        .expect("addx must come with a second param")
                        .parse::<i8>()
                        .expect("addx param must be a valid int"),
                ),
                unsupported => panic!("unknown instruction: {}", unsupported),
            }
        })
        .collect_vec()
}
