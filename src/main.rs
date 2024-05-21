fn main() { 
  println!("{:?}", remove_element(&mut vec![0,1,2,2,3,0,4,2], 2))
}

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|&x| x != val);
    nums.len() as i32
}