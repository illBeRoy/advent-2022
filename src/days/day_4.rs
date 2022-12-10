use itertools::Itertools;
use std::ops::RangeInclusive;

use crate::day::Day;
use crate::input::input_for_day;

#[derive(Clone, Copy)]
pub struct Day4 {}

impl Day for Day4 {
    fn title(&self) -> &'static str {
        "Camp Cleanup"
    }

    fn description(&self) -> &'static str {
        "
        First, the math:
        Range [a,b] contains range [c,d] if a <= c and b >= d.
        Range [a,b] overlaps range [c,d] if c <= a <= d or a <= c <= b.

        Using this math, our implementation is straightforward:
        1. Parse the input lines into tuples of range strings
        2. Parse each range string into rust's InclusiveRange (in my code I wrapped them in a CleaningJob struct)
        3. Task 1: check that one range contains the other
        4. Task 2: check that one range overlaps the other
        "
    }

    fn task_1(&self) -> String {
        let input = input_for_day(4);
        let cleaning_job_pairs = input.lines().map(parse_line_into_cleaning_jobs);

        let pairs_where_one_job_contains_the_other =
            cleaning_job_pairs.filter(|(job1, job2)| job1.contains(job2) || job2.contains(job1));

        format!(
            "the count of pairs where one job contains the other is {}",
            pairs_where_one_job_contains_the_other.count()
        )
    }

    fn task_2(&self) -> String {
        let input = input_for_day(4);
        let cleaning_job_pairs = input.lines().map(parse_line_into_cleaning_jobs);

        let pairs_where_one_job_overlaps_the_other =
            cleaning_job_pairs.filter(|(job1, job2)| job1.overlaps(job2));

        format!(
            "the count of pairs where one job overlaps the other is {}",
            pairs_where_one_job_overlaps_the_other.count()
        )
    }
}

struct CleaningJob {
    range: RangeInclusive<u32>,
}

impl CleaningJob {
    fn contains(&self, other_job: &Self) -> bool {
        self.range.contains(other_job.range.start()) && self.range.contains(other_job.range.end())
    }

    fn overlaps(&self, other_job: &Self) -> bool {
        self.range.contains(other_job.range.start()) || other_job.range.contains(self.range.start())
    }
}

fn parse_line_into_cleaning_jobs(line: &str) -> (CleaningJob, CleaningJob) {
    let (job_desc1, job_desc2) = line
        .split(',')
        .collect_tuple()
        .expect("invalid line: did not find specification of two cleaning jobs separated by comma");

    let cleaning_job1 = parse_range_string_into_cleaning_job(job_desc1);
    let cleaning_job2 = parse_range_string_into_cleaning_job(job_desc2);

    (cleaning_job1, cleaning_job2)
}

fn parse_range_string_into_cleaning_job(range_string: &str) -> CleaningJob {
    let from = range_string
        .split('-')
        .nth(0)
        .map(|s| s.parse::<u32>().unwrap())
        .expect("could not parse beginning of cleaning job range");

    let to = range_string
        .split('-')
        .nth(1)
        .map(|s| s.parse::<u32>().unwrap())
        .expect("could not parse beginning of cleaning job range");

    CleaningJob { range: from..=to }
}
