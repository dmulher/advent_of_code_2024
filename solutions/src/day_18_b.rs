use std::collections::VecDeque;

extern crate test;

pub fn main(contents: String) -> String {
  do_thing(contents)
}

fn do_thing(contents: String) -> String {
  let size: usize = if contents.lines().count() == 25 { 7 } else { 71 };
  let mut map = vec![vec![false; size]; size];
  for coord in contents.lines() {
    let (x, y) = coord.split_once(',').unwrap();
    map[y.parse::<usize>().unwrap()][x.parse::<usize>().unwrap()] = true;

    if !can_run_maze(&map) {
      return format!("{x},{y}");
    }
  }

  String::from("???")
}

fn can_run_maze(map: &Vec<Vec<bool>>) -> bool {
  let size: usize = map.len();

  let mut next_steps = VecDeque::<((usize, usize), u32)>::new();
  let mut visited = vec![vec![false; size]; size];

  next_steps.push_back(((0, 0), 0));

  while let Some(((y, x), steps)) = next_steps.pop_front() {
    if visited[y][x] {
      continue;
    }
    visited[y][x] = true;

    if x == size - 1 && y == size - 1 {
      return true;
    }

    if y > 0 && !visited[y-1][x] && !map[y-1][x] {
      next_steps.push_back(((y-1, x), steps + 1));
    }
    if y < size - 1 && !visited[y+1][x] && !map[y+1][x] {
      next_steps.push_back(((y+1, x), steps + 1));
    }
    if x > 0 && !visited[y][x-1] && !map[y][x-1] {
      next_steps.push_back(((y, x-1), steps + 1));
    }
    if x < size - 1 && !visited[y][x+1] && !map[y][x+1] {
      next_steps.push_back(((y, x+1), steps + 1));
    }
  }

  false
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 18;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_18_b() {
    let example_answer: Option<String> = Some(String::from("6,1"));
    let answer: Option<String> = Some(String::from("62,6"));
    match utils::run_method::<String>(&main, DAY, PART, (example_answer, answer.clone())) {
      Err(message) => panic!("{}", message),
      Ok(val) if answer.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_18_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
