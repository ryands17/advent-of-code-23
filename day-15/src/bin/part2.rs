use day_15::part2;

fn main() {
  let text = include_str!("../../input2.txt");
  let val = part2::process(text);

  println!("{val}");
}
