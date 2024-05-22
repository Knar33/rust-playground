fn main() { 
}

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    for i in 0..nums.len() {
      if nums[i] == target {
        return i as i32
      }
      if nums[i] > target {
        return i as i32 
      }
    }
    return nums.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asserts() {
        assert_eq!(search_insert(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 4), 4);
        assert_eq!(search_insert(vec![0, 1, 2, 3, 5, 6, 7, 8, 9, 10], 4), 4);
    }
}