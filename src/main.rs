mod challenges;

use challenges::day01;
use std::time::Instant;

fn main() {
    let days = [("Day 01", day01::run)];

    for (day, challenge) in days.iter() {
        let start = Instant::now();
        let result = challenge();
        let duration = start.elapsed();

        let duration_in_ms = duration.as_secs_f64() * 1000.0;

        println!(
            "{}: Result: {:?}, Time: {:.3} ms",
            day, result, duration_in_ms
        );
    }
}
