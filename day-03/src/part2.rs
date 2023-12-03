use std::collections::HashSet;

use regex::Regex;

pub fn process(input: &str) -> usize {
  let re = Regex::new(r"(\d+)").unwrap();

  let lines = input.lines().collect::<Vec<_>>();
  let vector_start = 0;
  let vector_end = lines.len() - 1;

  let mut sum = 0;
  for (i, line) in lines.iter().enumerate() {
    let line_start = 0;
    let line_end = line.len();

    let result = line
      .match_indices('*')
      .filter_map(|(pos, _)| {
        let mut gears = HashSet::new();

        // check left
        if pos > line_start {
          if (&line).chars().nth(pos - 1).unwrap().is_digit(10) {
            let val = re
              .find_iter(&line[..pos])
              .last()
              .unwrap()
              .as_str()
              .parse::<usize>()
              .unwrap();

            gears.insert(val);
          }
        }

        // check right
        if pos < line_end {
          if (&line).chars().nth(pos + 1).unwrap().is_digit(10) {
            let val = re
              .find_iter(&line[pos + 1..])
              .next()
              .unwrap()
              .as_str()
              .parse::<usize>()
              .unwrap();

            gears.insert(val);
          }
        }

        // check top and diagonal
        if i > vector_start {
          let top_line = lines.get(i - 1).unwrap();
          re.find_iter(&top_line).for_each(|mt| {
            let diagonal = (pos - 1, pos + 1);
            let start = mt.start();
            let end = mt.end() - 1;

            if (start <= pos && pos <= end)
              || (start == diagonal.0 || start == diagonal.1)
              || (end == diagonal.0 || end == diagonal.1)
            {
              gears.insert(mt.as_str().parse::<usize>().unwrap());
            }
          })
        }

        // check bottom and diagonal
        if i < vector_end {
          let bottom_line = lines.get(i + 1).unwrap();
          re.find_iter(&bottom_line).for_each(|mt| {
            let diagonal = (pos - 1, pos + 1);
            let start = mt.start();
            let end = mt.end() - 1;

            if (start <= pos && pos <= end)
              || (start == diagonal.0 || start == diagonal.1)
              || (end == diagonal.0 || end == diagonal.1)
            {
              gears.insert(mt.as_str().parse::<usize>().unwrap());
            }
          })
        }

        if gears.len() == 2 {
          Some(gears)
        } else {
          None
        }
      })
      .map(|gear| gear.iter().fold(1, |acc, val| acc * val))
      .sum::<usize>();

    sum += result;
  }

  sum
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    assert_eq!(process(input), 467835);
  }
}
