fn main() { 
}

pub fn length_of_longest_substring(s: String) -> i32 {
  let s_len = s.len() as i32;
  if s_len < 2 {
    return s_len
  }

  let mut longest = 0;

  for (i, ch) in s.chars().enumerate() {
    let mut prev = ch;
    let mut length = 1;
    for j in i..s_len as usize {
      if j == i{
        continue
      }
      let curr = s.chars().nth(j).unwrap();
      if prev == curr {
        break
      }
      length += 1;
    }
    if length > longest {
      longest = length;
    }
  }

  longest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asserts() {
      assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    }
}