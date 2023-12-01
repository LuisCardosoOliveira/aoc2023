// main.rs
mod challenges;

use challenges::day01;
use std::time::Instant;

fn main() {
  let days = [("Day 01", day01::run)];

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
