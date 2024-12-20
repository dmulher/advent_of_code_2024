use std::collections::HashMap;

extern crate test;

pub fn main(contents: String) -> u64 {
  make_towels(contents)
}

fn make_towels(contents: String) -> u64 {
  let mut lines = contents.lines();
  let mut threads: HashMap<char, Vec<&str>> = HashMap::new();
  lines
    .next()
    .unwrap()
    .split(", ")
    .for_each(|thread| threads.entry(thread.chars().next().unwrap()).or_insert(vec![]).push(thread));

  let mut seen_patterns: HashMap<String, u64> = HashMap::new();

  lines
    .skip(1)
    .map(|towel| can_make_towel(towel, &threads, &mut seen_patterns))
    .sum::<u64>() as u64
}

fn can_make_towel(pattern: &str, threads: &HashMap<char, Vec<&str>>, seen_patterns: &mut HashMap<String, u64>) -> u64 {
  if let Some(ans) = seen_patterns.get(&String::from(pattern)) {
    *ans
  } else {
    let first_char = pattern.chars().next().unwrap();
    if let Some(char_threads) = threads.get(&first_char) {
      let num_patterns = char_threads.iter().map(|thread| {
        if *thread == pattern {
          1
        } else if thread.len() > pattern.len() {
          0
        } else {
          let (first, last) = pattern.split_at(thread.len());
          if first == *thread {
            can_make_towel(last, threads, seen_patterns)
          } else {
            0
          }
        }
      })
      .sum();
      seen_patterns.insert(String::from(pattern), num_patterns);
      num_patterns
    } else {
      seen_patterns.insert(String::from(pattern), 0);
      0
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 19;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_19_b() {
    const EXAMPLE_ANSWER: Option<u64> = Some(16);
    const ANSWER: Option<u64> = Some(577474410989846);
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_19_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
