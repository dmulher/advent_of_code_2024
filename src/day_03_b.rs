use std::char;

extern crate test;

pub fn main(contents: String) -> u32 {
  add_instructions(contents)
}

fn add_instructions(contents: String) -> u32 {
  let mut ans = 0u32;
  let mut is_do = true;
  let mut is_donting: bool = false;
  let mut context: Option<char> = None;
  let mut first_num: String = "".to_string();
  let mut second_num: String = "".to_string();
  for char in contents
    .chars()
    .into_iter()
  {
    match (is_do, context, char) {
      (false, _, 'd') | (false, Some('d'), 'o') | (false, Some('o'), '(') => context = Some(char),
      (false, Some('('), ')') => {
        is_do = true;
        context = None;
      },
      (true, _, 'm') | (true, Some('m'), 'u') | (true, Some('u'), 'l') | (true, Some('l'), '(') => {
        is_donting = false;
        context = Some(char);
      },
      (true, Some('('), ',') if !is_donting => context = Some(char),
      (true, Some('('), ')') if is_donting => {
        is_do = false;
        context = None;
      },
      (true, Some('('), c) if c.is_digit(10) && !is_donting => first_num.push(c),
      (true, Some(','), c) if c.is_digit(10) => second_num.push(c),
      (true, Some(','), ')') if first_num.len() > 0 && second_num.len() > 0 => {
        ans += first_num.parse::<u32>().unwrap_or(0) * second_num.parse::<u32>().unwrap_or(0);
        context = None;
        first_num = "".to_string();
        second_num = "".to_string();
      },
      (true, _, 'd') | (true, Some('d'), 'o') | (true, Some('o'), 'n') | (true, Some('n'), '\'') | (true, Some('\''), 't') | (true, Some('t'), '(') => {
        is_donting = true;
        context = Some(char);
        first_num = "".to_string();
        second_num = "".to_string();
      },
      _ => {
        is_donting = false;
        context = None;
        first_num = "".to_string();
        second_num = "".to_string();
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

  const DAY: u8 = 3;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_03_b() {
    const EXAMPLE_ANSWER: Option<u32> = Some(48);
    const ANSWER: Option<u32> = Some(79845780);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_03_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
