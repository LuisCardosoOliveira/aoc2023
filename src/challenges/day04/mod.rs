use std::collections::HashSet;
use std::time::Instant;

const INPUT: &str = include_str!("input4.txt");

pub fn run() -> ((u32, f64), (u32, f64)) {
  let start_part1 = Instant::now();
  let result_part1 = part1();
  let duration_part1 = start_part1.elapsed();
  let duration_part1_in_ms = duration_part1.as_secs_f64() * 1000.0;

  // let start_part2 = Instant::now();
  // let result_part2 = part2();
  // let duration_part2 = start_part2.elapsed();
  // let duration_part2_in_ms = duration_part2.as_secs_f64() * 1000.0;

  ((result_part1, duration_part1_in_ms), (0, 0.0))
}

fn part1() -> u32 {
  INPUT
    .lines()
    .map(|line| {
      let line = line
        .strip_prefix("Card ")
        .and_then(|s| s.split_at(s.find(':').unwrap_or(0)).1.strip_prefix(": "))
        .unwrap_or("");
      let parts: Vec<&str> = line.split('|').collect();
      let set1: std::collections::HashSet<u32> = parts[0]
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
      let set2: std::collections::HashSet<u32> = parts[1]
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
      let intersection_count = set1.intersection(&set2).count();
      if intersection_count > 0 {
        2u32.pow(intersection_count as u32 - 1)
      } else {
        0
      }
    })
    .sum()
}
