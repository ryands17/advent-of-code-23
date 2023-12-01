use std::{collections::HashMap, iter};

fn word_to_digit() -> HashMap<&'static str, char> {
  let mut word_map = HashMap::new();

  word_map.insert("one", '1');
  word_map.insert("two", '2');
  word_map.insert("three", '3');
  word_map.insert("four", '4');
  word_map.insert("five", '5');
  word_map.insert("six", '6');
  word_map.insert("seven", '7');
  word_map.insert("eight", '8');
  word_map.insert("nine", '9');

  word_map
}

pub fn process(input: &str) -> usize {
  let output: usize = input
    .lines()
    .map(|line| {
      let mut i = 0;

      let converted_str = iter::from_fn(move || {
        let word_map = word_to_digit();
        let slice = &line[i..];
        let result: Option<char>;

        for (k, v) in &word_map {
          if slice.starts_with(k) {
            result = Some(*v);
            i += 1;
            return result;
          }
        }

        result = slice.chars().next();
        i += 1;
        result
      });

      converted_str.filter_map(|ch| ch.to_digit(10))
    })
    .map(|mut numbers| {
      let first = numbers.next().unwrap();
      let last = numbers.last().unwrap_or(first);

      format!("{first}{last}").parse::<usize>().unwrap_or(0)
    })
    .sum();

  output
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    assert_eq!(process(input), 281);
  }
}
