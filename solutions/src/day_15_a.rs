extern crate test;

pub fn main(contents: String) -> u32 {
  push_boxes(contents)
}

#[derive(PartialEq, Clone, Copy, Eq)]
enum Thing {
  WALL,
  BOX,
  BOT,
  NOTHING,
}

fn map_thing(c: char) -> Thing {
  match c {
    '#' => Thing::WALL,
    '.' => Thing::NOTHING,
    'O' => Thing::BOX,
    '@' => Thing::BOT,
    _ => panic!("Not a correct character"),
  }
}

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
      } else if mapping {
        if let Some(j) = line.find("@") {
          player_pos = (i, j);
        }
        map.push(line
          .chars()
          .map(map_thing)
          .collect::<Vec<Thing>>()
        );
      } else {
        line
          .chars()
          .map(map_move)
          .for_each(|mov| {
            player_pos = move_bot(&mut map, mov, player_pos);
          });
      }
    });

  map
    .into_iter()
    .enumerate()
    .map(|(i, line)| line
      .into_iter()
      .enumerate()
      .filter(|(_, thing)| thing == &Thing::BOX)
      .map(|(j, _)| {
        (i * 100 + j) as u32
      })
      .sum::<u32>()
    )
    .sum::<u32>()
}

fn move_bot(map: &mut Vec<Vec<Thing>>, mov: Move, player_pos: (usize, usize)) -> (usize, usize) {
  let mut next_pos = player_pos.clone();
  let mut distance = 0;
  while map[next_pos.0][next_pos.1] != Thing::NOTHING {
    if map[next_pos.0][next_pos.1] == Thing::WALL {
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

  map[player_pos.0][player_pos.1] = Thing::NOTHING;
  get_moved_pos(&mov, 1, &player_pos)
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
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_15_a() {
    const EXAMPLE_ANSWER: Option<u32> = Some(10092);
    const ANSWER: Option<u32> = Some(1406392);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_15_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
