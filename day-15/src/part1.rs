pub fn process(input: &str) -> usize {
  input.split(',').map(hash).sum::<usize>()
}

fn hash(s: &str) -> usize {
  s.chars().fold(0, |acc, ch| {
    let mut n = (ch as usize) + acc;
    n *= 17;
    n %= 256;
    n
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    assert_eq!(process(input), 1320);
  }
}
