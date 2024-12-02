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
      .collect::<Vec<u16>>()
    )
    .map(test_report)
    .sum()
}

fn test_report(report: Vec<u16>) -> u16 {
  let mut last_num: Option<u16> = None;
  let mut ascending: Option<bool> = None;
  for level in report.into_iter() {
    if let Some(last_level) = last_num {
      if level.abs_diff(last_level) < 1 || level.abs_diff(last_level) > 3 {
        return 0;
      }
      if let Some(asc) = ascending {
        if (level > last_level) != asc {
          return 0;
        }
      } else {
        ascending = Some(level > last_level);
      }
    }
    last_num = Some(level);
  }
  return 1;
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 2;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_01_a() {
    const EXAMPLE_ANSWER: Option<u16> = Some(2);
    const ANSWER: Option<u16> = Some(402);
    match utils::run_method::<u16>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_01_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
