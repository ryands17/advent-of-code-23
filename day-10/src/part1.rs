use std::collections::{HashSet, VecDeque};

pub fn process(input: &str) -> usize {
  let lines = input.lines().collect::<Vec<_>>();
  let row_len = lines.len() - 1;
  let col_len = lines.first().unwrap().len() - 1;
  let mut start_pos: (usize, usize) = (0, 0);

  'outer: for (i, line) in lines.iter().enumerate() {
    for (j, ch) in line.chars().enumerate() {
      if ch == 'S' {
        start_pos = (i, j);
        break 'outer;
      }
    }
  }

  let mut queue = VecDeque::new();
  queue.push_back(start_pos);
  let mut visited: HashSet<(usize, usize)> = HashSet::new();

  while !queue.is_empty() {
    let (row, col) = queue.pop_front().unwrap();
    let current_char = lines.get(row).unwrap().chars().nth(col).unwrap();

    // check top
    if row > 0 && "S|JL".contains(current_char) {
      let top_char = lines.get(row - 1).unwrap().chars().nth(col).unwrap();
      if "|7F".contains(top_char) && !visited.contains(&(row - 1, col)) {
        visited.insert((row - 1, col));
        queue.push_back((row - 1, col));
      }
    }

    // check bottom
    if row < row_len && "S|7F".contains(current_char) {
      let bottom_char = lines.get(row + 1).unwrap().chars().nth(col).unwrap();
      if "|JL".contains(bottom_char) && !visited.contains(&(row + 1, col)) {
        visited.insert((row + 1, col));
        queue.push_back((row + 1, col));
      }
    }

    // check left
    if col > 0 && "S-J7".contains(current_char) {
      let left_char = lines.get(row).unwrap().chars().nth(col - 1).unwrap();
      if "-LF".contains(left_char) && !visited.contains(&(row, col - 1)) {
        visited.insert((row, col - 1));
        queue.push_back((row, col - 1));
      }
    }

    // check right
    if col < col_len && "S-LF".contains(current_char) {
      let right_char = lines.get(row).unwrap().chars().nth(col + 1).unwrap();
      if "-J7".contains(right_char) && !visited.contains(&(row, col + 1)) {
        visited.insert((row, col + 1));
        queue.push_back((row, col + 1));
      }
    }
  }

  num::integer::div_ceil(visited.len(), 2)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input_1() {
    let input = ".....
.S-7.
.|.|.
.L-J.
.....";

    assert_eq!(process(input), 4);
  }

  #[test]
  fn sample_input_2() {
    let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    assert_eq!(process(input), 8);
  }
}
