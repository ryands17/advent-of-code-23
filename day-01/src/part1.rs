pub fn process(input: &str) -> usize {
  let output: usize = input
    .lines()
    .map(|line| line.chars().filter_map(|ch| ch.to_digit(10)))
    .map(|mut numbers| {
      let first = numbers.next().unwrap();
      let last = numbers.last().unwrap_or(first);

      println!("{first}{last}");

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
    let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    assert_eq!(process(input), 142);
  }
}
