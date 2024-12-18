extern crate test;

pub fn main(contents: String) -> String {
  do_thing(contents)
}

fn do_thing(contents: String) -> String {
  let mut lines = contents.lines();
  let mut a = lines.next().unwrap().strip_prefix("Register A: ").unwrap().parse::<u32>().unwrap();
  let mut b = lines.next().unwrap().strip_prefix("Register B: ").unwrap().parse::<u32>().unwrap();
  let mut c = lines.next().unwrap().strip_prefix("Register C: ").unwrap().parse::<u32>().unwrap();
  let instructions = lines
    .skip(1)
    .next().unwrap()
    .strip_prefix("Program: ").unwrap()
    .split(',')
    .map(|c| c.parse::<u8>().unwrap())
    .collect::<Vec<u8>>();
  let mut pointer = 0;
  let mut outputs: Vec<u8> = vec![];
  while pointer < instructions.len() {
    let mut jumped = false;
    println!("A = {a}... B = {b}... C = {c}... Next 2 = {}, {}", instructions[pointer], instructions[pointer + 1]);
    match instructions[pointer] {
      0 => a = a/(2u32.pow(get_combo(*instructions.get(pointer + 1).unwrap_or(&1), a, b, c))),
      1 => b = b ^ (*instructions.get(pointer + 1).unwrap_or(&1) as u32),
      2 => b = get_combo(*instructions.get(pointer + 1).unwrap_or(&1), a, b, c) % 8,
      3 => if a != 0 {
        jumped = true;
        pointer = *instructions.get(pointer + 1).unwrap_or(&(instructions.len() as u8)) as usize;
      },
      4 => b = b ^ c,
      5 => outputs.push((get_combo(*instructions.get(pointer + 1).unwrap_or(&1), a, b, c) % 8) as u8),
      6 => b = a/(2u32.pow(get_combo(*instructions.get(pointer + 1).unwrap_or(&1), a, b, c))),
      7 => c = a/(2u32.pow(get_combo(*instructions.get(pointer + 1).unwrap_or(&1), a, b, c))),
      _ => panic!("This should be impossible"),
    }
    if !jumped {
      pointer += 2;
    }
  }

  println!("outputs = {:?}", outputs);
  outputs.into_iter().map(|d| d.to_string()).collect::<Vec<String>>().join(",")
}

fn get_combo(operand: u8, a: u32, b: u32, c: u32) -> u32 {
  match operand {
    0..=3 => operand as u32,
    4 => a,
    5 => b,
    6 => c,
    _ => panic!("Not valid"),
  }
}

fn split_num(num: u32) -> Vec<char> {
  if num < 10 {
    vec![char::from_digit(num, 10).unwrap()]
  } else {
    num.to_string().chars().collect::<Vec<char>>()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 17;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_17_a() {
    let EXAMPLE_ANSWER: Option<String> = Some(String::from("4,6,3,5,6,3,5,2,1,0"));
    let ANSWER: Option<String> = Some(String::from("7,5,4,3,4,5,3,4,6"));
    match utils::run_method::<String>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER.clone())) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_17_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
