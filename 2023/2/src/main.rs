use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
  let file = File::open("input.txt")?;
  let reader = BufReader::new(file);

  for line in reader.lines() {
    let line_str: &str = &line.unwrap_or_default();

    let mut total1: i32 = 0;
    let _total2: i32 = 0;
    
    let result1: i32  = Solution::p1(line_str);
    total1 += result1;

    println!("Part A: {}", total1);

  }
  
  Ok(())
}

struct Solution;

impl Solution {
  pub fn p1(x: &str) -> i32 {
    let mut d: Vec<Vec<i32>> = Vec::new();
    let ca = vec![vec![0, 0, 0]];
  
    if let Some(id) = x.split(':').next().and_then(|s| s.split_whitespace().last()) {
      if let Ok(parsed_id) = id.parse::<i32>() {
        d.push(vec![parsed_id]);
      }
    }

    if let Some(l) = x.splitn(2, ": ").nth(1) {
      for s in l.split(';') {
        let mut ca = vec![0, 0, 0]; // Reinitialize ca within the loop if needed
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
        d.push(ca);
      }
    
      //println!("{:?}", d);
      let counts: Vec<i32> = vec![12, 13, 14]; 

      let mut sums: Vec<i32> = Vec::new();

      for inner_array in ca.iter().skip(1) {
        let total: i32 = inner_array[0] + inner_array[1] + inner_array[2];
        println!("{:?}", sums);
        sums.push(total);
      }

      for i in 0..sums.len() {
        if sums[i] > counts[i] {
          println!("{:?}", sums);
          println!("{:?}", counts);
          return 3;
        }
      }
    }
    0
  }
}

