extern crate test;

pub fn main(contents: String) -> u64 {
  find_trailheads(contents)
}

fn find_trailheads(contents: String) -> u64 {
  // TODO: Could do a cache like in part b but the cache would be a Vec<(u8, u8)>?
  let mut trail_map: Vec<Vec<u8>> = contents
    .lines()
    .map(|line| line
      .chars()
      .map(|c| c.to_digit(10).unwrap() as u8)
      .collect::<Vec<u8>>()
    )
    .collect::<Vec<Vec<u8>>>();

  let mut ans = 0u64;
  for i in 0..trail_map.len() {
    for j in 0..trail_map[i].len() {
      if trail_map[i][j] == 0 {
        let mut visited = trail_map.iter().map(|line| line.iter().map(|_| false).collect::<Vec<bool>>()).collect::<Vec<Vec<bool>>>();
        let val = assess_value(&mut trail_map, &mut visited, i, j) as u64;
        ans += val;
      }
    }
  }

  ans
}

fn assess_value(trail_map: &mut Vec<Vec<u8>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize) -> u8 {
  let height = trail_map[i][j];
  let previously_visited = visited[i][j];
  visited[i][j] = true;
  if previously_visited {
    0
  } else if height == 9 {
    1
  } else {
    let mut val = 0;
    if i > 0 && trail_map[i-1][j] == height + 1 {
      let up = assess_value(trail_map, visited, i-1, j);
      val += up;
    }
    if i < trail_map.len() - 1 && trail_map[i+1][j] == height + 1 {
      let down = assess_value(trail_map, visited, i+1, j);
      val += down;
    }
    if j > 0 && trail_map[i][j-1] == height + 1 {
      let left = assess_value(trail_map, visited, i, j-1);
      val += left;
    }
    if j < trail_map[i].len() - 1 && trail_map[i][j+1] == height + 1 {
      let right = assess_value(trail_map, visited, i, j+1);
      val += right;
    }
    val
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 10;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_10_a() {
    const EXAMPLE_ANSWER: Option<u64> = Some(36);
    const ANSWER: Option<u64> = Some(820);
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_10_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
