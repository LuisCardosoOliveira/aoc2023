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
    .map(|n| n.parse::<u128>().unwrap());
  let distances = lines
    .next()
    .unwrap()
    .split_whitespace()
    .skip(1)
    .map(|n| n.parse::<u128>().unwrap());

  times
    .zip(distances)
    .map(|(time, record)| calculate_winning_ways(time, record))
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
    .parse::<u128>()
    .unwrap();
  let distance = lines
    .next()
    .unwrap()
    .split_whitespace()
    .skip(1)
    .collect::<String>()
    .parse::<u128>()
    .unwrap();

  calculate_winning_ways(time, distance)
}

fn calculate_winning_ways(time: u128, record: u128) -> u128 {
  (0..time)
    .filter_map(|hold_time| {
      let distance = (time - hold_time) * hold_time;
      if distance > record {
        Some(1)
      } else {
        None
      }
    })
    .count() as u128
}
