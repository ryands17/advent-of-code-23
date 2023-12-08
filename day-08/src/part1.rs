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

      let coordinates = sp
        .next()
        .unwrap()
        .trim()
        .chars()
        .filter(|c| c != &'(' && c != &')')
        .collect::<String>();

      let mut sp = coordinates.split(',');
      let value = (
        sp.next().unwrap().trim().to_string(),
        sp.next().unwrap().trim().to_string(),
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
    if direction == &'L' {
      start_val = &val.0;
    } else {
      start_val = &val.1;
    }
    pointer += 1;
    steps += 1;

    if pointer == end {
      pointer = 0;
    }
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
