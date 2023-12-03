use std::{collections::HashMap, time::Instant};

const INPUT: &str = include_str!("input2.txt");

const LIMITS: [(&str, u32); 3] = [("red", 12), ("green", 13), ("blue", 14)];

pub fn run() -> ((u32, f64), (u32, f64)) {
  let start_part1 = Instant::now();
  let result_part1 = part1();
  let duration_part1 = start_part1.elapsed();
  let duration_part1_in_ms = duration_part1.as_secs_f64() * 1000.0;

  ((result_part1, duration_part1_in_ms), (0, 0.0))
}

fn part1() -> u32 {
  let mut sum = 0;

  for line in INPUT.lines() {
    if let Some(game_sum) = process_line(&line) {
      sum += game_sum;
    }
  }
  sum
}

fn process_line(line: &str) -> Option<u32> {
  let parts: Vec<&str> = line.split(": ").collect();
  let game_id: u32 = parts[0].split_whitespace().last()?.parse().ok()?;

  let color_counts = count_colors(parts[1]);
  if color_counts {
    return Some(game_id);
  }
  None
}

fn count_colors(colors: &str) -> bool {
  let sets = colors.split(';');
  for set in sets {
    let mut counts = HashMap::new();
    for color in set.split(',').map(|s| s.trim()) {
      let parts = color.split_whitespace().collect::<Vec<&str>>();
      let count: u32 = parts[0].parse().unwrap_or(0);
      *counts.entry(parts[1].to_string()).or_insert(0) += count;
    }

    if !is_below_threshold(&counts) {
      return false;
    }
  }
  true
}

fn is_below_threshold(counts: &HashMap<String, u32>) -> bool {
  for (color, limit) in LIMITS {
    if let Some(&count) = counts.get(color) {
      if count > limit {
        return false;
      }
    }
  }
  true
}
