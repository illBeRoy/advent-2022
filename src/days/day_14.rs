use itertools::Itertools;

use crate::day::Day;
use crate::input::input_for_day;

#[derive(Clone, Copy)]
pub struct Day14 {}

impl Day for Day14 {
    fn title(&self) -> &'static str {
        "Regolith Reservoir"
    }

    fn description(&self) -> &'static str {
        "
        We create a grid and draw the walls according to the input: we parse each line, separating the coordinates by using
        the arrows (\" -> \") as delimiters. We then iterate over the coordinates, and \"move\" a cursor between them to fill
        the relevant pixels as the terrain.
        
        We calculate the height of the abyss, which is as low as the last line that actually has any ground at all, and start
        simulating.

        For each grain, we drop it according to the simulation. We stop the simulation of any specific grain under one of two circumstances:
        1. It has come to rest
        2. It has reached the height of the abyss

        If we stopped due to condition 2, the simulation is over, and we return the amount of grains of sand that we simulated thus far.

        Part 2 is mostly similar, with three changes:
        1. Instead of the abyss, we calculate the height of the endless floor at <lowest point> + 2
        2. Once a grain reaches the endless floor, it stops (we don't actually \"draw\" the floor into the grid, instead just compare the y coords)
        3. We stop the simulation when a grain comes to rest at (500, 0)

        And that is all, pretty straightforward simulation over here!
        "
    }

    fn task_1(&self) -> String {
        let input = input_for_day(14);
        let mut grid = parse_input_into_grid(&input);

        let height_of_the_abyss = beep_bop_find_lowest_terrain_of_scan(&grid);

        let mut rested_grains_of_sand = 0;
        loop {
            let mut grain_pos = (500 as usize, 0 as usize);
            loop {
                if grain_pos.1 >= height_of_the_abyss {
                    break;
                }

                if grid[grain_pos.1 + 1][grain_pos.0] == Pixel::Empty {
                    grain_pos.1 += 1;
                    continue;
                }

                if grid[grain_pos.1 + 1][grain_pos.0 - 1] == Pixel::Empty {
                    grain_pos = (grain_pos.0 - 1, grain_pos.1 + 1);
                    continue;
                }

                if grid[grain_pos.1 + 1][grain_pos.0 + 1] == Pixel::Empty {
                    grain_pos = (grain_pos.0 + 1, grain_pos.1 + 1);
                    continue;
                }

                break;
            }

            if grain_pos.1 >= height_of_the_abyss {
                break;
            } else {
                grid[grain_pos.1][grain_pos.0] = Pixel::Sand;
                rested_grains_of_sand += 1;
            }
        }

        format!(
            "{} grains of sand rested before reaching the abyss",
            rested_grains_of_sand
        )
    }

    fn task_2(&self) -> String {
        let input = input_for_day(14);
        let mut grid = parse_input_into_grid(&input);

        let height_of_the_endless_floor = beep_bop_find_lowest_terrain_of_scan(&grid) + 2;

        let mut rested_grains_of_sand = 0;
        loop {
            let mut grain_pos = (500 as usize, 0 as usize);
            loop {
                if grain_pos.1 == height_of_the_endless_floor - 1 {
                    break;
                }

                if grid[grain_pos.1 + 1][grain_pos.0] == Pixel::Empty {
                    grain_pos.1 += 1;
                    continue;
                }

                if grid[grain_pos.1 + 1][grain_pos.0 - 1] == Pixel::Empty {
                    grain_pos = (grain_pos.0 - 1, grain_pos.1 + 1);
                    continue;
                }

                if grid[grain_pos.1 + 1][grain_pos.0 + 1] == Pixel::Empty {
                    grain_pos = (grain_pos.0 + 1, grain_pos.1 + 1);
                    continue;
                }

                break;
            }

            rested_grains_of_sand += 1;

            if grain_pos == (500, 0) {
                break;
            } else {
                grid[grain_pos.1][grain_pos.0] = Pixel::Sand;
            }
        }

        format!(
            "{} grains of sand rested before filling up to the top",
            rested_grains_of_sand
        )
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Pixel {
    Empty,
    Ground,
    Sand,
}

fn parse_input_into_grid(input: &String) -> Vec<Vec<Pixel>> {
    let mut grid = vec![vec![Pixel::Empty; 1000]; 1000];

    for line in input.lines() {
        let coords = line
            .split(" -> ")
            .map(|coord_str| {
                let coord_nums = coord_str
                    .split(',')
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect_vec();
                (coord_nums[0], coord_nums[1])
            })
            .collect_vec();

        for (mut a, b) in coords.windows(2).map(|w| (w[0], w[1])) {
            loop {
                let (x1, y1) = a;
                grid[y1][x1] = Pixel::Ground;

                let (x2, y2) = b;
                if a != b {
                    a = (
                        (x1 as isize + (x2 as isize - x1 as isize).clamp(-1, 1)) as usize,
                        (y1 as isize + (y2 as isize - y1 as isize).clamp(-1, 1)) as usize,
                    );
                } else {
                    break;
                }
            }
        }
    }

    grid
}

fn beep_bop_find_lowest_terrain_of_scan(grid: &Vec<Vec<Pixel>>) -> usize {
    grid.iter()
        .enumerate()
        .filter(|(_, line)| !line.iter().all(|pixel| pixel == &Pixel::Empty))
        .last()
        .map(|(i, _)| i)
        .expect("could not find terrain in any depth? this really is the abyss!")
}
