use std::collections::HashMap;

pub fn process(input: &str) -> usize {
  let mut game_size = HashMap::new();
  game_size.insert("red", 12);
  game_size.insert("green", 13);
  game_size.insert("blue", 14);

  let output: usize = input
    .lines()
    .map(|line| {
      let mut sp = line.split(':');

      let game = sp
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

      let rounds = sp.next().unwrap().trim();
      let rounds = rounds.split(';');

      let mut valid_games = rounds.map(|round| {
        let round = round.trim();
        let games = round.split(',');
        let mut games = games.map(|game| game.trim());

        let result = games.all(|game| {
          let mut sp = game.split(' ');

          let count = sp.next().unwrap().parse::<usize>().unwrap();
          let colour = sp.next().unwrap();

          if let Some(val) = game_size.get(colour) {
            return count <= *val;
          }

          return false;
        });

        result
      });

      let result = valid_games.all(|is_valid| is_valid);

      if result {
        game
      } else {
        0
      }
    })
    .sum();

  output
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_input() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    assert_eq!(process(input), 8);
  }
}
