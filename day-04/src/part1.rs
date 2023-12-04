pub fn process(input: &str) -> usize {
  let result = input
    .lines()
    .map(|line| {
      let all_numbers = line.split(':').last().unwrap().trim();
      let mut sp = all_numbers.split('|');

      let winning_numbers = sp
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .filter_map(|x| x.parse::<usize>().ok())
        .collect::<Vec<_>>();

      let lottery_numbers = sp
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .filter_map(|x| x.parse::<usize>().ok());

      let mut count = 0;
      for num in lottery_numbers {
        if winning_numbers.contains(&num) {
          count += 1;
        }
      }

      if count == 0 {
        0
      } else {
        (2 as usize).pow(count - 1)
      }
    })
    .sum::<_>();

  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    assert_eq!(process(input), 13);
  }
}
