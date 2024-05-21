fn main() { 
    println!("{:?}", is_valid("([)]".to_string()));
}

pub fn is_valid(s: String) -> bool {
  let mut openers: Vec<char> = Vec::new();

  for char in s.chars() {
    match char {
      matched @ (']' | ')' | '}') => {
        if let Some(popped) = openers.pop() {
          if matched != invert(popped) {
            return false
          }
        }
      },
      c => openers.push(c)
    }
  }
    
  return openers.len() == 0
}

fn invert(c: char) -> char {
  match c {
    '{' => '}',
    '[' => ']',
    '(' => ')',
    _ => c
  }
}