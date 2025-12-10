use std::{collections::HashSet, fs};

fn parse_input(input: &str) -> Vec<(i64, i64)> {
  input
    .split(',')
    .map(|range| {
      let (first, last) = range
        .trim()
        .split_once('-')
        .expect("Should be separated by -");

      (
        first.parse().expect("First range should be number."),
        last.parse().expect("Last range should be number."),
      )
    })
    .collect()
}

fn part01(input: &str) -> i64 {
  let ranges = parse_input(input);

  let sums: Vec<i64> = ranges
    .into_iter()
    .map(|(first, last)| {
      let mut result: Vec<i64> = vec![];
      for n in first..=last {
        let s = n.to_string();
        let len = s.len();
        if len % 2 == 0 {
          let mid = len / 2;
          let (left, right) = s.split_at(mid);
          if left == right {
            result.push(n);
          }
        }
      }
      result.into_iter().sum()
    })
    .collect();

  sums.into_iter().sum()
}

#[allow(dead_code)]
fn part02(input: &str) -> i64 {
  let ranges = parse_input(input);

  let sums: Vec<i64> = ranges
    .into_iter()
    .map(|(first, last)| {
      let mut result: Vec<i64> = vec![];
      for n in first..=last {
        let s = n.to_string();
        let len = s.len();
        if len > 1 {
          let chars: Vec<char> = s.chars().collect();
          for i in 1..len {
            let hs = chars.chunks(i).collect::<HashSet<_>>();

            if hs.len() == 1 {
              result.push(n);
              break;
            }
          }
        }
      }
      result.into_iter().sum()
    })
    .collect();

  sums.into_iter().sum()
}

fn part02_1(input: &str) -> i64 {
  let ranges = parse_input(input);

  let sums: Vec<i64> = ranges
    .into_iter()
    .map(|(first, last)| {
      let mut result: Vec<i64> = vec![];
      for n in first..=last {
        let s = n.to_string();
        let len = s.len();
        for i in 2..=len {
          if len % i == 0 {
            let split_point = len / i;
            let (left, _) = s.split_at(split_point);
            if left.repeat(i) == s {
              result.push(n);
              break;
            }
          }
        }
      }
      result.into_iter().sum()
    })
    .collect();

  sums.into_iter().sum()
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
  let input = fs::read_to_string("inputs/day02.txt")?;

  println!("Day02 test 01: {}", part01(&input));
  println!("Day02 test 02: {}", part02_1(&input));

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  type Result<T> = core::result::Result<T, Box<dyn core::error::Error>>;

  #[test]
  fn test_part02() -> Result<()> {
    // -- Setup & Fixtures
    let fx_input = fs::read_to_string("inputs/day02_test.txt")?;
    let fx_expected = 4174379265;

    // -- Exec
    let result = part02(&fx_input);

    // -- Check
    assert_eq!(result, fx_expected);

    Ok(())
  }
  #[test]
  fn test_part1() -> Result<()> {
    // -- Setup & Fixtures
    let fx_input = fs::read_to_string("inputs/day02_test.txt")?;
    let fx_expect = 1227775554;

    // -- Exec
    let result = part01(&fx_input);

    // -- Check
    assert_eq!(result, fx_expect);

    Ok(())
  }
}
