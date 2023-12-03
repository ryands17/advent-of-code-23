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

    let result = re
      .find_iter(line)
      .filter_map(|mt| {
        let start_index = mt.start();
        let end_index = mt.end();
        let val = mt.as_str().parse::<usize>().unwrap();

        // left
        if start_index > line_start {
          let ch = &line[start_index - 1..start_index];
          if matcher(ch) {
            return Some(val);
          }
        }

        // right
        if end_index < line_end {
          let ch = &line[end_index..end_index + 1];
          if matcher(ch) {
            return Some(val);
          }
        }

        // top and diagonal
        if i > vector_start {
          let start_index = if start_index > line_start {
            start_index - 1
          } else {
            start_index
          };

          let end_index = if end_index < line_end {
            end_index + 1
          } else {
            end_index
          };

          let ch = &lines.get(i - 1).unwrap()[start_index..end_index];
          if matcher(ch) {
            return Some(val);
          }
        }

        // bottom and diagonal
        if i < vector_end {
          let start_index = if start_index > line_start {
            start_index - 1
          } else {
            start_index
          };

          let end_index = if end_index < line_end {
            end_index + 1
          } else {
            end_index
          };

          let ch = &lines.get(i + 1).unwrap()[start_index..end_index];
          if matcher(ch) {
            return Some(val);
          }
        }

        None
      })
      .sum::<usize>();

    sum += result;
  }

  sum
}

fn matcher(val: &str) -> bool {
  let re = Regex::new(r"[^0-9.]").unwrap();
  re.is_match(val)
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

    assert_eq!(process(input), 4361);
  }
}
