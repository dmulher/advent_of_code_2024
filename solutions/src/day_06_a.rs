use std::collections::{HashMap, HashSet};

extern crate test;

pub fn main(contents: String) -> u32 {
  get_instructions(contents)
}

enum Dir {
  NORTH,
  SOUTH,
  WEST,
  EAST,
}

fn get_instructions(contents: String) -> u32 {
  let row_map = contents
    .lines()
    .enumerate()
    .map(|(y, line)|{
      (y, line
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '#')
        .map(|(x, _)| x)
        .collect::<Vec<usize>>()
      )
    })
    .collect::<HashMap<usize, Vec<usize>>>();

  let mut col_map: HashMap<usize, Vec<usize>> = HashMap::new();

  for (y, row) in row_map.iter() {
    for x in row.iter() {
      col_map.entry(*x).or_insert(vec![]).push(*y);
    }
  }

  col_map.values_mut().for_each(|col| col.sort());

  let mut start: Option<(usize, usize)> = None;

  for (y, line) in contents.lines().enumerate() {
    if line.contains('^') {
      if let Some(x) = line.chars().position(|c| c == '^') {
        start = Some((x, y));
      }
    }
  }

  let max = (contents.lines().next().unwrap().len() - 1, contents.lines().enumerate().last().unwrap().0);

  let mut pos = start.unwrap();
  let mut dir = Dir::NORTH;
  let mut complete = false;
  let mut path: HashSet<(usize, usize)> = HashSet::new();
  path.insert(pos);

  while !complete {
    let mut next_pos = get_next_pos(&dir, pos, &col_map, &row_map);
    if next_pos.is_none() {
      complete = true;
      next_pos = Some(match dir {
        Dir::NORTH => (pos.0, 0),
        Dir::SOUTH => (pos.0, max.1),
        Dir::WEST => (0, pos.1),
        Dir::EAST => (max.0, pos.1),
      })
    }
    match dir {
      Dir::NORTH => (next_pos.unwrap().1..=pos.1).for_each(|y| {path.insert((pos.0, y));}),
      Dir::SOUTH => (pos.1..=next_pos.unwrap().1).for_each(|y| {path.insert((pos.0, y));}),
      Dir::WEST => (next_pos.unwrap().0..=pos.0).for_each(|x| {path.insert((x, pos.1));}),
      Dir::EAST => (pos.0..=next_pos.unwrap().0).for_each(|x| {path.insert((x, pos.1));}),
    }
    pos = next_pos.unwrap();
    dir = match dir {
      Dir::NORTH => Dir::EAST,
      Dir::SOUTH => Dir::WEST,
      Dir::WEST => Dir::NORTH,
      Dir::EAST => Dir::SOUTH,
    };
  }

  println!("{:?}", path);
  path.len() as u32
}

fn get_next_pos(dir: &Dir, pos: (usize, usize), col_map: &HashMap<usize, Vec<usize>>, row_map: &HashMap<usize, Vec<usize>>) -> Option<(usize, usize)> {
  let next_x_y = match dir {
    Dir::NORTH => col_map.get(&pos.0).unwrap().iter().filter(|y| **y < pos.1).last(),
    Dir::SOUTH => col_map.get(&pos.0).unwrap().iter().filter(|y| **y > pos.1).next(),
    Dir::WEST => row_map.get(&pos.1).unwrap().iter().filter(|y| **y < pos.0).last(),
    Dir::EAST => row_map.get(&pos.1).unwrap().iter().filter(|y| **y > pos.0).next(),
  };
  match (dir, next_x_y) {
    (_, None) => None,
    (Dir::NORTH, Some(y)) => Some((pos.0, y+1)),
    (Dir::SOUTH, Some(y)) => Some((pos.0, y-1)),
    (Dir::WEST, Some(x)) => Some((x+1, pos.1)),
    (Dir::EAST, Some(x)) => Some((x-1, pos.1)),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 6;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_06_a() {
    const EXAMPLE_ANSWER: Option<u32> = Some(41);
    const ANSWER: Option<u32> = Some(4433);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_06_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
