extern crate test;

pub fn main(contents: String) -> u64 {
  get_instructions(contents)
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Operation {
  ADD,
  MULT,
}

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
      let numbers: Vec<u64> = parts.map(|p| p.parse::<u64>().unwrap()).collect::<Vec<u64>>();
      Instruction { answer, numbers }
    })
    .filter(is_instruction_correct)
    .map(|instruction| instruction.answer)
    .sum()
}

fn is_instruction_correct(instruction: &Instruction) -> bool {
  let mut operations: Vec<Operation> = vec![Operation::ADD; instruction.numbers.len() - 1];
  for _ in 0..2usize.pow(instruction.numbers.len() as u32 - 1) {
    let mut check_answer = 0u64;
    instruction.numbers.iter().enumerate().for_each(|(idx, e)| {
      if idx == 0 {
        check_answer = *e;
      } else {
        match operations[idx - 1] {
          Operation::ADD => check_answer += *e,
          Operation::MULT => check_answer *= *e,
        }
      }
    });
    if check_answer == instruction.answer {
      return true;
    }
    operations = get_next_operations(operations);
  }

  return false;
}

fn get_next_operations(operations: Vec<Operation>) -> Vec<Operation> {
  let mut new_operations: Vec<Operation> = vec![];
  let mut carrying = true;
  operations.into_iter().rev().for_each(|op| {
    new_operations.push(match (carrying, op) {
      (true, Operation::ADD) => {
        carrying = false;
        Operation::MULT
      }
      (true, Operation::MULT) => Operation::ADD,
      (false, op) => op,
    });
  });
  new_operations.reverse();
  new_operations
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 6;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_06_a() {
    const EXAMPLE_ANSWER: Option<u64> = Some(3749);
    const ANSWER: Option<u64> = Some(1611660863222);
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_06_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
