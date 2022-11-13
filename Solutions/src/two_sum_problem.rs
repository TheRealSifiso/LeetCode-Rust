pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, &a) in nums.iter().enumerate(){
            for (j, &b) in nums.iter().enumerate(){

                if i != j && (nums[i] + nums [j] == target) {
                    return vec![i as i32, j as i32]
                }
            }
        }
        vec![]
    }
}