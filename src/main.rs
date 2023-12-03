mod challenges;

use challenges::{day01, day02};
use std::time::Instant;

type ChallengeFn = fn() -> ((u32, f64), (u32, f64));

fn main() {
  let days: [(&str, ChallengeFn); 2] =
    [("Day 01", day01::run), ("Day 02", day02::run)];

  for (day, challenge) in days.iter() {
    let overall_start = Instant::now();
    let ((result_part1, time_part1), (result_part2, time_part2)) = challenge();
    let overall_duration = overall_start.elapsed();
    let overall_duration_in_ms = overall_duration.as_secs_f64() * 1000.0;

    println!(
            "{}: \n\tPart 1: Result: {:?}, Time: {:.3} ms\n\tPart 2: Result: {:?}, Time: {:.3} ms\n\tOverall Time: {:.3} ms",
            day, result_part1, time_part1, result_part2, time_part2, overall_duration_in_ms
        );
  }
}
