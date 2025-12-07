use std::fs;

type Result<T> = core::result::Result<T, Box<dyn core::error::Error>>;

fn zero_count(input: &str) -> i32 {
  let lines = input
    .lines()
    .map(|v| {
      let (l, r) = v.trim().split_at(1);
      let r = r.parse::<i32>().unwrap();

      if l == "L" { -r } else { r }
    })
    .collect::<Vec<_>>();

  let mut z_count = 0i32;
  lines.into_iter().fold(50, |dial, clicks| {
    let rs = (dial + clicks) % 100;

    let rs = rs + if rs.is_negative() { 100 } else { 0 };

    z_count += if rs == 0 { 1 } else { 0 };

    rs
  });

  z_count
}

fn all_zero_count(input: &str) -> i32 {
  let lines = input
    .lines()
    .map(|v| {
      let (l, r) = v.trim().split_at(1);
      let r = r.parse::<i32>().unwrap();

      if l == "L" { -r } else { r }
    })
    .collect::<Vec<_>>();

  let mut z_count = 0i32;

  lines.into_iter().fold(50, |dial, rotations| {
    let rotation_rest = rotations % 100;

    z_count += rotations.abs() / 100;

    let rs = dial + rotation_rest;

    if rotations.is_negative() {
      z_count += if dial != 0 && rs <= 0 { 1 } else { 0 };
    } else {
      z_count += if rs >= 100 { 1 } else { 0 }
    }

    let mut next_dial = (dial + rotations) % 100;

    if next_dial < 0 {
      next_dial += 100; // Back to range 0 -> 99
    }

    next_dial
  });

  z_count
}

fn main() -> Result<()> {
  let input = fs::read_to_string("inputs/day01.txt")?;

  println!("Day 01 test 01: {}", zero_count(&input));

  println!("Day 01 test 02: {}", all_zero_count(&input));

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  const FX_INPUT: &str = "\
      L68
      L30
      R48
      L5
      R60
      L55
      L1
      L99
      R14
      L82";

  #[test]
  fn test_part2() {
    let fx_expect = 6;

    assert_eq!(all_zero_count(FX_INPUT), fx_expect)
  }

  #[test]
  fn test_part1() {
    let fx_expect = 3;

    assert_eq!(zero_count(FX_INPUT), fx_expect)
  }
}
