use day_03::part1;

fn main() {
  let text = include_str!("../../input1.txt");
  let val = part1::process(text);

  println!("{val}");
}
