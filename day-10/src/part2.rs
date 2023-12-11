pub fn process(input: &str) -> isize {
  println!("{:?}", input);
  42
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    assert_eq!(process(input), 42);
  }
}
