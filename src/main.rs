use std::collections::HashMap;

fn main() { 
}

pub fn length_of_longest_substring(s: String) -> i32 {
  let s_len = s.len() as i32;
  if s_len < 2 {
    return s_len
  }

  let mut longest = 0;
  let mut i = 0;
  let mut j = 0;

  //left pointer 
  while i < s_len {
    let mut chars: HashMap<u8, i32> = HashMap::with_capacity(512);
    let mut length = 1;
    //right pointer
    while j < s_len {
      if j == i {
        //second pointer is at first position
        chars.insert(s.as_bytes()[j as usize], j);
        j += 1;
        continue;
      } else {
        //second pointer is ahead of first and in a valid spot
        let j_char = s.as_bytes()[j as usize];
        if chars.contains_key(&j_char) {
          i = *chars.get(&j_char).unwrap() + 1;
          j = i;
          break;
        }
        chars.insert(j_char, j);
        length += 1;
        j += 1;
      }
    }
    if length > longest  {
      longest = length;
    }
    if j == s_len {
      break
    }
  }
  longest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asserts() {
      assert_eq!(length_of_longest_substring("dvdf".to_string()), 2);
    }
}