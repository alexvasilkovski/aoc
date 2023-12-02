use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;

fn main() -> io::Result<()> {
  let file = File::open("input.txt")?;
  let reader = BufReader::new(file);

  // initialize variables
  let mut total1: i32 = 0;
  let mut total2: i32 = 0;

  for line in reader.lines() {
    let line_str: &str = &line.unwrap_or_default();
    // run p1 for each line of input
    let result1: i32  = Solution::p1(line_str);
    // sum results
    total1 += result1;
    // run p2 for each line of input
    let result2: i32 = Solution::p2(line_str);
    // sum p2
    total2 += result2;
  }

  // print answers
  println!("Part A: {}", total1);
  println!("Part B: {}", total2);
  
  Ok(())
}

struct Solution;

impl Solution {
  pub fn p1(x: &str) -> i32 {
    // regex to return numbers from string
    let r = Regex::new(r"\d+").unwrap();
    // return numbers from string
    let n: String = r.find_iter(x).map(|m| m.as_str()).collect();
    // read first number
    let f: i32 = n.chars().next().unwrap_or('0').to_digit(10).unwrap_or(0) as i32;
    // read last number
    let l: i32 = n.chars().last().unwrap_or('0').to_digit(10).unwrap_or(0) as i32;
    // concatenate and return first and last number
    f * 10 + l
  }
  pub fn p2(x: &str) -> i32 {
    // regex for first number or written number
    let r = Regex::new(r#"\d|one|two|three|four|five|six|seven|eight|nine"#).unwrap();
    // regex for first number or written number reversed
    let e = Regex::new(r#"\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin"#).unwrap();
    // reverse input line
    let y: &str = &x.chars().rev().collect::<String>();
    // initialize variables
    let f: &str;
    let l: &str;
    // run regex to retrieve first number or written number
    if let Some(caps) = r.captures(x) {
      f = caps.get(0).unwrap().as_str();
    } else {
      println!("no match f");
      f = "0";
    };
    // run regex to retrieve last number or written number
    if let Some(caps) = e.captures(y) {
      l = caps.get(0).unwrap().as_str();
    } else {println!("no match f");
      println!("no match l");
      l = "0";
    };
    // concatenate and return first and last number or written number
    return Self::parse(f) * 10 + Self::parserev(l);
  }
    
  pub fn parse(v: &str) -> i32 {
    let v: i32 = match v {
      "one" => 1,
      "two" => 2,
      "three" => 3,
      "four" => 4,
      "five" => 5,
      "six" => 6,
      "seven" => 7,
      "eight" => 8,
      "nine" => 9,
      _ => v.parse::<i32>().unwrap_or_default(),
    };
    v // return first number as i32
  }

  pub fn parserev(c: &str) -> i32 {
    let c: i32 = match c {
      "eno" => 1,
      "owt" => 2,
      "eerht" => 3,
      "ruof" => 4,
      "evif" => 5,
      "xis" => 6,
      "neves" => 7,
      "thgie" => 8,
      "enin" => 9,
      _ => c.parse::<i32>().unwrap_or_default(),
    };
    c // return last number as i32
  }
}

