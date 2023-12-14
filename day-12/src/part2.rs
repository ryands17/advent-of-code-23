pub fn process(input: &str) -> usize {
  let repeat = 5;

  let data = input
    .lines()
    .map(|line| {
      let mut sp = line.split(' ');
      let arrangement = sp.next().unwrap();
      let arrangement = (0..repeat)
        .map(|_| arrangement.to_string())
        .collect::<Vec<_>>()
        .join("?");

      let nums = sp.next().unwrap();
      let mut nums = (0..repeat)
        .map(|_| nums.to_string())
        .collect::<Vec<_>>()
        .join(",")
        .split(',')
        .filter_map(|x| x.parse::<usize>().ok())
        .flat_map(|val| {
          let mut v = (0..val).map(|_| true).collect::<Vec<_>>();
          v.push(false);
          v
        })
        .collect::<Vec<_>>();

      nums.insert(0, false);

      (format!(".{arrangement}."), nums)
    })
    .collect::<Vec<_>>();

  data.iter().map(|v| count_arrangements(&v.0, &v.1)).sum()
}

pub fn count_arrangements(arrangement: &String, springs: &Vec<bool>) -> usize {
  let n = arrangement.len();
  let m = springs.len();

  let mut dp = vec![vec![0_usize; m + 1]; n + 1];
  dp[n][m] = 1;

  for i in (0..=n - 1).rev() {
    for j in (0..=m - 1).rev() {
      let (damaged, operational) = match arrangement.chars().nth(i).unwrap() {
        '#' => (true, false),
        '.' => (false, true),
        _ => (true, true),
      };

      let mut sum = 0;
      let v = dp[i + 1][j + 1];

      if damaged && springs[j] {
        sum += v;
      } else if operational && !springs[j] {
        sum += v + dp[i + 1][j];
      }

      dp[i][j] = sum;
    }
  }

  dp[0][0]
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    assert_eq!(process(input), 525152);
  }
}
