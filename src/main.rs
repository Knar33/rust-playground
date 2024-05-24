fn main() { 
}

pub fn my_atoi(s: String) -> i32 {
  let numerals = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

  let mut return_number = 0_i64;
  let mut return_string = String::new();

  let mut whitespace = false;
  let mut sign = false;
  let mut leading_zeroes = false;

  for (i, si) in s.chars().enumerate() {
    if !whitespace {
      if si == ' ' {
        continue
      } else {
        whitespace = true;
      }
    }
    if !sign {
      if si == '-' {
        return_string.push(si);
        sign = true;
        continue
      }
      if si == '+' {
        sign = true;
        continue
      }
      sign = true;
    }

    if numerals.contains(&si) {
      if !leading_zeroes {
        if si != '0' {
          leading_zeroes = true;
          return_string.push(si);
        }
      } else {
        return_string.push(si);
      }
    } else {
      break
    }
  }

  if return_string.len() == 0 || return_string == "_".to_string() || return_string == "+".to_string() || return_string == "-".to_string() {
    return_string.push('0');
  }

  if return_string.len() > 11 {
    if return_string.chars().next().unwrap() == '-' {
      return std::i32::MIN;
    } else {
      return std::i32::MAX;
    }
  } else {
    return_number = return_string.parse::<i64>().unwrap();
    if return_number > std::i32::MAX as i64 {
      return_number = std::i32::MAX as i64;
    }
    if return_number < std::i32::MIN as i64 {
      return_number = std::i32::MIN as i64;
    }
  }
  return_number as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asserts() {
      assert_eq!(my_atoi("-+12".to_string()), 0);
      assert_eq!(my_atoi("42".to_string()), 42);
      assert_eq!(my_atoi("   -042".to_string()), -42);
    }
}