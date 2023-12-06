pub fn process(input: &str) -> usize {
  let mut lines = input.lines();

  let times = lines
    .next()
    .unwrap()
    .split(':')
    .last()
    .unwrap()
    .split(' ')
    .filter_map(|x| x.parse::<usize>().ok());

  lines
    .next()
    .unwrap()
    .split(':')
    .last()
    .unwrap()
    .split(' ')
    .filter_map(|x| x.parse::<usize>().ok())
    .zip(times)
    .map(|(distance, time)| {
      (0..time)
        .filter(|t| {
          let covered_distance = t * (time - t);

          covered_distance > distance
        })
        .count()
    })
    .product()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = "Time:      7  15   30
Distance:  9  40  200";

    assert_eq!(process(input), 288);
  }
}
