extern crate test;

pub fn main(contents: String) -> u64 {
  get_checksum(contents)
}

#[derive(Debug)]
enum MemoryBlock {
  File(u64, u64),
  Free(u64),
}

fn get_checksum(contents: String) -> u64 {
  let mut ans = 0u64;
  let mut file_system = contents
    .chars()
    .enumerate()
    .map(|(idx, c)| {
      let size = c.to_digit(10).unwrap() as u64;
      if idx % 2 == 0 {
        MemoryBlock::File(idx as u64 / 2, size)
      } else {
        MemoryBlock::Free(size)
      }
    })
    .collect::<Vec<MemoryBlock>>();

  let mut file_idx = 0u64;
  let mut idx = 0;
  while idx < file_system.len() {
    if let MemoryBlock::File(id_num, length) = file_system[idx] {
      let larger_triangle = (file_idx + length - 1) * (file_idx + length) / 2;
      let smaller_triangle = if file_idx == 0 {0} else {(file_idx - 1) * file_idx / 2};
      ans += id_num * (larger_triangle - smaller_triangle);
      file_idx += length;
    }
    else if let MemoryBlock::Free(space) = file_system[idx] {
      let mut remaining_space = space;
      for back_idx in (idx+1..file_system.len()).rev() {
        if let MemoryBlock::File(id_num, length) = file_system[back_idx] {
          if length <= remaining_space {
            let larger_triangle = (file_idx + length - 1) * (file_idx + length) / 2;
            let smaller_triangle = if file_idx == 0 {0} else {(file_idx - 1) * file_idx / 2};
            ans += id_num * (larger_triangle - smaller_triangle);
            remaining_space -= length;
            file_idx += length;
            file_system[back_idx] = MemoryBlock::Free(length);
            if remaining_space == 0 {
              break;
            }
          }
        }
      }
      file_idx += remaining_space;
    }
    idx += 1;
  }

  ans
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 9;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_09_b() {
    const EXAMPLE_ANSWER: Option<u64> = Some(2858);
    const ANSWER: Option<u64> = Some(6289564433984);
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_09_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
