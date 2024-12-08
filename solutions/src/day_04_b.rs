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
        'M' => b'M',
        'A' => b'A',
        'S' => b'S',
        _ => 0,
      })
      .collect::<Vec<u8>>()
    )
    .collect::<Vec<Vec<u8>>>();

  for i in 1..(map.len()-1) {
    for j in 1..(map[i].len()-1) {
      if map[i][j] == b'A' {
        let tl_br = (map[i-1][j-1] == b'M' && map[i+1][j+1] == b'S') || (map[i-1][j-1] == b'S' && map[i+1][j+1] == b'M');
        let tr_bl = (map[i-1][j+1] == b'M' && map[i+1][j-1] == b'S') || (map[i-1][j+1] == b'S' && map[i+1][j-1] == b'M');
        if tl_br && tr_bl {
          ans += 1;
        }
      }
    }
  }

  ans
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 4;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_04_b() {
    const EXAMPLE_ANSWER: Option<u32> = Some(9);
    const ANSWER: Option<u32> = Some(1815);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_04_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
