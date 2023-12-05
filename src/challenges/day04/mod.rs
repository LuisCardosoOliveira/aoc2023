use std::time::Instant;

const INPUT: &str = include_str!("input4.txt");

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
    (result_part1 as u128, duration_part1_in_ms),
    (result_part2 as u128, duration_part2_in_ms),
  )
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
      let mut vec1: Vec<u128> = parts[0]
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
      let mut vec2: Vec<u128> = parts[1]
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
      vec1.sort_unstable();
      vec2.sort_unstable();
      let intersection_count = vec1
        .iter()
        .filter(|&n| vec2.binary_search(n).is_ok())
        .count();
      if intersection_count > 0 {
        2_u32.pow(intersection_count as u32 - 1)
      } else {
        0
      }
    })
    .sum::<u32>()
}

fn part2() -> u128 {
  let mut card_copies = INPUT.lines().map(|_| 1).collect::<Vec<u128>>();

  let mut index = 0;
  while index < card_copies.len() {
    let line = INPUT.lines().nth(index).unwrap();
    let line = line
      .strip_prefix("Card ")
      .and_then(|s| s.split_at(s.find(':').unwrap_or(0)).1.strip_prefix(": "))
      .unwrap_or("");
    let parts: Vec<&str> = line.split('|').collect();
    let vec1: Vec<u128> = parts[0]
      .split_whitespace()
      .filter_map(|s| s.parse().ok())
      .collect();
    let vec2: Vec<u128> = parts[1]
      .split_whitespace()
      .filter_map(|s| s.parse().ok())
      .collect();

    let intersection_count = vec1.iter().filter(|&n| vec2.contains(n)).count();

    for i in 1..=intersection_count {
      if index + i < card_copies.len() {
        card_copies[index + i] += card_copies[index];
      }
    }

    index += 1;
  }

  card_copies.iter().sum()
}
