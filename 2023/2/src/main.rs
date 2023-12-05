use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
  let file = File::open("input.txt")?;
  let reader = BufReader::new(file);

  let mut total1: i32 = 0;
  let mut total2: i32 = 0;

  for line in reader.lines() {
    let line_str: &str = &line.unwrap_or_default();
    
    let result1: i32  = Solution::p1(line_str);
    total1 = total1 + result1;

    let result2: i32  = Solution::p2(line_str);
    total2 = total2 + result2;
  }

  println!("Part A: {}", total1);
  println!("Part B: {}", total2);
  
  Ok(())
}

struct Solution;

impl Solution {
  pub fn p1(x: &str) -> i32 {
    let mut d: Vec<Vec<i32>> = Vec::new();
    let mut ca = vec![0, 0, 0];
    let counts: Vec<i32> = vec![12, 13, 14]; 
    let mut high: Vec<i32> = vec![0, 0, 0];
    let mut identifier: i32 = 0;

    if let Some(id) = x.split(':').next().and_then(|s| s.split_whitespace().last()) {
      if let Ok(parsed_id) = id.parse::<i32>() {
        identifier = parsed_id;
        d.push(vec![parsed_id]);
      }
    }

    if let Some(l) = x.splitn(2, ": ").nth(1) {
      for s in l.split(';') {
        for e in s.split(',') {
          let p: Vec<&str> = e.trim().split_whitespace().collect();
          if let Some(q) = p.get(0).and_then(|&l| l.parse::<i32>().ok()) {
            if let Some(c) = p.get(1) {
              let mut index = 0;
              match *c {
                "red" => index = 0,
                "green" => index = 1,
                "blue" => index = 2,
                _ => (),
              }
              ca[index] = q;
            }
          }
        }
        d.push(ca.clone());
      }

      for inner in d.iter().skip(1) {
        for (i, &element) in inner.iter().enumerate() {
          if element > high[i] {
            high[i] = element;
          }
        }
      }
    }

    if Self::compare(high, counts) {
      return identifier;
    }
    0
  }

  pub fn compare(x: Vec<i32>, y: Vec<i32>) -> bool {
    for (i, &element) in x.iter().enumerate() {
      if element > y[i] {
        return false;
      }
    }
    true
  }
    
  pub fn p2(x: &str) -> i32 {
    let mut d: Vec<Vec<i32>> = Vec::new();
    let mut ca = vec![0, 0, 0];
    let mut high: Vec<i32> = vec![0, 0, 0];

    if let Some(id) = x.split(':').next().and_then(|s| s.split_whitespace().last()) {
      if let Ok(parsed_id) = id.parse::<i32>() {
        d.push(vec![parsed_id]);
      }
    }
    
    if let Some(l) = x.splitn(2, ": ").nth(1) {
      for s in l.split(';') {
        for e in s.split(',') {
          let p: Vec<&str> = e.trim().split_whitespace().collect();
          if let Some(q) = p.get(0).and_then(|&l| l.parse::<i32>().ok()) {
            if let Some(c) = p.get(1) {
              let mut index = 0;
              match *c {
                "red" => index = 0,
                "green" => index = 1,
                "blue" => index = 2,
                _ => (),
              }
              ca[index] = q;
            }
          }
        }
        d.push(ca.clone());
      }

      for inner in d.iter().skip(1) {
        for (i, &element) in inner.iter().enumerate() {
          if element > high[i] {
            high[i] = element;
          }
        }
      }
    }
    return high.iter().fold(1, |acc, &x| acc * x);
  }
}
