use std::collections::VecDeque;

extern crate test;

pub fn main(contents: String) -> u32 {
  do_thing(contents)
}

fn do_thing(contents: String) -> u32 {
  let size: usize = if contents.lines().count() == 25 { 7 } else { 71 };
  let num_bytes: u32 = if size == 7 { 12 } else { 1024 };
  println!("Size is {size}");
  let mut map = vec![vec![false; size]; size];
  let mut lines = contents.lines();
  for _ in 0..num_bytes {
    let (x, y) = lines.next().unwrap().split_once(',').unwrap();
    map[y.parse::<usize>().unwrap()][x.parse::<usize>().unwrap()] = true;
  }

  for line in map.iter() {
    println!("{}", line.iter().map(|x| if *x {'#'} else {'.'}).collect::<String>());
  }

  let mut next_steps = VecDeque::<((usize, usize), u32)>::new();
  let mut visited = vec![vec![false; size]; size];

  next_steps.push_back(((0, 0), 0));

  while let Some(((y, x), steps)) = next_steps.pop_front() {
    if visited[y][x] {
      continue;
    }
    visited[y][x] = true;

    if x == size - 1 && y == size - 1 {
      return steps;
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

  0u32
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 18;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_18_a() {
    const EXAMPLE_ANSWER: Option<u32> = Some(22);
    const ANSWER: Option<u32> = Some(506);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_18_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
