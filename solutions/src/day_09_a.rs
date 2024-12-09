extern crate test;

pub fn main(contents: String) -> u64 {
  get_checksum(contents)
}

fn get_checksum(contents: String) -> u64 {
  let file_system = contents.chars().enumerate().map(|(idx, c)| ((idx / 2) as u64, c.to_digit(10).unwrap() as u64)).collect::<Vec<(u64, u64)>>();
  let mut left_pointer = 0;
  let mut right_pointer = file_system.len() + 1;
  let mut idx = 0u64;
  let mut right_storage = (0u64, 0u64);
  let mut ans = 0u64;

  while left_pointer <= right_pointer {
    let next_block = file_system[left_pointer];
    let mut block_val = file_system[left_pointer].1 as u64;
    left_pointer += 1;
    if block_val == 0 {
      continue;
    }

    if (left_pointer - 1) % 2 == 0 {
      // Actual space
      let smaller_triangle = if idx > 1 {(idx - 1) * idx / 2} else {0};
      let total_val = next_block.0 as u64 * ((idx + block_val - 1) * (idx + block_val) / 2 - smaller_triangle);
      ans += total_val;
      idx += block_val;
    } else {
      // free space
      while block_val > 0 {
        let is_at_end = left_pointer == right_pointer;
        let (id, mut remaining_space) = right_storage;
        if remaining_space > 0 {
          let space_to_cover = if is_at_end {remaining_space} else {(remaining_space).min(block_val as u64)};
          let smaller_triangle = if idx > 1 {(idx - 1) * idx / 2} else {0};
          let total_val = id as u64 * ((idx - 1 + space_to_cover as u64) * (idx + space_to_cover as u64) / 2 - smaller_triangle);
          ans += total_val;
          if is_at_end {
            return ans;
          }
          idx += space_to_cover as u64;
          remaining_space -= space_to_cover;
          block_val -= space_to_cover as u64;
          right_storage = (id, remaining_space);
        } else {
          right_pointer -= 2;
          if right_pointer < left_pointer {
            return ans;
          }
          right_storage = (file_system[right_pointer].0, file_system[right_pointer].1 as u64);
        }
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

  const DAY: u8 = 9;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_09_a() {
    const EXAMPLE_ANSWER: Option<u64> = Some(1928);
    const ANSWER: Option<u64> = Some(6259790630969);
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_09_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
