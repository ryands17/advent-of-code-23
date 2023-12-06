pub fn process(input: &str) -> usize {
  let mut lines = input.lines();
  let seeds = lines.next().unwrap().split(':').last().unwrap().split(' ');
  let mappings = lines.filter(|l| !l.is_empty()).collect::<Vec<_>>();

  seeds
    .filter_map(|x| x.parse::<usize>().ok())
    .map(|seed| {
      let mut val = seed;
      let mut is_set = false;

      mappings.iter().for_each(|line| {
        if !line.chars().next().unwrap().is_alphabetic() {
          let mut numbers = line.split(' ').filter_map(|x| x.parse::<usize>().ok());
          let to = numbers.next().unwrap();
          let from = numbers.next().unwrap();
          let range = numbers.next().unwrap();

          if (from..(from + range)).contains(&val) && !is_set {
            val = val - from + to;
            is_set = true;
          }
        } else {
          is_set = false;
        }
      });

      val
    })
    .min()
    .unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    assert_eq!(process(input), 35);
  }
}
