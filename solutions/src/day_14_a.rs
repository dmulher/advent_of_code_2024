extern crate test;

pub fn main(contents: String) -> u32 {
  predict_robots(contents)
}

fn predict_robots(contents: String) -> u32 {
  let testing = contents.starts_with("p=0,4");
  let height: u8 = if testing {7} else {103};
  let width: u8 = if testing {11} else {101};
  contents
    .lines()
    .map(construct_robot)
    .map(|robot| predict_robot(height, width, 100, robot))
    .fold([0u32; 4], |acc, quad| {
      match quad {
        Quadrant::NW => [acc[0] + 1, acc[1], acc[2], acc[3]],
        Quadrant::NE => [acc[0], acc[1] + 1, acc[2], acc[3]],
        Quadrant::SE => [acc[0], acc[1], acc[2] + 1, acc[3]],
        Quadrant::SW => [acc[0], acc[1], acc[2], acc[3] + 1],
        Quadrant::NONE => acc,
      }
    })
    .into_iter()
    .reduce(|acc, a| acc * a)
    .unwrap()
}

#[derive(Debug)]
enum Quadrant {
  NW,
  NE,
  SE,
  SW,
  NONE
}

#[derive(Debug)]
struct Robot {
  start_x: u16,
  start_y: u16,
  vel_x: i16,
  vel_y: i16,
}

fn predict_robot(height: u8, width: u8, seconds: u8, robot: Robot) -> Quadrant {
  let x = (robot.start_x as i16 + robot.vel_x * seconds as i16) % width as i16;
  let y = (robot.start_y as i16 + robot.vel_y * seconds as i16) % height as i16;
  let final_x = if x < 0 {width - x.abs() as u8} else {x as u8};
  let final_y = if y < 0 {height - y.abs() as u8} else {y as u8};
  determine_quadrant(height, width, final_x, final_y)
}

fn construct_robot(robot: &str) -> Robot {
  let (pos, vel) = robot.strip_prefix("p=").unwrap().split_once(" v=").unwrap();
  let (pos_x, pos_y) = pos.split_once(",").unwrap();
  let (vel_x, vel_y) = vel.split_once(",").unwrap();
  Robot {
    start_x: pos_x.parse::<u16>().unwrap(),
    start_y: pos_y.parse::<u16>().unwrap(),
    vel_x: vel_x.parse::<i16>().unwrap(),
    vel_y: vel_y.parse::<i16>().unwrap(),
  }
}

fn determine_quadrant(height: u8, width: u8, x: u8, y: u8) -> Quadrant {
  let x_axis = height / 2;
  let y_axis = width / 2;
  if x < y_axis && y < x_axis {
    Quadrant::NW
  } else if x > y_axis && y < x_axis {
    Quadrant::NE
  } else if x > y_axis && y > x_axis {
    Quadrant::SE
  } else if x < y_axis && y > x_axis {
    Quadrant::SW
  } else {
    Quadrant::NONE
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 14;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_14_a() {
    const EXAMPLE_ANSWER: Option<u32> = Some(12);
    const ANSWER: Option<u32> = Some(208437768);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_14_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
