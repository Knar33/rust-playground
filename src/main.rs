use std::rc::Rc;
use std::cell::RefCell;

fn main() { 
    println!("{:?}", longest_common_prefix(vec!["reflower".to_string(),"flow".to_string(),"flight".to_string()]));
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
  let mut letters: Vec<char> = Vec::new();
  let mut val: Option<usize> = None;
  'outer: for str in strs {
    let mut new_letters: Vec<char> = Vec::new();
    for (i, ref char) in  str.chars().enumerate() {
      if let Some(letter) = letters.get(i) {
        if letter != char {
          letters = new_letters.clone();
          break
        }
      }
      new_letters.push(*char);
    }
    if let Some(value) = val {
      if (new_letters.len() < value) {
        val = Some(new_letters.len());
        letters = new_letters.clone();
      }
    }
    else {
      val = Some(new_letters.len());
      letters = new_letters.clone();
    }
  }
  letters.into_iter().collect()
}