use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
  static ref NUMBER_MATCHER: Regex = Regex::new(r"(\d+)").unwrap();
  static ref NUMBER_AND_WORD_MATCHER: Regex = Regex::new(r"(\d+) (\w+)").unwrap();
}

pub fn process(input: &str) -> usize {
  let mut game_size = HashMap::new();
  game_size.insert("red", 12);
  game_size.insert("green", 13);
  game_size.insert("blue", 14);

  input
    .lines()
    .map(|line| {
      let mut sp = line.split(':');

      let game = NUMBER_MATCHER
        .find(sp.next().unwrap())
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();

      let rounds = sp.next().unwrap().trim();

      let result = NUMBER_AND_WORD_MATCHER
        .captures_iter(rounds)
        .map(|val| {
          let count = val.get(1).unwrap().as_str().parse::<usize>().unwrap();
          let colour = val.get(2).unwrap().as_str();

          (count, colour)
        })
        .all(|val| {
          if let Some(c) = game_size.get(val.1) {
            return val.0 <= *c;
          }

          false
        });

      if result {
        game
      } else {
        0
      }
    })
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    assert_eq!(process(input), 8);
  }
}
