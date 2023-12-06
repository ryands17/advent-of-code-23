pub fn process(input: &str) -> usize {
  let mut input = input.split("\n\n");

  let seeds = input
    .next()
    .unwrap()
    .split(':')
    .last()
    .unwrap()
    .split(' ')
    .filter_map(|x| x.parse::<usize>().ok())
    .collect::<Vec<_>>();

  let mut seeds = (0..seeds.len())
    .step_by(2)
    .map(|i| {
      (
        *seeds.get(i).unwrap(),
        seeds.get(i + 1).unwrap() + seeds.get(i).unwrap(),
      )
    })
    .collect::<Vec<_>>();

  for block in input {
    let mut ranges: Vec<(usize, usize, usize)> = Vec::new();

    let mut block = block.lines();
    block.next();
    for line in block {
      let mut nums = line.split(' ').filter_map(|x| x.parse::<usize>().ok());

      ranges.push((
        nums.next().unwrap(),
        nums.next().unwrap(),
        nums.next().unwrap(),
      ));
    }

    let mut mappings: Vec<(usize, usize)> = Vec::new();
    while seeds.len() > 0 {
      let (start, end) = seeds.pop().unwrap();
      let mut is_match = false;

      for (to, from, offset) in &ranges {
        let offset_start = start.max(*from);
        let offset_end = end.min(*from + *offset);

        if offset_start < offset_end {
          mappings.push((offset_start - *from + *to, offset_end - *from + *to));

          if offset_start > start {
            seeds.push((offset_start, start));
          }

          if offset_end < end {
            seeds.push((offset_end, end));
          }

          is_match = true;
          break;
        }
      }

      if !is_match {
        mappings.push((start, end));
      }
    }

    seeds = mappings;
  }

  let (start, end) = seeds.iter().min().unwrap();

  start.min(end);
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

    assert_eq!(process(input), 46);
  }
}
