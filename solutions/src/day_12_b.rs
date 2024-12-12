use std::ops::{Add, AddAssign};

extern crate test;

pub fn main(contents: String) -> u64 {
  fund_fencing(contents)
}

fn fund_fencing(contents: String) -> u64 {
  let garden = contents
    .lines()
    .map(|line| line
      .as_bytes()
    )
    .collect::<Vec<&[u8]>>();
  let mut visited = vec![vec![false; garden[0].len()]; garden.len()];

  let mut total_cost = 0u64;
  for i in 0..garden.len() {
    for j in 0..garden[i].len() {
      if visited[i][j] {
        continue;
      }
      total_cost += assess_plot_cost(i, j, &garden, &mut visited).get_total_cost() as u64;
    }
  }

  total_cost
}

struct PlotCost {
  area: u16,
  corners: u16
}

impl Add for PlotCost {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    PlotCost{area: self.area + rhs.area, corners: self.corners + rhs.corners}
  }
}

impl AddAssign for PlotCost {
  fn add_assign(&mut self, rhs: Self) {
    self.area += rhs.area;
    self.corners += rhs.corners;
  }
}

impl PlotCost {
  fn get_total_cost(self) -> u16 {
    self.area * self.corners
  }
}

fn assess_plot_cost(i: usize, j: usize, garden: &Vec<&[u8]>, visited: &mut Vec<Vec<bool>>) -> PlotCost {
  visited[i][j] = true;
  let mut plot_cost = PlotCost{area: 1, corners: count_corners(i, j, garden)};
  let current_plot_type = garden[i][j];
  if i > 0 && garden[i-1][j] == current_plot_type {
    if !visited[i-1][j] {
      plot_cost += assess_plot_cost(i-1, j, garden, visited);
    }
  }
  if i < garden.len() - 1 && garden[i+1][j] == current_plot_type {
    if !visited[i+1][j] {
      plot_cost += assess_plot_cost(i+1, j, garden, visited);
    }
  }
  if j > 0 && garden[i][j-1] == current_plot_type {
    if !visited[i][j-1] {
      plot_cost += assess_plot_cost(i, j-1, garden, visited);
    }
  }
  if j < garden[i].len() - 1 && garden[i][j+1] == current_plot_type {
    if !visited[i][j+1] {
      plot_cost += assess_plot_cost(i, j+1, garden, visited);
    }
  }
  plot_cost
}

fn count_corners(i: usize, j: usize, garden: &Vec<&[u8]>) -> u16 {
  let current_plot_type = garden[i][j];
  let is_upper_fence = i == 0 || garden[i-1][j] != current_plot_type;
  let is_lower_fence = i == garden.len() - 1 || garden[i+1][j] != current_plot_type;
  let is_left_fence = j == 0 || garden[i][j-1] != current_plot_type;
  let is_right_fence = j == garden[i].len() - 1 || garden[i][j+1] != current_plot_type;
  let outer_fences = (is_upper_fence && is_left_fence) as u16
    + (is_left_fence && is_lower_fence) as u16
    + (is_lower_fence && is_right_fence) as u16
    + (is_right_fence && is_upper_fence) as u16;
  let inner_corners = (!is_upper_fence && !is_left_fence && garden[i-1][j-1] != current_plot_type) as u16
    + (!is_left_fence && !is_lower_fence && garden[i+1][j-1] != current_plot_type) as u16
    + (!is_lower_fence && !is_right_fence && garden[i+1][j+1] != current_plot_type) as u16
    + (!is_right_fence && !is_upper_fence && garden[i-1][j+1] != current_plot_type) as u16;
  outer_fences + inner_corners
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 12;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_12_b() {
    const EXAMPLE_ANSWER: Option<u64> = Some(1206);
    const ANSWER: Option<u64> = Some(978590);
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_12_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
