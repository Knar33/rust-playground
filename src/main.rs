fn main() { 
  println!("{:?}", remove_duplicates(&mut vec![0,0,1,1,1,2,2,3,3,4]))
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
let num_len = nums.len();
  if nums.len() == 0 {
    return 0
  }

  let mut j = 0;
  for i in 0..num_len {
    if nums[j] != nums[i] {
      j += 1;
      nums[j] = nums[i];
      
    }
  }
  j as i32 + 1
}
