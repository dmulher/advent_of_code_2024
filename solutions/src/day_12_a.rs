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
      let plot_cost = assess_plot_cost(i, j, &garden, &mut visited);
      total_cost += plot_cost.area as u64 * plot_cost.perimeter as u64;
    }
  }

  total_cost
}

struct PlotCost {
  area: u16,
  perimeter: u16
}

fn assess_plot_cost(i: usize, j: usize, garden: &Vec<&[u8]>, visited: &mut Vec<Vec<bool>>) -> PlotCost {
  visited[i][j] = true;
  let mut area = 1;
  let mut fencing = 0;
  let current_plot_type = garden[i][j];
  if i > 0 && garden[i-1][j] == current_plot_type {
    if !visited[i-1][j] {
      let up_cost = assess_plot_cost(i-1, j, garden, visited);
      area += up_cost.area;
      fencing += up_cost.perimeter;
    }
  } else {
    fencing += 1;
  }
  if i < garden.len() - 1 && garden[i+1][j] == current_plot_type {
    if !visited[i+1][j] {
      let down_cost = assess_plot_cost(i+1, j, garden, visited);
      area += down_cost.area;
      fencing += down_cost.perimeter;
    }
  } else {
    fencing += 1;
  }
  if j > 0 && garden[i][j-1] == current_plot_type {
    if !visited[i][j-1] {
      let left_cost = assess_plot_cost(i, j-1, garden, visited);
      area += left_cost.area;
      fencing += left_cost.perimeter;
    }
  } else {
    fencing += 1;
  }
  if j < garden[i].len() - 1 && garden[i][j+1] == current_plot_type {
    if !visited[i][j+1] {
      let right_cost = assess_plot_cost(i, j+1, garden, visited);
      area += right_cost.area;
      fencing += right_cost.perimeter;
    }
  } else {
    fencing += 1;
  }
  PlotCost{area, perimeter: fencing}
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 12;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_12_a() {
    const EXAMPLE_ANSWER: Option<u64> = Some(1930);
    const ANSWER: Option<u64> = Some(1546338);
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_12_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
