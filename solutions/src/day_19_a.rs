use std::collections::HashMap;

extern crate test;

pub fn main(contents: String) -> u16 {
  make_towels(contents)
}

fn make_towels(contents: String) -> u16 {
  let mut lines = contents.lines();
  let mut threads: HashMap<char, Vec<&str>> = HashMap::new();
  lines
    .next()
    .unwrap()
    .split(", ")
    .for_each(|thread| threads.entry(thread.chars().next().unwrap()).or_insert(vec![]).push(thread));

  let mut seen_patterns: HashMap<String, bool> = HashMap::new();

  lines
    .skip(1)
    .filter(|towel| can_make_towel(towel, &threads, &mut seen_patterns))
    .count() as u16
}

// struct StartNode {
//   children: Vec<ThreadNode>
// }

// struct ThreadNode {
//   colour: char,
//   terminateable: bool,
//   children: Vec<ThreadNode>
// }

// fn create_thread_tree(threads: &str) -> StartNode {
//   let mut start_node = StartNode{ children: vec![] };
//   threads
//     .split(", ")
//     .for_each(|thread| {
//       let curr_node = start_node;
//       thread
//         .chars()
//         .for_each(|colour| {
//           if curr_node.children.contains(&colour) {
//             curr_node = curr_node.children[]
//           }
//         });
//     });
//   StartNode{ children: start_children }
// }

fn can_make_towel(pattern: &str, threads: &HashMap<char, Vec<&str>>, seen_patterns: &mut HashMap<String, bool>) -> bool {
  if let Some(ans) = seen_patterns.get(&String::from(pattern)) {
    *ans
  } else {
    let first_char = pattern.chars().next().unwrap();
    if let Some(char_threads) = threads.get(&first_char) {
      let can_make_pattern = char_threads.iter().any(|thread| {
        if *thread == pattern {
          true
        } else if thread.len() > pattern.len() {
          false
        } else {
          let (first, last) = pattern.split_at(thread.len());
          first == *thread && can_make_towel(last, threads, seen_patterns)
        }
      });
      seen_patterns.insert(String::from(pattern), can_make_pattern);
      can_make_pattern
    } else {
      seen_patterns.insert(String::from(pattern), false);
      false
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 19;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_19_a() {
    const EXAMPLE_ANSWER: Option<u16> = Some(6);
    const ANSWER: Option<u16> = Some(360);
    match utils::run_method::<u16>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_19_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
