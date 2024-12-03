use std::char;

extern crate test;

pub fn main(contents: String) -> u32 {
  add_instructions(contents)
}

fn add_instructions(contents: String) -> u32 {
  let mut ans = 0u32;
  let mut context: Option<char> = None;
  let mut first_num: String = "".to_string();
  let mut second_num: String = "".to_string();
  for char in contents
    .chars()
    .into_iter()
  {
    match (context, char) {
      (None, 'm') | (Some('m'), 'u') | (Some('u'), 'l') | (Some('l'), '(') | (Some('('), ',') => context = Some(char),
      (Some('('), c) if c.is_digit(10) => first_num.push(c),
      (Some(','), c) if c.is_digit(10) => second_num.push(c),
      (Some(','), ')') if first_num.len() > 0 && second_num.len() > 0 => {
        ans += first_num.parse::<u32>().unwrap_or(0) * second_num.parse::<u32>().unwrap_or(0);
        context = None;
        first_num = "".to_string();
        second_num = "".to_string()
      },
      _ => {
        context = None;
        first_num = "".to_string();
        second_num = "".to_string()
      }
    }
  }
  return ans;
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 3;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_03_a() {
    const EXAMPLE_ANSWER: Option<u32> = Some(161);
    const ANSWER: Option<u32> = Some(157621318);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_03_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
