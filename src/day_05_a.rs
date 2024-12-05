use std::collections::{HashMap, HashSet};

extern crate test;

pub fn main(contents: String) -> u32 {
  get_instructions(contents)
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Rule {
  requirements: Vec<u16>,
  futures: Vec<u16>,
}

impl Rule {
  pub fn new() -> Self {
    Rule {requirements: vec![], futures: vec![]}
  }
}

fn get_instructions(contents: String) -> u32 {
  let mut ans = 0u32;

  let mut rules: HashMap<u16, Rule> = HashMap::new();
  let mut acquiring_rules = true;

  for line in contents.lines() {
    if line == "" {
      acquiring_rules = false;
      println!("{:?}", rules);
    } else if acquiring_rules {
      if let Some((left, right)) = line.split_once('|') {
        let left_num = left.parse::<u16>().unwrap();
        let right_num = right.parse::<u16>().unwrap();
        rules.entry(left_num).or_insert(Rule::new()).futures.push(right_num);
        rules.entry(right_num).or_insert(Rule::new()).requirements.push(left_num);
      }
    } else {
      ans += check_update(line, &rules);
    }
  }

  ans
}

fn check_update(line: &str, rules: &HashMap<u16, Rule>) -> u32 {
  println!("Checking {}", line);
  let update = line.split(',').map(|u| u.parse::<u16>().unwrap()).collect::<Vec<u16>>();
  let mut requirements = HashSet::<u16>::new();
  let mut specials = HashSet::<u16>::new();
  for u in update.iter().rev() {
    if let Some(rule) = rules.get(u) {
      specials.insert(*u);
      requirements.extend(rule.requirements.iter());
      requirements.remove(u);
    }
  }
  if requirements.iter().any(|req| specials.contains(req)) {
    0
  } else {
    update[update.len() / 2] as u32
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 5;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_05_a() {
    const EXAMPLE_ANSWER: Option<u32> = Some(143);
    const ANSWER: Option<u32> = Some(4957);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_05_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
