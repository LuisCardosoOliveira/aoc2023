use std::time::Instant;

const INPUT: &str = include_str!("input2.txt");

pub fn run() -> ((u32, f64), (u32, f64)) {
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
    (result_part2, duration_part2_in_ms),
  )
}

// red, green, blue
const LIMITS: [(usize, u32); 3] = [(0, 12), (1, 13), (2, 14)];

fn part1() -> u32 {
  INPUT.lines().filter_map(process_line).sum()
}

fn process_line(line: &str) -> Option<u32> {
  let (game_id_str, colors) = line.split_once(": ")?;
  let game_id: u32 = game_id_str.split_whitespace().last()?.parse().ok()?;

  let mut counts = [0; 3];
  if check_game_feasibility(colors, &mut counts) {
    Some(game_id)
  } else {
    None
  }
}

fn check_game_feasibility(colors: &str, counts: &mut [u32; 3]) -> bool {
  for set in colors.split(';') {
    // Reset counts for each set to avoid allocate a new array
    *counts = [0; 3];

    for color in set.split(',').map(str::trim) {
      let mut parts = color.split_whitespace();
      let count: u32 = parts.next().unwrap().parse().unwrap_or(0);
      let color_index = match parts.next().unwrap() {
        "red" => 0,
        "green" => 1,
        "blue" => 2,
        _ => return false,
      };
      counts[color_index] += count;
    }

    if !is_below_threshold(counts) {
      return false;
    }
  }
  true
}

fn is_below_threshold(counts: &[u32; 3]) -> bool {
  LIMITS.iter().all(|&(idx, limit)| counts[idx] <= limit)
}

fn part2() -> u32 {
  INPUT.lines().map(calculate_minimum_set_power).sum()
}

fn calculate_minimum_set_power(line: &str) -> u32 {
  let (_, colors) = line.split_once(": ").unwrap();

  let mut max_counts = [0; 3];
  for set in colors.split(';') {
    let mut current_counts = [0; 3];

    for color in set.split(',').map(str::trim) {
      let mut parts = color.split_whitespace();
      let count: u32 = parts.next().unwrap().parse().unwrap_or(0);
      let color_index = match parts.next().unwrap() {
        "red" => 0,
        "green" => 1,
        "blue" => 2,
        _ => return 0,
      };
      current_counts[color_index] += count;
    }

    // Update max_counts with the maximum counts seen so far
    for i in 0..3 {
      max_counts[i] = max_counts[i].max(current_counts[i]);
    }
  }

  // Calculate the power of the minimum set (product of the counts of red, green, and blue cubes)
  max_counts.iter().product()
}
