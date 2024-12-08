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

  let tester = start.unwrap() == (4usize, 6usize);
  if tester {
    println!("{:?}", path);
  }
  check_path_brute(&row_map, &col_map, start.unwrap(), path)
}

fn check_path_brute(row_map: &HashMap<usize, Vec<usize>>, col_map: &HashMap<usize, Vec<usize>>, start: (usize, usize), passed_points: HashSet<(usize, usize)>) -> u32 {
  let max = (col_map.len() - 1, row_map.len() - 1);

  let mut ans: u32 = 0;

  let tester = start == (4usize, 6usize);
  if tester {
    println!("{:?}", passed_points);
  }
  for point in passed_points {
    let debug = (3usize, 6usize) == point;
    let mut pos = start;
    let mut dir = Dir::NORTH;
    let mut complete = false;
    let mut path_points: HashSet<(usize, usize)> = HashSet::new();
    path_points.insert(pos);

    let mut new_row_map = HashMap::new();
    row_map.iter().for_each(|(i, x)| {new_row_map.insert(*i, x.clone());});
    let mut new_col_map = HashMap::new();
    col_map.iter().for_each(|(i, x)| {new_col_map.insert(*i, x.clone());});
    new_row_map.entry(point.1).or_insert(vec![]).push(point.0);
    new_col_map.entry(point.0).or_insert(vec![]).push(point.1);
    new_row_map.values_mut().for_each(|row| row.sort());
    new_col_map.values_mut().for_each(|col| col.sort());

    if debug {
      println!("new_row_map = {:?}", new_row_map);
      println!("new_col_map = {:?}", new_col_map);
    }


    while !complete {
      let mut next_pos = get_next_pos(&dir, pos, &new_col_map, &new_row_map);
      if next_pos.is_none() {
        complete = true;
        next_pos = Some(match dir {
          Dir::NORTH => (pos.0, 0),
          Dir::SOUTH => (pos.0, max.1),
          Dir::WEST => (0, pos.1),
          Dir::EAST => (max.0, pos.1),
        })
      }
      if pos != next_pos.unwrap() && !path_points.insert(next_pos.unwrap()) {
        ans += 1;
        complete = true;
      } else {
        pos = next_pos.unwrap();
        dir = match dir {
          Dir::NORTH => Dir::EAST,
          Dir::SOUTH => Dir::WEST,
          Dir::WEST => Dir::NORTH,
          Dir::EAST => Dir::SOUTH,
        };
      }
    }
  }
  
  ans
}

fn get_next_pos(dir: &Dir, pos: (usize, usize), col_map: &HashMap<usize, Vec<usize>>, row_map: &HashMap<usize, Vec<usize>>) -> Option<(usize, usize)> {
  if !col_map.contains_key(&pos.0) {
    println!("Trying to access {:?} with {} from {:?}", col_map, pos.0, pos);
  }
  let next_x_y = match dir {
    Dir::NORTH => {
      match col_map.get(&pos.0) {
        Some(col) => col.iter().filter(|y| **y < pos.1).last(),
        None => None,
      }
    },
    Dir::SOUTH => {
      match col_map.get(&pos.0) {
        Some(col) => col.iter().filter(|y| **y > pos.1).next(),
        None => None,
      }
    },
    Dir::WEST => {
      match row_map.get(&pos.1) {
        Some(row) => row.iter().filter(|x| **x < pos.0).last(),
        None => None,
      }
    },
    Dir::EAST => {
      match row_map.get(&pos.1) {
        Some(row) => row.iter().filter(|x| **x > pos.0).next(),
        None => None,
      }
    },
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
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_06_b() {
    const EXAMPLE_ANSWER: Option<u32> = Some(6);
    const ANSWER: Option<u32> = Some(1516);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_06_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
