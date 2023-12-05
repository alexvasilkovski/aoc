use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
  let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let mut engine = Vec::new();

    for line in reader.lines() {
      let line_str: &str = &line.unwrap_or_default();
      engine.push(line_str.chars().collect());
    }

    let result1 = Solution::p1(&engine);
    println!("Part A: {}", result1);

    Ok(())
  }

struct Solution;

impl Solution {
  pub fn p1(engine: &[Vec<char>]) -> i32 {
    let mut total = 0;

    for i in 0..engine.len() {
      for j in 0..engine[i].len() {
        if engine[i][j].is_digit(10) {
          let neighbors = [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (-1, 1),
            (1, -1),
            (-1, -1),
          ];
          let is_adjacent = |&(di, dj)| {
            let (ni, nj) = (i as i32 + di, j as i32 + dj);
            if ni >= 0
              && ni < engine.len() as i32
              && nj >= 0
              && nj < engine[i].len() as i32
              && engine[ni as usize][nj as usize] != '.'
            {
              Some(engine[ni as usize][nj as usize].to_digit(10).unwrap_or(0) as i32)
            } else {
              None
            }
          };
          total += neighbors.iter().filter_map(is_adjacent).next().unwrap_or(0);
        }
      }
    }

    total
    
  }
    
  //pub fn p2(x: &str) -> i32 {
  //}
}
