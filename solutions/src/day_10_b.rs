extern crate test;

pub fn main(contents: String) -> u64 {
  find_trailheads(contents)
}

fn find_trailheads(contents: String) -> u64 {
  let mut trail_map: Vec<Vec<(u8, Option<u8>)>> = contents
    .lines()
    .map(|line| line
      .chars()
      .map(|c| (c.to_digit(10).unwrap() as u8, None))
      .collect::<Vec<(u8, Option<u8>)>>()
    )
    .collect::<Vec<Vec<(u8, Option<u8>)>>>();

  let mut ans = 0u64;
  for i in 0..trail_map.len() {
    for j in 0..trail_map[i].len() {
      if trail_map[i][j].0 == 0 {
        ans += assess_value(&mut trail_map, i, j) as u64;
      }
    }
  }

  ans
}

fn assess_value(trail_map: &mut Vec<Vec<(u8, Option<u8>)>>, i: usize, j: usize) -> u8 {
  let (height, cache) = trail_map[i][j];
  if height == 9 {
    1
  } else if let Some(val) = cache {
    return val;
  } else {
    let mut val = 0;
    if i > 0 && trail_map[i-1][j].0 == height + 1 {
      val += assess_value(trail_map, i-1, j);
    }
    if i < trail_map.len() - 1 && trail_map[i+1][j].0 == height + 1 {
      val += assess_value(trail_map, i+1, j);
    }
    if j > 0 && trail_map[i][j-1].0 == height + 1 {
      val += assess_value(trail_map, i, j-1);
    }
    if j < trail_map[i].len() - 1 && trail_map[i][j+1].0 == height + 1 {
      val += assess_value(trail_map, i, j+1);
    }
    trail_map[i][j] = (height, Some(val));
    val
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 10;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_10_b() {
    const EXAMPLE_ANSWER: Option<u64> = Some(81);
    const ANSWER: Option<u64> = Some(1786);
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_10_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
