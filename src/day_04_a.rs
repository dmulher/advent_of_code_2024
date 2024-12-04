extern crate test;

pub fn main(contents: String) -> u32 {
  add_instructions(contents)
}

fn add_instructions(contents: String) -> u32 {
  let mut ans = 0u32;
  let map = contents
    .lines()
    .map(|line| line
      .chars()
      .map(|c| match c {
        'X' => b'X',
        'M' => b'M',
        'A' => b'A',
        'S' => b'S',
        _ => b'N',
      })
      .collect::<Vec<u8>>()
    )
    .collect::<Vec<Vec<u8>>>();

  for i in 0..map.len() {
    for j in 0..map[i].len() {
      if map[i][j] == b'X' {
        let all_directions = get_all_directions((i, j), &map);
        ans += all_directions
          .into_iter()
          .map(|dir| {
            let mut new_idx = (i, j);
            for c in [b'M', b'A', b'S'] {
              new_idx = ((new_idx.0 as i16 + dir.0 as i16) as usize, (new_idx.1 as i16 + dir.1 as i16) as usize);
              if map[new_idx.0][new_idx.1] != c {
                return 0u32;
              }
            }
            return 1u32;
          })
          .sum::<u32>();
      }
    }
  }

  ans
}

fn get_all_directions(x_idx: (usize, usize), map: &Vec<Vec<u8>>) -> Vec<(i8, i8)> {
  let mut all_directions: Vec<(i8, i8)> = vec!();
  let can_go_up = x_idx.0 > 2;
  let can_go_down = x_idx.0 < map.len() - 3;
  let can_go_left = x_idx.1 > 2;
  let can_go_right = x_idx.1 < map[x_idx.0].len() - 3;
  if can_go_up && can_go_left {
    all_directions.push((-1, -1));
  }
  if can_go_up {
    all_directions.push((-1, 0));
  }
  if can_go_up && can_go_right {
    all_directions.push((-1, 1));
  }
  if can_go_left {
    all_directions.push((0, -1));
  }
  all_directions.push((0, 0));
  if can_go_right {
    all_directions.push((0, 1));
  }
  if can_go_down && can_go_left {
    all_directions.push((1, -1));
  }
  if can_go_down {
    all_directions.push((1, 0));
  }
  if can_go_down && can_go_right {
    all_directions.push((1, 1));
  }
  all_directions
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 4;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_04_a() {
    const EXAMPLE_ANSWER: Option<u32> = Some(18);
    const ANSWER: Option<u32> = Some(2344);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_04_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
