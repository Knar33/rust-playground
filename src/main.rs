fn main() { 
}

pub fn reverse(x: i32) -> i32 {
  let mut number = x.to_string();
  let mut reverse_number = String::new();
  let negative: bool = number.chars().next().unwrap() == '-';

  if negative {
    number.remove(0);
    reverse_number.push('-')
  }

  let length = number.len();
  let mut i = 0; while i < length {
    reverse_number.push(number.chars().nth(length - i - 1).unwrap());
    i += 1;
  }

  let mut return_number = 0;
  if let Ok(num) = reverse_number.parse::<i32>() {
    return_number = num;
  }
  return_number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asserts() {
      assert_eq!(reverse(1234), 4321);
      assert_eq!(reverse(-1234), -4321);
      assert_eq!(reverse(std::i32::MAX), 0);
    }
}