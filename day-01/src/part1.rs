pub fn process(input: &str) -> usize {
  let output: usize = input
    .lines()
    .map(|line| {
      line
        .chars()
        .filter(|ch| ch.is_numeric())
        .collect::<Vec<_>>()
    })
    .map(|numbers| {
      let first = numbers.first().unwrap().clone();
      let last = numbers.last().unwrap().clone();

      format!("{first}{last}")
    })
    .map(|num| num.parse::<usize>().unwrap_or(0))
    .sum();

  output
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    assert_eq!(process(input), 142)
  }
}
