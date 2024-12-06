use std::{cmp::Ordering, collections::HashMap};

extern crate test;

pub fn main(contents: String) -> u32 {
  get_instructions(contents)
}

#[derive(Debug)]
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
  let (rules_str, updates_str) = contents.split_once("\n\n").unwrap();

  let rules = rules_str
    .lines()
    .map(|line| line.split_once('|').unwrap())
    .map(|(left, right)| (left.parse::<u16>().unwrap(), right.parse::<u16>().unwrap()))
    .fold(HashMap::<u16, Rule>::new(), |mut acc, (left, right)| {
      acc.entry(left).or_insert(Rule::new()).futures.push(right);
      acc.entry(right).or_insert(Rule::new()).requirements.push(left);
      acc
    });

  updates_str
    .lines()
    .map(|update| {
      let commands = update.split(",").map(|command| command.parse::<u16>().unwrap()).enumerate().collect::<Vec<(usize, u16)>>();
      if commands.is_sorted_by(|a, b| compare_fn(a, b, &rules) == Ordering::Less) {
        commands[commands.len()/2].1 as u32
      } else {
        0
      }
    })
    .sum()
}

fn compare_fn(a: &(usize, u16), b: &(usize, u16), rules: &HashMap<u16, Rule>) -> Ordering {
  if let Some(rule) = rules.get(&a.1) {
    if rule.requirements.contains(&b.1) {
      Ordering::Greater
    } else if rule.futures.contains(&b.1) {
      Ordering::Less
    } else {
      a.0.cmp(&b.0)
    }
  } else {
    Ordering::Equal
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
