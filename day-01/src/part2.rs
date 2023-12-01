use std::iter;

pub fn process(input: &str) -> usize {
  let output: usize = input
    .lines()
    .map(|line| {
      let mut i = 0;

      let converted_str = iter::from_fn(move || {
        let slice = &line[i..];

        let result: Option<char>;
        if slice.starts_with("one") {
          result = Some('1');
        } else if slice.starts_with("two") {
          result = Some('2');
        } else if slice.starts_with("three") {
          result = Some('3');
        } else if slice.starts_with("four") {
          result = Some('4');
        } else if slice.starts_with("five") {
          result = Some('5');
        } else if slice.starts_with("six") {
          result = Some('6');
        } else if slice.starts_with("seven") {
          result = Some('7');
        } else if slice.starts_with("eight") {
          result = Some('8');
        } else if slice.starts_with("nine") {
          result = Some('9');
        } else {
          result = slice.chars().next();
        }

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
