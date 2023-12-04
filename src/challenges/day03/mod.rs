use std::collections::HashSet;
use std::time::Instant;

const INPUT: &str = include_str!("input3.txt");

pub fn run() -> ((u32, f64), (u32, f64)) {
  let start_part1 = Instant::now();
  let result_part1 = part1();
  let duration_part1 = start_part1.elapsed();
  let duration_part1_in_ms = duration_part1.as_secs_f64() * 1000.0;

  ((result_part1, duration_part1_in_ms), (0, 0.0))
}

fn part1() -> u32 {
  let (mut numbers, symbols) = parse_input(INPUT);
  numbers.retain(|num| num.is_adjacent_to_symbol(&symbols));
  numbers.iter().map(|num| num.value).sum()
}

fn parse_input(input: &str) -> (Vec<Number>, HashSet<(i32, i32)>) {
  let mut numbers = Vec::new();
  let mut symbols = HashSet::new();
  let mut current_number: Option<Number> = None;

  for (row, line) in input.lines().enumerate() {
    for (col, ch) in line.chars().enumerate() {
      match ch {
        ch if ch.is_ascii_digit() => {
          current_number = Some(
            current_number
              .unwrap_or_default()
              .add_digit(row as i32, col as i32, ch),
          );
        }
        _ => {
          if let Some(num) = current_number.take() {
            numbers.push(num);
          }
          if ch != '.' {
            symbols.insert((row as i32, col as i32));
          }
        }
      }
    }
    // Add number if it's at the end of a line
    if let Some(num) = current_number.take() {
      numbers.push(num);
    }
  }

  (numbers, symbols)
}

#[derive(Default, Debug)]
struct Number {
  value: u32,
  adjacent_positions: HashSet<(i32, i32)>,
}

impl Number {
  fn add_digit(mut self, row: i32, col: i32, ch: char) -> Self {
    self.value = self.value * 10 + (ch as u8 - b'0') as u32;
    self
      .adjacent_positions
      .extend(Number::adjacent_positions(row, col));
    self
  }

  fn is_adjacent_to_symbol(&self, symbols: &HashSet<(i32, i32)>) -> bool {
    self
      .adjacent_positions
      .iter()
      .any(|pos| symbols.contains(pos))
  }

  fn adjacent_positions(row: i32, col: i32) -> Vec<(i32, i32)> {
    (-1..=1)
      .flat_map(|dx| (-1..=1).map(move |dy| (row + dx, col + dy)))
      .filter(|&(r, c)| !(r == row && c == col))
      .collect()
  }
}
