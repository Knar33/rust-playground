fn main() {
    println!("{:?}", two_sum(vec![1,2,3,4,3,5] as Vec<i32>, 7i32));
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() - 1 {
        let num1 = nums[i];
        for j in 0..nums.len() {
            if i == j {
                continue;
            }
            let num2 = nums[j];
            if num1 + num2 == target {
                return vec![i as i32, j as i32]
            }
        }
    }
    unreachable!()
}