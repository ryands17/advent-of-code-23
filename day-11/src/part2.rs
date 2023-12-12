pub fn process(input: &str) -> usize {
  let space = input
    .lines()
    .map(|line| line.chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();

  let empty_rows = space
    .iter()
    .enumerate()
    .filter_map(|(i, line)| {
      if line.iter().all(|ch| ch == &'.') {
        return Some(i);
      }
      None
    })
    .collect::<Vec<_>>();

  let empty_cols = (0..space.len())
    .map(|i| space.iter().map(|sp| sp[i]).collect::<Vec<_>>())
    .enumerate()
    .filter_map(|(i, line)| {
      if line.iter().all(|ch| ch == &'.') {
        return Some(i);
      }
      None
    })
    .collect::<Vec<_>>();

  let galaxies = space
    .iter()
    .enumerate()
    .flat_map(|(row, line)| {
      line.iter().enumerate().filter_map(move |(col, ch)| {
        if ch == &'#' {
          return Some((row, col));
        }
        None
      })
    })
    .collect::<Vec<_>>();

  let expansion = 1000000_usize;

  galaxies
    .iter()
    .enumerate()
    .map(|(i, (row1, col1))| {
      galaxies[..i]
        .iter()
        .map(|(row2, col2)| {
          let mut total = 0_usize;

          for row in *row1.min(row2)..*row1.max(row2) {
            total += if empty_rows.contains(&row) {
              expansion
            } else {
              1
            };
          }

          for col in *col1.min(col2)..*col1.max(col2) {
            total += if empty_cols.contains(&col) {
              expansion
            } else {
              1
            };
          }
          total
        })
        .sum::<usize>()
    })
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    assert_eq!(process(input), 82000210);
  }
}
