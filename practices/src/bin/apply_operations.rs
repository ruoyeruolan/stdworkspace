// @Introduce  :
// @File       : apply_operations.rs
// @Author     : ryrl
// @Email      : ryrl970311@gmail.com
// @Time       : 2025/03/01 18:23
// @Description:

use core::num;

use practices::Solution;

pub trait ApplyOperations {
    fn apply_operations(nums: Vec<i32>) -> Vec<i32>;
}

impl ApplyOperations for Solution {
    fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let res = Vec::with_capacity(n);

        for idx in 0..n {
            if nums[idx] == nums[idx + 1] {
                // let nums[idx] = 2 * nums[idx];
            }
        }
        res
    }
}
