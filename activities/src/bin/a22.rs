// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
  if (b == 0) { return None; }
  Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
  use crate::*;

  #[test]
  fn check_clamp_lower_bound() {
    let lower = std::i32::MIN + 1;
    let upper = std::i32::MAX - 1;
    let min = lower - 1;
    let result = clamp(min, lower.to_owned(), upper.to_owned());
    assert_eq!(result, lower, "should result in lower bound");
  }

  #[test]
  fn check_clamp_upper_bound() {
    let lower = std::i32::MIN + 1;
    let upper = std::i32::MAX - 1;
    let max = upper + 1;
    let result = clamp(max, lower.to_owned(), upper.to_owned());
    assert_eq!(result, upper, "should result in upper bound");
  }

  #[test]
  fn check_clamp_between() {
    let lower = std::i32::MIN + 1;
    let upper = std::i32::MAX - 1;

    let mid_lower = lower + 1;
    let result_mid_lower = clamp(mid_lower.to_owned(), lower.to_owned(), upper.to_owned());
    assert_eq!(result_mid_lower, mid_lower, "should result in upper bound");

    let mid_upper = upper - 1;
    let result_mid_upper = clamp(mid_upper.to_owned(), lower.to_owned(), upper.to_owned());
    assert_eq!(result_mid_upper, mid_upper, "should result in upper bound");
  }

  #[test]
  fn check_div_by_1_min() {
    let min = std::i32::MIN;
    let result = div(min.to_owned(), 1.to_owned()).unwrap_or(0);
    assert_eq!(result, min, "should be equal to original");
  }

  #[test]
  fn check_div_by_self_min() {
    let min = std::i32::MIN;
    let result = div(min.to_owned(), min.to_owned()).unwrap_or(0);
    assert_eq!(result, 1, "should be equal to 1");
  }

  #[test]
  fn check_div_by_zero_min() {
    let min = std::i32::MIN;
    let result = div(min.to_owned(), 0);
    assert_eq!(result, None, "should be equal to 1");
  }

  #[test]
  fn check_concat_empty_strings() {
    let first = "";
    let second = "";
    let expected = "".to_string();
    let result = concat(first, second);
    assert_eq!(result, expected, "should concatenate strings one after another immediately");
  }

  #[test]
  fn check_concat_one_empty_one_non_empty() {
    let first = " ";
    let second = "";
    let expected = " ".to_string();
    let result = concat(first, second);
    assert_eq!(result, expected, "should concatenate strings one after another immediately");
  }
}
