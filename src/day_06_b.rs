extern crate test;

pub fn main(contents: String) -> u64 {
  get_instructions(contents)
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Operation {
  ADD,
  MULT,
  MERGE,
}

fn get_instructions(contents: String) -> u64 {
  contents
    .lines()
    .map(|line| {
      let mut parts = line.split_ascii_whitespace();
      let answer_part = parts.next().unwrap();
      let answer = answer_part[0..answer_part.len()-1].parse::<u64>().unwrap();
      let eq = parts.map(|p| p.parse::<u64>().unwrap()).collect::<Vec<u64>>();
      // let highest = eq.clone().into_iter().reduce(|acc, e| if e == 1 {merge_numbers(acc, e)} else {merge_numbers(acc, e)}).unwrap();
      // let lowest = eq.clone().into_iter().reduce(|acc, e| if e == 1 {acc * e} else {acc + e}).unwrap();
      // if answer > highest || answer < lowest {
      //   return 0;
      // } else if answer == highest || answer == lowest {
      //   return answer;
      // }
      let mut operations: Vec<Operation> = vec![Operation::ADD; eq.len() - 1];
      for _ in 0..3usize.pow(eq.len() as u32) {
        let mut check_answer = 0u64;
        eq.iter().enumerate().for_each(|(idx, e)| {
          if idx == 0 {
            check_answer = *e;
          } else {
            match operations[idx - 1] {
              Operation::ADD => check_answer += *e,
              Operation::MULT => check_answer *= *e,
              Operation::MERGE => check_answer = merge_numbers(check_answer, *e),
            }
          }
        });
        if check_answer == answer {
          return answer;
        }
        operations = get_next_operations(operations);
      }

      return 0;
    })
    .sum()
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
      (true, Operation::MULT) => {
        carrying = true;
        Operation::MERGE
      },
      (true, Operation::MERGE) => Operation::ADD,
      (false, op) => op,
    });
  });
  new_operations.reverse();
  new_operations
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

  const DAY: u8 = 6;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_06_b() {
    const EXAMPLE_ANSWER: Option<u64> = Some(11387);
    const ANSWER: Option<u64> = Some(945341732469724);
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_06_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
