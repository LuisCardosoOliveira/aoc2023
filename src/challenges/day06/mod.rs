use std::time::Instant;

const INPUT: &str = include_str!("input6.txt");

pub fn run() -> ((u128, f64), (u128, f64)) {
  let start_part1 = Instant::now();
  let result_part1 = part1();
  let duration_part1 = start_part1.elapsed();
  let duration_part1_in_ms = duration_part1.as_secs_f64() * 1000.0;

  let start_part2 = Instant::now();
  let result_part2 = part2();
  let duration_part2 = start_part2.elapsed();
  let duration_part2_in_ms = duration_part2.as_secs_f64() * 1000.0;

  (
    (result_part1, duration_part1_in_ms),
    (result_part2 as u128, duration_part2_in_ms),
  )
}

fn part1() -> u128 {
  let mut lines = INPUT.lines();

  let times = lines
    .next()
    .unwrap()
    .split_whitespace()
    .skip(1)
    .map(|n| n.parse::<f64>().unwrap());
  let distances = lines
    .next()
    .unwrap()
    .split_whitespace()
    .skip(1)
    .map(|n| n.parse::<f64>().unwrap());

  times
    .zip(distances)
    .map(|(time, record)| calculate_optimized_winning_ways(time, record))
    .product()
}

fn part2() -> u128 {
  let mut lines = INPUT.lines();

  let time = lines
    .next()
    .unwrap()
    .split_whitespace()
    .skip(1)
    .collect::<String>()
    .parse::<f64>()
    .unwrap();
  let distance = lines
    .next()
    .unwrap()
    .split_whitespace()
    .skip(1)
    .collect::<String>()
    .parse::<f64>()
    .unwrap();

  calculate_optimized_winning_ways(time, distance)
}

fn calculate_optimized_winning_ways(time: f64, record: f64) -> u128 {
  // Solving the quadratic equation: h * (time - h) = distance
  let delta = ((time * time) - (4.0 * record)).sqrt();
  let min_hold = ((time - delta) / 2.0).ceil();
  let max_hold = ((time + delta) / 2.0).floor();

  // Check if the range is valid and calculate the count
  if min_hold <= max_hold {
    // +1 to include both boundaries
    (max_hold - min_hold + 1.0) as u128
  } else {
    0
  }
}
