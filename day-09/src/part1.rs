pub fn process(input: &str) -> isize {
  input
    .lines()
    .map(|line| {
      line
        .split(' ')
        .filter_map(|x| x.parse::<isize>().ok())
        .collect::<Vec<_>>()
    })
    .map(|mut section| {
      let mut diffs = Vec::new();
      diffs.push(*section.iter().last().unwrap());

      while !section.iter().all(|x| x == &0) {
        section = section
          .windows(2)
          .map(|val| val[1] - val[0])
          .collect::<Vec<_>>();

        diffs.push(*section.iter().last().unwrap());
      }

      diffs.iter().sum::<isize>()
    })
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    assert_eq!(process(input), 114);
  }
}
