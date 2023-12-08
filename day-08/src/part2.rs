use num::integer::lcm;
use std::collections::HashMap;

pub fn process(input: &str) -> usize {
  let mut lines = input.lines();

  let pattern = lines.next().unwrap().chars().collect::<Vec<_>>();
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

  mappings
    .keys()
    .filter(|k| k.ends_with('A'))
    .map(|start| {
      let mut start_val = *start;
      let mut steps = 0_usize;
      let mut pointer = 0_usize;
      let end = pattern.len();

      while !start_val.ends_with('Z') {
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
          pointer = 0_usize;
        }
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
