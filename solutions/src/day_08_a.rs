use std::collections::{HashMap, HashSet};

extern crate test;

pub fn main(contents: String) -> u16 {
  get_instructions(contents)
}

fn get_instructions(contents: String) -> u16 {
  let height = contents.lines().count();
  let width = contents.lines().next().unwrap().chars().count();
  let mut map: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();
  contents
    .lines()
    .enumerate()
    .for_each(|(i, line)| {
      line
        .chars()
        .map(|c| c as u8)
        .enumerate()
        .filter(|(_, c)| *c != b'.')
        .for_each(|(j, c)| {
          map
            .entry(c)
            .or_insert(Vec::<(usize, usize)>::new())
            .push((i, j));
        })
    });
  let total_signals = map
    .into_iter()
    .map(|(_, positions)| {
      let mut signals: HashSet<(i32, i32)> = HashSet::new();
      for pos_a in positions.iter().map(|(x, y)| (*x as i32, *y as i32)) {
        for pos_b in positions.iter().map(|(x, y)| (*x as i32, *y as i32)) {
          if pos_a == pos_b {
            continue;
          }
          let distance = (pos_b.0 - pos_a.0, pos_b.1 - pos_a.1);
          let beyond_a = (pos_a.0 - distance.0, pos_a.1 - distance.1);
          if beyond_a.0 >= 0 && beyond_a.0 < height as i32 && beyond_a.1 >= 0 && beyond_a.1 < width as i32 {
            signals.insert((beyond_a.0, beyond_a.1));
          }
          let beyond_b = (pos_b.0 + distance.0, pos_b.1 + distance.1);
          if beyond_b.0 >= 0 && beyond_b.0 < height as i32 && beyond_b.1 >= 0 && beyond_b.1 < width as i32 {
            signals.insert((beyond_b.0, beyond_b.1));
          }
        }
      }
      signals
    })
    .flatten()
    .collect::<HashSet<(i32, i32)>>();
  println!("{:?}", total_signals);
  total_signals.len() as u16
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 7;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_07_a() {
    const EXAMPLE_ANSWER: Option<u16> = Some(14);
    const ANSWER: Option<u16> = Some(254);
    match utils::run_method::<u16>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_07_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
