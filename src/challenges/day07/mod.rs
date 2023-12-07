use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

const INPUT: &str = include_str!("input7.txt");

pub fn run() -> ((u128, f64), (u128, f64)) {
  let start_part1 = Instant::now();
  let result_part1 = part1();
  let duration_part1 = start_part1.elapsed();
  let duration_part1_in_ms = duration_part1.as_secs_f64() * 1000.0;

  // let start_part2 = Instant::now();
  // let result_part2 = part2();
  // let duration_part2 = start_part2.elapsed();
  // let duration_part2_in_ms = duration_part2.as_secs_f64() * 1000.0;

  (
    (result_part1 as u128, duration_part1_in_ms),
    // (result_part2 as u128, duration_part2_in_ms),
    (0, 0.0),
  )
}

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Ten,
  Jack,
  Queen,
  King,
  Ace,
}

impl Card {
  fn from_char(ch: char) -> Self {
    match ch {
      '2' => Self::Two,
      '3' => Self::Three,
      '4' => Self::Four,
      '5' => Self::Five,
      '6' => Self::Six,
      '7' => Self::Seven,
      '8' => Self::Eight,
      '9' => Self::Nine,
      'T' => Self::Ten,
      'J' => Self::Jack,
      'Q' => Self::Queen,
      'K' => Self::King,
      'A' => Self::Ace,
      _ => panic!("Invalid card character"),
    }
  }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
  HighCard,
  OnePair,
  TwoPair,
  ThreeOfAKind,
  FullHouse,
  FourOfAKind,
  FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
  cards: Vec<Card>,
  bet: u128,
}

impl Hand {
  fn new(cards: Vec<Card>, bet: u128) -> Self {
    Self { cards, bet }
  }

  fn get_hand_type(&self) -> HandType {
    let mut counts = HashMap::new();
    for &card in &self.cards {
      *counts.entry(card).or_insert(0) += 1;
    }

    match counts
      .values()
      .sorted()
      .rev()
      .collect::<Vec<_>>()
      .as_slice()
    {
      [5] => HandType::FiveOfAKind,
      [4, _] => HandType::FourOfAKind,
      [3, 2] => HandType::FullHouse,
      [3, 1, 1] => HandType::ThreeOfAKind,
      [2, 2, 1] => HandType::TwoPair,
      [2, 1, 1, 1] => HandType::OnePair,
      [1, 1, 1, 1, 1] => HandType::HighCard,
      _ => unreachable!(),
    }
  }
}

fn parse_hand(line: &str) -> Hand {
  let mut parts = line.split_whitespace();
  let cards = parts.next().unwrap().chars().map(Card::from_char).collect();
  let bet = parts.next().unwrap().parse().unwrap();
  Hand::new(cards, bet)
}

fn part1() -> u128 {
  let mut hands: Vec<Hand> = INPUT.lines().map(parse_hand).collect();

  hands.sort_by(|h1, h2| {
    let h1_type = h1.get_hand_type();
    let h2_type = h2.get_hand_type();
    if h1_type == h2_type {
      h1.cards.cmp(&h2.cards)
    } else {
      h1_type.cmp(&h2_type)
    }
  });

  hands
    .iter()
    .enumerate()
    .map(|(index, hand)| (index as u128 + 1) * hand.bet)
    .sum()
}
