use num::integer::lcm;
use std::collections::HashMap;

pub fn process(input: &str) -> usize {
  let mut lines = input.lines();

  let pattern = lines.next().unwrap().chars().collect::<Vec<_>>();
  let end = pattern.len();
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

      (key, value)
    })
    .collect::<HashMap<_, _>>();

  mappings
    .keys()
    .filter(|k| k.ends_with('A'))
    .map(|start| {
      let mut start_val = *start;
      let mut steps = 0_usize;
      let mut pointer = 0_usize;

      while !start_val.ends_with('Z') {
        let val = mappings.get(&start_val).unwrap();
        let direction = pattern.get(pointer).unwrap();
        start_val = if direction == &'L' { &val.0 } else { &val.1 };

        pointer += 1;
        steps += 1;
        pointer = if pointer == end { 0 } else { pointer };
      }
      steps
    })
    .fold(1, lcm)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = "LR

    11A = (11B, XXX)
    11B = (XXX, 11Z)
    11Z = (11B, XXX)
    22A = (22B, XXX)
    22B = (22C, 22C)
    22C = (22Z, 22Z)
    22Z = (22B, 22B)
    XXX = (XXX, XXX)";

    assert_eq!(process(input), 6);
  }
}
