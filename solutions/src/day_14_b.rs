extern crate test;

pub fn main(contents: String) -> u64 {
  predict_robots(contents)
}

fn predict_robots(contents: String) -> u64 {
  let testing = contents.starts_with("p=0,4");
  let height: u8 = if testing {7} else {103};
  let width: u8 = if testing {11} else {101};
  let mut robots = contents
    .lines()
    .map(construct_robot)
    .collect::<Vec<Robot>>();

  let mut time_elapsed = 0u64;
  while is_tree(&robots) {
    time_elapsed += 1;
    robots = robots
      .clone()
      .iter()
      .map(|robot| update_robot(robot, width, height))
      .collect::<Vec<Robot>>();
  }
  time_elapsed
}

#[derive(Debug, Clone, Copy)]
struct Robot {
  curr_x: u8,
  curr_y: u8,
  vel_x: i16,
  vel_y: i16,
}

fn is_tree(robots: &Vec<Robot>) -> bool {
  let (x, y): (Vec<u8>, Vec<u8>) = robots
    .iter()
    .map(|robot| (robot.curr_x, robot.curr_y))
    .unzip();

  let x_score = std_deviation(&x);
  let y_score = std_deviation(&y);
  x_score < 25.0 && y_score < 25.0
}

fn std_deviation(vals: &Vec<u8>) -> f32 {
  let sum = vals.iter().map(|val| *val as u32).sum::<u32>() as f32;
  let count = vals.len() as f32;
  let mean = sum / count;
  let variance = vals.iter()
    .map(|&value| {
      let distance = mean - value as f32;
      distance * distance
    })
    .sum::<f32>() / count;
  variance.sqrt()
}

fn construct_robot(robot: &str) -> Robot {
  let (pos, vel) = robot.strip_prefix("p=").unwrap().split_once(" v=").unwrap();
  let (pos_x, pos_y) = pos.split_once(",").unwrap();
  let (vel_x, vel_y) = vel.split_once(",").unwrap();
  Robot {
    curr_x: pos_x.parse::<u8>().unwrap(),
    curr_y: pos_y.parse::<u8>().unwrap(),
    vel_x: vel_x.parse::<i16>().unwrap(),
    vel_y: vel_y.parse::<i16>().unwrap(),
  }
}

fn update_robot(robot: &Robot, width: u8, height: u8) -> Robot {
  let x = (robot.curr_x as i16 + robot.vel_x as i16) % width as i16;
  let y = (robot.curr_y as i16 + robot.vel_y as i16) % height as i16;
  let final_x = if x < 0 {width - x.abs() as u8} else {x as u8};
  let final_y = if y < 0 {height - y.abs() as u8} else {y as u8};
  Robot {
    curr_x: final_x,
    curr_y: final_y,
    vel_x: robot.vel_x,
    vel_y: robot.vel_y,
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 14;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_14_b() {
    const EXAMPLE_ANSWER: Option<u64> = Some(12);
    const ANSWER: Option<u64> = Some(40);
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_14_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
