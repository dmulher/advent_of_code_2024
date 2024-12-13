extern crate test;

pub fn main(contents: String) -> u64 {
  get_min_presses(contents)
}

fn get_min_presses(contents: String) -> u64 {
  contents
    .lines()
    .array_chunks::<4>()
    .map(solve_linear)
    .sum()
}

fn solve_linear(machine: [&str; 4]) -> u64 {
  let [a_s, b_s, p_s, _] = machine;
  let (a_xs, a_ys) = a_s.strip_prefix("Button A: X+").unwrap().split_once(", Y+").unwrap();
  let (b_xs, b_ys) = b_s.strip_prefix("Button B: X+").unwrap().split_once(", Y+").unwrap();
  let (p_xs, p_ys) = p_s.strip_prefix("Prize: X=").unwrap().split_once(", Y=").unwrap();
  let [a_x, a_y, b_x, b_y, p_x, p_y] = [a_xs.parse::<i64>().unwrap(), a_ys.parse::<i64>().unwrap(), b_xs.parse::<i64>().unwrap(), b_ys.parse::<i64>().unwrap(), p_xs.parse::<i64>().unwrap() + 10000000000000, p_ys.parse::<i64>().unwrap() + 10000000000000];

  // Cramer's Rule
  // | a_x | b_x | p_x
  // | a_y | b_y | p_y
  let a_num = p_x * b_y - p_y * b_x;
  let b_num = a_x * p_y - a_y * p_x;
  let a_b_det = a_x * b_y - a_y * b_x;

  if a_num % a_b_det == 0 && b_num % a_b_det == 0 {
    println!("For Machine {:?}", machine);
    println!("answer is {}", a_num / a_b_det + b_num / a_b_det);
    (a_num * 3 / a_b_det + b_num / a_b_det) as u64
  } else {
    0
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 13;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_13_b() {
    const EXAMPLE_ANSWER: Option<u64> = None;
    const ANSWER: Option<u64> = Some(93866170395343);
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_13_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
