extern crate test;

pub fn main(contents: String) -> u64 {
  reverse_engineer(contents)
}

fn reverse_engineer(contents: String) -> u64 {
  let instructions = contents
    .lines()
    .skip(4)
    .next().unwrap()
    .strip_prefix("Program: ").unwrap()
    .split(',')
    .map(|c| c.parse::<u8>().unwrap())
    .collect::<Vec<u8>>();

  // Determine settings
  let pairs = instructions.iter().array_chunks::<2>().collect::<Vec<[&u8; 2]>>();
  let mut division_value = 0u64;
  for pair in pairs.iter() {
    if *pair[0] == 0 {
      division_value = *pair[1] as u64;
      break;
    }
  }

  match brute_force_chunk(&instructions, 0, instructions.len() - 1, division_value) {
    Some(ans) => ans,
    None => 0u64
  }
}

fn brute_force_chunk(instructions: &Vec<u8>, current_total: u64, output_idx: usize, div_value: u64) -> Option<u64> {
  for a_attempt in 0..=(2u64.pow(div_value as u32)) {
    let initial_a = (current_total << div_value) + a_attempt;
    let mut a = initial_a;
    let mut b = 0;
    let mut c = 0;
    let desired_output = instructions[output_idx];
    for instruction in instructions.iter().array_chunks::<2>() {
      match instruction {
        [0, op] => a = a/(2u64.pow(get_combo(*op, a, b, c) as u32)),
        [1, lit] => b = b ^ (*lit as u64),
        [2, op] => b = get_combo(*op, a, b, c) % 8,
        [3, _] => {
          if a == 0 && output_idx > 0 {
            return None;
          }
        },
        [4, _] => b = b ^ c,
        [5, op] => {
          let output_val = get_combo(*op, a, b, c) % 8;
          if output_val as u8 == desired_output {
            if output_idx == 0 {
              return Some(initial_a);
            }
            let final_chunk = brute_force_chunk(instructions, initial_a, output_idx - 1, div_value);
            if final_chunk.is_some() {
              return final_chunk;
            }
          }
          break;
        },
        [6, op] => b = a/(2u64.pow(get_combo(*op, a, b, c) as u32)),
        [7, op] => c = a/(2u64.pow(get_combo(*op, a, b, c) as u32)),
        _ => panic!("This should be impossible"),
      }
    }
  }
  None
}

fn get_combo(operand: u8, a: u64, b: u64, c: u64) -> u64 {
  match operand {
    0..=3 => operand as u64,
    4 => a,
    5 => b,
    6 => c,
    _ => panic!("Not valid"),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 17;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_17_b() {
    const EXAMPLE_ANSWER: Option<u64> = Some(117440);
    const ANSWER: Option<u64> = Some(164278899142333);
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_17_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
