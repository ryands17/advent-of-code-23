pub fn process(input: &str) -> usize {
  let mut lines = input.lines();

  let time = lines
    .next()
    .unwrap()
    .split(':')
    .last()
    .unwrap()
    .replace(' ', "")
    .parse::<usize>()
    .unwrap();

  let distance = lines
    .next()
    .unwrap()
    .split(':')
    .last()
    .unwrap()
    .replace(' ', "")
    .parse::<usize>()
    .unwrap();

  (0..time)
    .filter(|t| {
      let computed_distance = t * (time - t);
      computed_distance > distance
    })
    .count()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = "Time:      7  15   30
Distance:  9  40  200";

    assert_eq!(process(input), 71503);
  }
}
