use std::{cmp::Reverse, collections::BinaryHeap};

extern crate test;

pub fn main(contents: String) -> u32 {
  do_thing(contents)
}

#[derive(PartialEq, Eq, Debug)]
enum MapTile {
  WALL,
  PATH,
  END,
  START,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
#[repr(u8)]
enum Direction {
  NORTH = 0,
  EAST = 1,
  SOUTH = 2,
  WEST = 3,
}

#[derive(PartialEq, Eq, Debug)]
struct DijkstraNode {
  min_cost: u32,
  pos: (usize, usize),
  facing: Direction,
}

impl PartialOrd for DijkstraNode {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for DijkstraNode {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.min_cost.cmp(&other.min_cost)
  }
}

fn do_thing(contents: String) -> u32 {
  let map = contents
    .lines()
    .map(|line| {
      line
        .chars()
        .map(|c| match c {
          '#' => MapTile::WALL,
          '.' => MapTile::PATH,
          'E' => MapTile::END,
          'S' => MapTile::START,
          _ => panic!("Something unexpected happened"),
        })
        .collect::<Vec<MapTile>>()
    })
    .collect::<Vec<Vec<MapTile>>>();

  let width = map.len();
  let height = map[0].len();

  let start_pos: (usize, usize) = (height - 2, 1);

  let mut dijkstra = BinaryHeap::<Reverse<DijkstraNode>>::new();  // Maybe
  dijkstra.push(Reverse(DijkstraNode{ min_cost: 0, pos: start_pos, facing: Direction::EAST }));
  let mut visited: Vec<Vec<[bool; 4]>> = vec![vec![[false; 4]; width]; height];

  while let Some(Reverse(next_node)) = dijkstra.pop() {
    // Are we done?
    if visited[next_node.pos.0][next_node.pos.1][next_node.facing as usize] {
      continue;
    }
    visited[next_node.pos.0][next_node.pos.1][next_node.facing as usize] = true;
    if map[next_node.pos.0][next_node.pos.1] == MapTile::END {
      return next_node.min_cost;
    }

    // Check forwards
    let forward_pos = go_forwards(&next_node.pos, &next_node.facing);
    if !visited[forward_pos.0][forward_pos.1][next_node.facing as usize] && map[forward_pos.0][forward_pos.1] != MapTile::WALL {
      dijkstra.push(Reverse(DijkstraNode { min_cost: next_node.min_cost + 1, pos: forward_pos, facing: next_node.facing }));
    }

    // Check left
    let left_dir = turn_left(&next_node.facing);
    let left_pos = go_forwards(&next_node.pos, &left_dir);
    if !visited[next_node.pos.0][next_node.pos.1][left_dir as usize] && map[left_pos.0][left_pos.1] != MapTile::WALL {
      dijkstra.push(Reverse(DijkstraNode { min_cost: next_node.min_cost + 1000, pos: next_node.pos, facing: left_dir }));
    }

    // Check right
    let right_dir = turn_right(&next_node.facing);
    let right_pos = go_forwards(&next_node.pos, &right_dir);
    if !visited[next_node.pos.0][next_node.pos.1][right_dir as usize] && map[right_pos.0][right_pos.1] != MapTile::WALL {
      dijkstra.push(Reverse(DijkstraNode { min_cost: next_node.min_cost + 1000, pos: next_node.pos, facing: right_dir }));
    }
  }

  0u32
}

fn go_forwards(pos: &(usize, usize), direction: &Direction) -> (usize, usize) {
  match direction {
    Direction::NORTH => (pos.0-1, pos.1),
    Direction::WEST => (pos.0, pos.1-1),
    Direction::SOUTH => (pos.0+1, pos.1),
    Direction::EAST => (pos.0, pos.1+1),
  }
}

fn turn_left(direction: &Direction) -> Direction {
  match direction {
    Direction::NORTH => Direction::WEST,
    Direction::WEST => Direction::SOUTH,
    Direction::SOUTH => Direction::EAST,
    Direction::EAST => Direction::NORTH,
  }
}

fn turn_right(direction: &Direction) -> Direction {
  match direction {
    Direction::NORTH => Direction::EAST,
    Direction::EAST => Direction::SOUTH,
    Direction::SOUTH => Direction::WEST,
    Direction::WEST => Direction::NORTH,
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 16;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_16_a() {
    const EXAMPLE_ANSWER: Option<u32> = Some(7036);
    const ANSWER: Option<u32> = Some(105508);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_16_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
