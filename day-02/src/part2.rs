use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
  static ref NUMBER_AND_WORD_MATCHER: Regex = Regex::new(r"(\d+) (\w+)").unwrap();
}

pub fn process(input: &str) -> usize {
  input
    .lines()
    .map(|line| {
      let line = line.split(':').last().unwrap().trim();

      let mut red_count = 1;
      let mut blue_count = 1;
      let mut green_count = 1;

      NUMBER_AND_WORD_MATCHER
        .captures_iter(line)
        .map(|val| {
          let count = val.get(1).unwrap().as_str().parse::<usize>().unwrap();
          let colour = val.get(2).unwrap().as_str();

          (count, colour)
        })
        .for_each(|(count, colour)| {
          if colour == "red" {
            red_count = std::cmp::max(red_count, count);
          } else if colour == "green" {
            green_count = std::cmp::max(green_count, count);
          } else if colour == "blue" {
            blue_count = std::cmp::max(blue_count, count);
          }
        });

      red_count * green_count * blue_count
    })
    .sum::<usize>()
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

    assert_eq!(process(input), 2286);
  }
}
