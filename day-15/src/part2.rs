use std::collections::HashMap;

pub fn process(input: &str) -> usize {
  let mut boxes: Vec<Vec<&str>> = vec![vec![]; 256];
  let mut focal_lengths = HashMap::new();

  input.split(',').for_each(|val| {
    if val.contains('-') {
      let label = &val[..val.len() - 1];
      let index = hash(label);
      if let Some(i) = boxes[index].iter().position(|&x| x == label) {
        boxes[index].remove(i);
      }
    } else {
      let mut sp = val.split('=');
      let label = sp.next().unwrap();
      let length = sp.next().unwrap().parse::<usize>().unwrap();

      let index = hash(label);
      if !boxes[index].contains(&label) {
        boxes[index].push(label);
      }

      focal_lengths.insert(label, length);
    }
  });

  boxes
    .iter()
    .enumerate()
    .map(|(i, bx)| {
      bx.iter().enumerate().fold(0, |acc, (j, inner)| {
        acc + (i + 1) * (j + 1) * focal_lengths.get(inner).unwrap()
      })
    })
    .sum::<usize>()
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
    assert_eq!(process(input), 145);
  }
}
