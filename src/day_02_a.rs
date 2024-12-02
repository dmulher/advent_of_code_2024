extern crate test;

pub fn main(contents: String) -> u16 {
  find_safe_reports(contents)
}

fn find_safe_reports(contents: String) -> u16 {
  contents
    .lines()
    .map(|line| line
      .split_ascii_whitespace()
      .map(|level| utils::get_int_from_string_slice(Some(level), 0u16))
    )
    .map(test_report)
    .filter(|result| *result)
    .count() as u16
}

fn test_report(report: impl Iterator<Item = u16>) -> bool {
  let mut last_num: Option<u16> = None;
  let mut ascending: Option<bool> = None;
  for level in report {
    if !check_order(level, last_num, ascending) || !check_distance(level, last_num) {
      return false;
    }
    if ascending.is_none() {
      if let Some(last_level) = last_num {
        ascending = Some(level > last_level);
      }
    }
    last_num = Some(level);
  }
  return true;
}

fn check_order(level: u16, last_level: Option<u16>, ascending: Option<bool>) -> bool {
  last_level.is_none() || ascending.is_none() || ascending.unwrap() == (level > last_level.unwrap())
}

fn check_distance(level: u16, last_level: Option<u16>) -> bool {
  last_level.is_none() || (level.abs_diff(last_level.unwrap()) >= 1 && level.abs_diff(last_level.unwrap()) <= 3)
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 2;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_02_a() {
    const EXAMPLE_ANSWER: Option<u16> = Some(2);
    const ANSWER: Option<u16> = Some(402);
    match utils::run_method::<u16>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_02_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
