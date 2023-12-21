use std::collections::HashMap;

fn main() { 
}

pub fn max_area(height: Vec<i32>) -> i32 {
  let mut max_area = 0;
  let mut i: i32 = 0;
  let mut j: i32 = height.len() as i32 - 1;

  while i < j {
    let w = j - i;
    let h = height[j as usize].min(height[i as usize]);
    let area = w * h;
    max_area = max_area.max(area);

    if height[i as usize] > height[j as usize] {
      j -= 1;
    } else {
      i += 1;
    }
  }
  max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asserts() {

    }
}