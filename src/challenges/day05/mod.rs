use std::time::Instant;

const INPUT: &str = include_str!("input5.txt");

pub fn run() -> ((u128, f64), (u128, f64)) {
  let start_part1 = Instant::now();
  let result_part1 = part1();
  let duration_part1 = start_part1.elapsed();
  let duration_part1_in_ms = duration_part1.as_secs_f64() * 1000.0;

  // let start_part2 = Instant::now();
  // let result_part2 = part2();
  // let duration_part2 = start_part2.elapsed();
  // let duration_part2_in_ms = duration_part2.as_secs_f64() * 1000.0;

  ((result_part1, duration_part1_in_ms), (0 as u128, 0.0))
}

fn part1() -> u128 {
  let mut lines = INPUT.lines();

  // Parse seeds
  let seeds: Vec<u128> = lines
    .next()
    .unwrap()
    .strip_prefix("seeds: ")
    .unwrap()
    .split_whitespace()
    .filter_map(|n| n.parse().ok())
    .collect();

  // Skip the empty line
  lines.next();

  // Process mappings
  let mut maps = Vec::new();
  let mut current_map = Vec::new();
  for line in lines {
    if line.trim().is_empty() {
      if !current_map.is_empty() {
        maps.push(current_map);
        current_map = Vec::new();
      }
      continue;
    }

    let nums: Vec<u128> = line
      .split_whitespace()
      .filter_map(|n| n.parse().ok())
      .collect();
    if nums.len() == 3 {
      current_map.push((nums[0], nums[1], nums[2]));
    }
  }
  if !current_map.is_empty() {
    maps.push(current_map);
  }

  seeds
    .into_iter()
    .map(|seed| map_through_categories(seed, &maps))
    .min()
    .unwrap()
}

fn map_through_categories(
  mut number: u128,
  maps: &[Vec<(u128, u128, u128)>],
) -> u128 {
  for map in maps {
    number = map_number(number, map);
  }
  number
}

fn map_number(number: u128, map: &[(u128, u128, u128)]) -> u128 {
  for &(dest_start, src_start, range_len) in map {
    if number >= src_start && number < src_start + range_len {
      return dest_start + (number - src_start);
    }
  }
  number
}
