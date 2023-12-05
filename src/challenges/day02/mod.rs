use std::time::Instant;

const INPUT: &str = include_str!("input2.txt");

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
    (result_part2, duration_part2_in_ms),
  )
}

// red, green, blue
const LIMITS: [(usize, u128); 3] = [(0, 12), (1, 13), (2, 14)];

fn part1() -> u128 {
  INPUT
    .lines()
    .filter_map(|line| {
      let (game_id_str, colors) = line.split_once(": ")?;
      let game_id: u128 =
        game_id_str.split_whitespace().last()?.parse().ok()?;
      for set in colors.split(';') {
        let mut counts = [0; 3];

        for color in set.split(',').map(str::trim) {
          let mut parts = color.split_whitespace();
          let count: u128 = parts.next().unwrap().parse().unwrap_or(0);
          let color_index = match parts.next().unwrap() {
            "red" => 0,
            "green" => 1,
            "blue" => 2,
            _ => return None,
          };
          counts[color_index] += count;
        }

        if !LIMITS.iter().all(|&(idx, limit)| counts[idx] <= limit) {
          return None;
        }
      }

      Some(game_id)
    })
    .sum()
}

fn part2() -> u128 {
  INPUT
    .lines()
    .map(|line| {
      let (_, colors) = line.split_once(": ").unwrap();

      let mut max_counts = [0; 3];

      for set in colors.split(';') {
        let mut current_counts = [0; 3];

        for color in set.split(',').map(str::trim) {
          let mut parts = color.split_whitespace();
          let count: u128 = parts.next().unwrap().parse().unwrap_or(0);
          let color_index = match parts.next().unwrap() {
            "red" => 0,
            "green" => 1,
            "blue" => 2,
            _ => return 0,
          };
          current_counts[color_index] += count;
        }

        for i in 0..3 {
          max_counts[i] = max_counts[i].max(current_counts[i]);
        }
      }

      max_counts.iter().product()
    })
    .sum()
}
