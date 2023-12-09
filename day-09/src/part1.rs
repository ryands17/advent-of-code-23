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
      let mut i = 0;
      std::iter::from_fn(|| {
        if i == 0 {
          i += 1;
          return Some(*section.last().unwrap());
        }

        section = section.windows(2).map(|v| v[1] - v[0]).collect::<Vec<_>>();
        if section.iter().all(|v| v == &0) {
          return None;
        }

        return Some(*section.last().unwrap());
      })
      .sum::<isize>()
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
