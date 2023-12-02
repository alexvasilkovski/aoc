use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;

fn main() -> io::Result<()> {
  let file = File::open("input.txt")?;
  let reader = BufReader::new(file);

  let mut total1: i32 = 0;
  let mut total2: i32 = 0;

  for line in reader.lines() {
    let line_str: &str = &line.unwrap_or_default();

    let result1: i32  = Solution::p1(line_str);
    total1 += result1;

    let result2: i32 = Solution::p2(line_str);
    total2 += result2;
  }

  println!("Part A: {}", total1);
  println!("Part B: {}", total2);
  
  Ok(())
}

struct Solution;

impl Solution {
  pub fn p1(x: &str) -> i32 {

    let r = Regex::new(r"\d+").unwrap();

    let n: String = r.find_iter(x).map(|m| m.as_str()).collect();
    let f: i32 = n.chars().next().unwrap_or('0').to_digit(10).unwrap_or(0) as i32;
    let l: i32 = n.chars().last().unwrap_or('0').to_digit(10).unwrap_or(0) as i32;

    f * 10 + l
  }
  pub fn p2(x: &str) -> i32 {

    let r1 = Regex::new(r#"\d|one|two|three|four|five|six|seven|eight|nine"#).unwrap();
    let r2 = Regex::new(r#"\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin"#).unwrap();

    let y: &str = &x.chars().rev().collect::<String>();

    let f: &str;
    let l: &str;

    if let Some(caps) = r1.captures(x) {
      f = caps.get(0).unwrap().as_str();
    } else {
      println!("no match f");
      f = "0";
    };

    if let Some(caps) = r2.captures(y) {
      l = caps.get(0).unwrap().as_str();
    } else {println!("no match f");
      println!("no match l");
      l = "0";
    };

    Self::parse(f) * 10 + Self::parserev(l)
  }
    
  pub fn parse(x: &str) -> i32 {
    let x: i32 = match x {
      "one" => 1,
      "two" => 2,
      "three" => 3,
      "four" => 4,
      "five" => 5,
      "six" => 6,
      "seven" => 7,
      "eight" => 8,
      "nine" => 9,
      _ => x.parse::<i32>().unwrap_or_default(),
    };
    x
  }

  pub fn parserev(y: &str) -> i32 {
    let y: i32 = match y {
      "eno" => 1,
      "owt" => 2,
      "eerht" => 3,
      "ruof" => 4,
      "evif" => 5,
      "xis" => 6,
      "neves" => 7,
      "thgie" => 8,
      "enin" => 9,
      _ => y.parse::<i32>().unwrap_or_default(),
    };
    y
  }
}

