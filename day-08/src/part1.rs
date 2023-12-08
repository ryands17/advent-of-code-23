use std::collections::HashMap;

pub fn process(input: &str) -> usize {
  let mut lines = input.lines();

  let pattern = lines.next().unwrap().chars().collect::<Vec<_>>();
  let mut start_val: &str = "AAA";
  lines.next();

  let mappings = lines
    .map(|line| {
      let mut sp = line.split('=');

      let key = sp.next().unwrap().trim();
      let mut coordinates = sp.next().unwrap().trim()[1..][..8].split(',');
      let value = (
        coordinates.next().unwrap().trim(),
        coordinates.next().unwrap().trim(),
      );

      (key, value.to_owned())
    })
    .collect::<HashMap<_, _>>();

  let mut steps = 0_usize;
  let mut pointer = 0_usize;
  let end = pattern.len();

  while start_val != "ZZZ" {
    let val = mappings.get(&start_val).unwrap();
    let direction = pattern.get(pointer).unwrap();
    start_val = if direction == &'L' { &val.0 } else { &val.1 };

    pointer += 1;
    steps += 1;

    pointer = if pointer == end { 0 } else { pointer };
  }

  steps
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input_1() {
    let input = "RL

    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)";

    assert_eq!(process(input), 2);
  }

  #[test]
  fn sample_input_2() {
    let input = "LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)";

    assert_eq!(process(input), 6);
  }
}
