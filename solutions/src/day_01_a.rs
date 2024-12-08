extern crate test;

pub fn main(contents: String) -> u32 {
  get_distance(contents)
}

fn get_distance(contents: String) -> u32 {
  let mut left = vec!();
  let mut right = vec!();
  for line in contents.lines() {
    let mut split_line = line.split_ascii_whitespace();
    left.push(utils::get_int_from_string_slice(split_line.next(), 0u32));
    right.push(utils::get_int_from_string_slice(split_line.next(), 0u32));
  }

  left.sort();
  right.sort();

  left
    .into_iter()
    .zip(right.into_iter())
    .map(|(a, b)| b.max(a) - b.min(a))
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 1;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_01_a() {
    const EXAMPLE_ANSWER: Option<u32> = Some(11);
    const ANSWER: Option<u32> = Some(1388114);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
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
