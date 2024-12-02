extern crate test;

use std::collections::HashMap;

pub fn main(contents: String) -> u32 {
  get_similarity(contents)
}

fn get_similarity(contents: String) -> u32 {
  let mut left: HashMap<u32, u32> = HashMap::new();
  let mut right: HashMap<u32, u32> = HashMap::new();
  for line in contents.lines() {
    let mut split_line = line.split_ascii_whitespace();
    *left.entry(utils::get_int_from_string_slice(split_line.next(), 0u32)).or_insert(0) += 1;
    *right.entry(utils::get_int_from_string_slice(split_line.next(), 0u32)).or_insert(0) += 1;
  }

  left.keys().into_iter().map(|num| {
    *num * left.get(num).unwrap_or(&0u32) * right.get(num).unwrap_or(&0u32)
  }).sum()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 1;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_01_b() {
    const EXAMPLE_ANSWER: Option<u32> = Some(31);
    const ANSWER: Option<u32> = Some(23529853);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_01_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
