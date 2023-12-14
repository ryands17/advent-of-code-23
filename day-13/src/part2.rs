pub fn process(input: &str) -> usize {
  input
    .split("\n\n")
    .map(|pattern| {
      let grid = pattern.lines().map(|l| l.to_string()).collect::<Vec<_>>();
      let row_match = find_row_mirror(grid.clone());

      // transpose grid
      let grid = (0..grid[0].len())
        .map(|i| {
          grid
            .iter()
            .map(|g| g.chars().nth(i).unwrap())
            .collect::<String>()
        })
        .collect::<Vec<_>>();

      let col_match = find_row_mirror(grid);

      row_match * 100 + col_match
    })
    .sum()
}

fn find_row_mirror(grid: Vec<String>) -> usize {
  let len = grid.len();
  for i in 1..len {
    let mut above = Vec::from_iter(grid[..i].iter().cloned());
    above.reverse();
    let below = Vec::from_iter(grid[i..].iter().cloned());

    let total = above
      .iter()
      .zip(below)
      .map(|(a, b)| {
        a.chars()
          .zip(b.chars())
          .map(|(x, y)| if x == y { 0 } else { 1 })
          .sum::<usize>()
      })
      .sum::<usize>();

    if total == 1 {
      return i;
    }
  }

  0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

    assert_eq!(process(input), 400);
  }
}
