fn main() { 
  println!("{:?}", str_str("leetcode".to_string(), "leeto".to_string()))
}

pub fn str_str(haystack: String, needle: String) -> i32 {
  let needle_len = needle.len();
  let haystack_len = haystack.len();
  if needle_len > haystack_len {
    return -1
  }
  for i in 0..haystack_len - needle_len + 1 {
    let mut matched = true;
    for j in 0..needle_len {
      if haystack.chars().nth(i + j).unwrap() != needle.chars().nth(j).unwrap() {
        matched = false;
        break;
      }
    }

    if matched {
      return i as i32
    }
  }
  return -1
}