extern crate test;

pub fn main(contents: String) -> u64 {
  find_trailheads(contents)
}

fn find_trailheads(contents: String) -> u64 {
  contents
    .split_ascii_whitespace()
    .map(|s| blink(s.parse::<u64>().unwrap(), 0))
    .sum::<u64>() as u64
}

fn blink(stone_val: u64, total_blinks: u8) -> u64 {
  if total_blinks == 25 {
    1
  } else if stone_val == 0 {
    blink(1, total_blinks+1)
  } else {
    let num_str = stone_val.to_string();
    if num_str.len() % 2 == 0 {
      blink(num_str[0..num_str.len()/2].parse::<u64>().unwrap(), total_blinks + 1) + blink(num_str[num_str.len()/2..num_str.len()].parse::<u64>().unwrap(), total_blinks + 1)
    } else {
      blink(stone_val * 2024, total_blinks + 1)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 11;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_11_a() {
    const EXAMPLE_ANSWER: Option<u64> = Some(55312);
    const ANSWER: Option<u64> = Some(224529);
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_11_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
