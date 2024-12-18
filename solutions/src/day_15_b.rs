use std::collections::HashSet;

extern crate test;

pub fn main(contents: String) -> u32 {
  push_boxes(contents)
}

#[derive(PartialEq, Clone, Copy, Eq, Debug)]
enum Thing {
  Wall,
  BoxLeft,
  BoxRight,
  Bot,
  Nothing,
}

fn map_thing(c: char) -> [Thing; 2] {
  match c {
    '#' => [Thing::Wall, Thing::Wall],
    '.' => [Thing::Nothing, Thing::Nothing],
    'O' => [Thing::BoxLeft, Thing::BoxRight],
    '@' => [Thing::Bot, Thing::Nothing],
    _ => panic!("Not a correct character"),
  }
}

#[derive(PartialEq, Clone, Copy, Eq)]
enum Move {
  LEFT,
  RIGHT,
  UP,
  DOWN,
}

fn map_move(c: char) -> Move {
  match c {
    '<' => Move::LEFT,
    '>' => Move::RIGHT,
    '^' => Move::UP,
    'v' => Move::DOWN,
    _ => panic!("Not a correct character"),
  }
}

fn push_boxes(contents: String) -> u32 {
  let mut map: Vec<Vec<Thing>> = Vec::<Vec<Thing>>::new();
  let mut mapping = true;
  let mut player_pos: (usize, usize) = (0, 0);
  contents
    .lines()
    .enumerate()
    .for_each(|(i, line)| {
      if line == "" {
        mapping = false;
        print_map(&map);
      } else if mapping {
        if let Some(j) = line.find("@") {
          player_pos = (i, j * 2);
        }
        map.push(line
          .chars()
          .flat_map(map_thing)
          .collect::<Vec<Thing>>()
        );
      } else {
        line
          .chars()
          .map(map_move)
          .for_each(|mov| {
            player_pos = match mov {
              Move::LEFT | Move::RIGHT => move_horizontal(&mut map, mov, player_pos),
              _ => move_up_or_down(&mut map, mov, player_pos),
            };
            if map.len() < 40 {
              print_map(&map);
            }
          });
      }
    });

  map
    .into_iter()
    .enumerate()
    .map(|(i, line)| line
      .into_iter()
      .enumerate()
      .filter(|(_, thing)| thing == &Thing::BoxLeft)
      .map(|(j, _)| {
        (i * 100 + j) as u32
      })
      .sum::<u32>()
    )
    .sum::<u32>()
}

fn move_horizontal(map: &mut Vec<Vec<Thing>>, mov: Move, player_pos: (usize, usize)) -> (usize, usize) {
  let mut next_pos = player_pos.clone();
  let mut distance = 0;
  while map[next_pos.0][next_pos.1] != Thing::Nothing {
    if map[next_pos.0][next_pos.1] == Thing::Wall {
      return player_pos;
    }
    distance += 1;
    next_pos = get_moved_pos(&mov, 1, &next_pos)
  }
  (0..distance)
    .rev()
    .for_each(|block| {
      let curr_pos = get_moved_pos(&mov, block, &player_pos);
      let forward_pos = get_moved_pos(&mov, block + 1, &player_pos);
      map[forward_pos.0][forward_pos.1] = map[curr_pos.0][curr_pos.1];
    });

  map[player_pos.0][player_pos.1] = Thing::Nothing;
  get_moved_pos(&mov, 1, &player_pos)
}

fn move_up_or_down(map: &mut Vec<Vec<Thing>>, mov: Move, player_pos: (usize, usize)) -> (usize, usize) {
  let mut fronts = vec![HashSet::from([Some(player_pos.clone())])];
  let mut front_length = 1u8;
  while front_length > 0 {
    let new_front = fronts
      .last()
      .unwrap()
      .iter()
      .flat_map(|some_pos| {
        let pos = some_pos.unwrap();
        let next_pos = get_moved_pos(&mov, 1, &pos);

        match map[next_pos.0][next_pos.1] {
          Thing::BoxLeft => HashSet::from([Some(next_pos), Some((next_pos.0, next_pos.1 + 1))]),
          Thing::BoxRight => HashSet::from([Some(next_pos), Some((next_pos.0, next_pos.1 - 1))]),
          Thing::Wall => HashSet::from([None]),
          Thing::Nothing => HashSet::from([]),
          Thing::Bot => HashSet::from([Some(next_pos)]),
        }
      })
      .collect::<HashSet<Option<(usize, usize)>>>();
    front_length = new_front.len() as u8;

    if new_front.contains(&None) {
      return player_pos;
    } else {
      fronts.push(new_front);
    }
  }

  fronts
    .into_iter()
    .rev()
    .for_each(|front| {
      front
        .into_iter()
        .for_each(|pos| {
          let curr_pos = pos.unwrap();
          let forward_pos = get_moved_pos(&mov, 1, &curr_pos);
          map[forward_pos.0][forward_pos.1] = map[curr_pos.0][curr_pos.1];
          map[curr_pos.0][curr_pos.1] = Thing::Nothing;
        });
    });

  map[player_pos.0][player_pos.1] = Thing::Nothing;
  get_moved_pos(&mov, 1, &player_pos)
}

fn print_map(map: &Vec<Vec<Thing>>) -> () {
  map
    .iter()
    .map(|line| line
      .iter()
      .map(pretty_thang)
      .collect::<String>()
    )
    .for_each(|line| {println!("{line}")});
}

fn pretty_thang(thing: &Thing) -> char {
  match thing {
    Thing::Bot => '@',
    Thing::BoxLeft => '[',
    Thing::BoxRight => ']',
    Thing::Wall => '#',
    Thing::Nothing => '.'
  }
}

fn get_moved_pos(mov: &Move, distance: usize, pos: &(usize, usize)) -> (usize, usize) {
  match mov {
    Move::UP => (pos.0-distance, pos.1),
    Move::DOWN => (pos.0+distance, pos.1),
    Move::LEFT => (pos.0, pos.1-distance),
    Move::RIGHT => (pos.0, pos.1+distance),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 15;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_15_b() {
    const EXAMPLE_ANSWER: Option<u32> = Some(9021);
    const ANSWER: Option<u32> = Some(1429013);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_15_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
