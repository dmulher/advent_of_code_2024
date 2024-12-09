use std::cmp::Ordering;

extern crate test;

pub fn main(contents: String) -> u64 {
  get_instructions(contents)
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Operation {
  ADD,
  MULT,
  MERGE,
}

#[derive(Debug)]
struct Instruction {
  answer: u64,
  numbers: Vec<u64>,
}

fn get_instructions(contents: String) -> u64 {
  contents
    .lines()
    .map(|line| {
      let mut parts = line.split_ascii_whitespace();
      let answer_part = parts.next().unwrap();
      let answer = answer_part[0..answer_part.len()-1].parse::<u64>().unwrap();
      let numbers: Vec<u64> = parts.map(|p| p.parse::<u64>().unwrap()).rev().collect::<Vec<u64>>();
      Instruction { answer, numbers }
    })
    .filter(|instruction| is_instruction_correct(instruction.numbers.clone(), instruction.answer, 0, Operation::ADD) == Ordering::Equal)
    .map(|instruction| instruction.answer)
    .sum()
}

fn is_instruction_correct(mut numbers: Vec<u64>, answer: u64, total: u64, operation: Operation) -> Ordering {
  let val = apply_operator(total, numbers.pop().unwrap(), operation);

  if numbers.len() == 0 {
    return val.cmp(&answer);
  } else if val > answer {
    return Ordering::Greater;
  }

  check_next_step(numbers.clone(), answer, val, Operation::MERGE)
}

fn check_next_step(numbers: Vec<u64>, answer: u64, total: u64, operation: Operation) -> Ordering {
  let result = is_instruction_correct(numbers.clone(), answer, total, operation);
  match (result, operation) {
    (Ordering::Less, Operation::MULT) if numbers.contains(&1) || total == 1 => {
      check_next_step(numbers.clone(), answer, total, Operation::ADD)
    },
    (Ordering::Greater, Operation::MERGE) => {
      match check_next_step(numbers.clone(), answer, total, Operation::MULT) {
        Ordering::Less => Ordering::Greater,
        ord => ord,
      }
    },
    (Ordering::Greater, Operation::MULT) => {
      match check_next_step(numbers.clone(), answer, total, Operation::ADD) {
        Ordering::Less => Ordering::Greater,
        ord => ord,
      }
    },
    (ord, _) => {
      ord
    }
  }
}

fn apply_operator(total: u64, new_num: u64, operation: Operation) -> u64 {
  match operation {
    Operation::ADD => total + new_num,
    Operation::MULT => total * new_num,
    Operation::MERGE => merge_numbers(total, new_num),
  }
}

fn merge_numbers(a: u64, b: u64) -> u64 {
  let mut s = a.to_string();
  s.push_str(&b.to_string());
  s.parse::<u64>().unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 7;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_07_b() {
    const EXAMPLE_ANSWER: Option<u64> = Some(11387);
    const ANSWER: Option<u64> = Some(945341732469724);
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_07_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
