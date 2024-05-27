use std::collections::HashMap;

fn main() { 
}

pub fn longest_palindrome(s: String) -> String {
  let s_len: usize = s.len();
  let s_chars = s.as_bytes().as_ref();
  if s_len == 1 {
    return s
  }

  let mut char_positions: HashMap<u8, Vec<usize>> = HashMap::new();
  let mut longest_palindrome: String = String::new();

  let mut i = 0; while i < s_len {
    let s_char = s_chars[i];
    char_positions.entry(s_char).or_insert(vec![i]).push(i);
    i+=1;
  }

  for (pair_key, pair_value) in char_positions.iter() {
    let mut intermediate_palindrome = String::new();
    let pair_value_len = pair_value.len();

    if pair_value_len == 1 {
      intermediate_palindrome = pair_key.to_string();
    } else {
      let mut i = 0;
      while i < pair_value_len {
        let mut j = pair_value_len - 1;
        while j > i {
          if pair_value[j] - pair_value[i] + 1 < longest_palindrome.len() {
            break
          } 
          let substring = &s_chars[pair_value[i]..=pair_value[j]];
          if is_palindrome(substring) && substring.len() > intermediate_palindrome.len() {
            intermediate_palindrome = String::from_utf8(substring.to_vec()).unwrap();
            break
          }
          j -= 1;
        }
        i += 1;
      }
    }
    if intermediate_palindrome.len() > longest_palindrome.len() {
      longest_palindrome = intermediate_palindrome;
    }
  }
  longest_palindrome
}

pub fn is_palindrome(chars: &[u8]) -> bool {
  let midpoint = (chars.len() as f32 / 2f32).ceil();
  let mut i = 0;
  while i < chars.len() {
    if chars[i] != chars[chars.len() - i - 1] {
      return false
    }
    i+=1;
  }
  true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asserts() {
      assert_eq!(longest_palindrome("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabcaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string()), "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
      //assert_eq!(longest_palindrome("cbbd".to_string()), "bb".to_string());
    }
}