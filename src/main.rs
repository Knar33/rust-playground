fn main() {
    println!("{}", find_length_of_lcis(vec![1,2,3,4,3,5] as Vec<i32>));
}

pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
    let mut longest: i32 = 1;
    for i in 0..nums.len() - 1 {
        let mut length: i32 = 1;
        for j in i + 1..nums.len() {  
            if nums[j] > nums[j-1] {
                length += 1;
            } else {
                break;
            }
        }
        if length > longest {
            longest = length;
        }
    }
    longest
}