use std::time::Instant;

const INPUT: &str = include_str!("input1.txt");

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

fn part1() -> u32 {
  let mut sum = 0;

  for line in INPUT.lines() {
    let mut first_digit = None;
    let mut last_digit = None;

    for c in line.chars() {
      if c.is_digit(10) {
        last_digit = Some(c.to_digit(10).unwrap_or(0));
        if first_digit.is_none() {
          first_digit = last_digit;
        }
      }
    }

    if let (Some(first), Some(last)) = (first_digit, last_digit) {
      sum += first * 10 + last;
    }
  }

  sum
}

const DIGITS: &[&[u8]] = &[
  b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight",
  b"nine",
];

fn part2() -> u32 {
  let mut result = 0;
  for line in INPUT.lines() {
    let line_bytes = line.as_bytes();
    let mut digits =
      (0..line_bytes.len()).filter_map(|i| match line_bytes[i] {
        b'0'..=b'9' => Some((line_bytes[i] - b'0') as usize),
        _ => DIGITS
          .iter()
          .enumerate()
          .find_map(|(di, d)| line_bytes[i..].starts_with(d).then_some(di + 1)),
      });
    let a = digits.next().unwrap();
    let b = digits.last().unwrap_or(a);
    result += a * 10 + b;
  }
  result as u32
}
