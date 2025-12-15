use std::fs;

fn parse_input(input: &str) -> Vec<Vec<u32>> {
  input
    .lines()
    .map(|line| {
      line
        .trim()
        .chars()
        .map(|n| n.to_digit(10).expect("Char should be a number"))
        .collect::<Vec<_>>()
    })
    .collect()
}

fn part01(input: &str) -> u32 {
  let banks = parse_input(input);
  let mut result: u32 = 0;
  for bank in banks {
    let first_batteries = bank
      .iter()
      .take(bank.len() - 1)
      .max()
      .expect("Should be get max number");

    let (i, _) = bank
      .iter()
      .enumerate()
      .find(|n| n.1 == first_batteries)
      .expect("Should be find a number");

    let b = bank.iter().skip(i + 1).collect::<Vec<_>>();
    let last_batteries = b.into_iter().max().unwrap();

    result += first_batteries * 10 + last_batteries;
  }

  result
}

fn part02(input: &str) -> u64 {
  let banks = parse_input(input);
  let mut result: u64 = 0;
  for mut bank in banks {
    let mut jolts = 0u64;
    for j in (0..=11).rev() {
      let digit = bank
        .iter()
        .take(bank.len() - j)
        .max()
        .expect("Should be get max number");

      let (i, _) = bank
        .iter()
        .enumerate()
        .find(|n| n.1 == digit)
        .expect("Should be find a number");

      jolts = (jolts * 10) + *digit as u64;
      bank = bank.into_iter().skip(i + 1).collect::<Vec<_>>();
    }
    result += jolts;
  }

  result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let input = fs::read_to_string("inputs/day03.txt")?;

  println!("Day03 test 01: {}", part01(&input));
  println!("Day03 test 02: {}", part02(&input));

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  type Result<T> = core::result::Result<T, Box<dyn core::error::Error>>;

  #[test]
  fn test_part02() -> Result<()> {
    // -- Setup & Fixtures
    let fx_input = fs::read_to_string("inputs/day03_test.txt")?;
    let fx_expected = 3121910778619;

    // -- Exec
    let result = part02(&fx_input);

    // -- Check
    assert_eq!(result, fx_expected);

    Ok(())
  }
  #[test]
  fn test_part1() -> Result<()> {
    // -- Setup & Fixtures
    let fx_input = fs::read_to_string("inputs/day03_test.txt")?;
    let fx_expect = 357;

    // -- Exec
    let result = part01(&fx_input);

    // -- Check
    assert_eq!(result, fx_expect);

    Ok(())
  }
}
