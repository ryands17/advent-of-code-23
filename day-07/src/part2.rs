use std::{cmp::Ordering, collections::HashMap};

use lazy_static::lazy_static;

lazy_static! {
  static ref MAPPINGS: HashMap<char, char> = {
    let mut mappings = HashMap::new();
    mappings.insert('T', 'A');
    mappings.insert('Q', 'C');
    mappings.insert('K', 'D');
    mappings.insert('A', 'E');
    mappings.insert('J', '1');

    mappings
  };
}

fn map_rank(count: &[usize]) -> usize {
  if count.contains(&5) {
    return 6;
  }
  if count.contains(&4) {
    return 5;
  }
  if count.contains(&3) {
    if count.contains(&2) {
      return 4;
    }
    return 3;
  }
  if count.iter().filter(|c| *c == &2).count() == 4 {
    return 2;
  }
  if count.contains(&2) {
    return 1;
  }

  0
}

fn get_rank(hand: &str) -> usize {
  let hand = hand.chars().collect::<Vec<_>>();

  if hand.contains(&'J') {
    let mut cl = hand.clone();
    let j_count = cl.iter().filter(|h| *h == &'J').count();

    cl.retain(|c| c != &'J');

    let mut count = cl
      .iter()
      .map(|h| cl.iter().filter(|a| *a == h).count())
      .collect::<Vec<_>>();

    let m = count.iter().max().unwrap_or(&0) + j_count;
    count.push(m);
    return map_rank(&count);
  }

  let count = hand
    .iter()
    .map(|h| hand.iter().filter(|a| *a == h).count())
    .collect::<Vec<_>>();

  map_rank(&count)
}

fn map_str(s: &str) -> String {
  s.chars()
    .map(|c| *MAPPINGS.get(&c).unwrap_or(&c))
    .collect::<String>()
}

pub fn process(input: &str) -> usize {
  let mut result = input
    .lines()
    .map(|line| {
      let mut line = line.split(' ');
      let hand = line.next().unwrap();
      let bid = line.next().unwrap().parse::<usize>().unwrap();

      (hand, bid)
    })
    .collect::<Vec<_>>();

  result.sort_by(|a, b| {
    let a_rank = get_rank(a.0);
    let b_rank = get_rank(b.0);

    if a_rank == b_rank {
      return map_str(a.0).cmp(&map_str(b.0));
    }

    if a_rank > b_rank {
      return Ordering::Greater;
    }

    Ordering::Less
  });

  result
    .iter()
    .enumerate()
    .map(|(i, val)| val.1 * (i + 1))
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    assert_eq!(process(input), 5905);
  }
}
