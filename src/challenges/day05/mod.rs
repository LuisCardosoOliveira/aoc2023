use itertools::Itertools;
use std::time::Instant;

const INPUT: &str = include_str!("input5.txt");

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

fn part2() -> usize {
  let mut lines = INPUT.lines();

  let seeds: Vec<usize> = lines
    .next()
    .unwrap_or("")
    .strip_prefix("seeds: ")
    .unwrap_or("")
    .split_whitespace()
    .filter_map(|n| n.parse().ok())
    .collect();

  // Skip the empty line
  lines.next();

  // Process mappings
  let mut layers = Vec::new();
  let mut current_layer = Vec::new();
  for line in lines {
    if line.trim().is_empty() {
      layers.push(current_layer);
      current_layer = Vec::new();
      continue;
    }

    let nums: Vec<usize> = line
      .split_whitespace()
      .filter_map(|n| n.parse().ok())
      .collect();
    if nums.len() == 3 {
      current_layer.push((nums[0], nums[1], nums[2]));
    }
  }
  if !current_layer.is_empty() {
    layers.push(current_layer);
  }

  find_minimum_location(seeds, &layers)
}

fn find_minimum_location(
  seeds: Vec<usize>,
  layers: &[Vec<(usize, usize, usize)>],
) -> usize {
  let seeds = seeds
    .into_iter()
    .tuples()
    .map(|(a, len)| (a, a + len))
    .collect::<Vec<_>>();

  let locations = layers.iter().fold(seeds, |seeds, mappings| {
    seeds
      .iter()
      .flat_map(|&(start, end)| {
        let mut mapped = Vec::new();
        let mut unmapped = vec![(start, end)];
        for &(dst, src, len) in mappings {
          let mut m = Vec::new();
          for (start, end) in unmapped {
            let a = (start, end.min(src));
            let b = (start.max(src), (src + len).min(end));
            let c = ((src + len).max(start), end);
            if a.0 < a.1 {
              m.push(a);
            }
            if b.0 < b.1 {
              mapped.push((b.0 - src + dst, b.1 - src + dst));
            }
            if c.0 < c.1 {
              m.push(c);
            }
          }
          unmapped = m;
        }
        mapped.extend(unmapped);
        mapped
      })
      .collect()
  });

  locations.iter().map(|r| r.0).min().unwrap()
}
